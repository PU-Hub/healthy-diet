use crate::{
    api::model::ErrorResponse,
    model::{AppState, ENVKey},
    utils::{jwt::AuthUser, rag_worker::enqueue_rag_document},
};
use axum::{
    Json,
    body::Body,
    extract::{Path, Query, State},
    http::{
        HeaderValue, StatusCode,
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    },
    response::{IntoResponse, Response},
};
use axum_extra::extract::Multipart;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::{
    env,
    path::{Path as StdPath, PathBuf},
    sync::Arc,
};
use tracing::error;
use uuid::Uuid;

const RAG_STATUS_UPLOADED: &str = "uploaded";

const MAX_UPLOAD_BYTES: usize = 20 * 1024 * 1024;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RagDocumentItem {
    pub id: Uuid,
    pub filename: String,
    pub storage_path: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub chunk_count: Option<i32>,
    pub embedding_model: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub next_retry_at: Option<chrono::DateTime<chrono::Utc>>,
    pub processing_started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_error_at: Option<chrono::DateTime<chrono::Utc>>,
    pub uploaded_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub file_url: String,
    pub preview_url: String,
}

#[derive(Debug, sqlx::FromRow)]
struct RagDocumentRow {
    pub id: Uuid,
    pub filename: String,
    pub storage_path: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub chunk_count: Option<i32>,
    pub embedding_model: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub next_retry_at: Option<chrono::DateTime<chrono::Utc>>,
    pub processing_started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_error_at: Option<chrono::DateTime<chrono::Utc>>,
    pub uploaded_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RagDocumentPreviewResponse {
    pub document_id: Uuid,
    pub filename: String,
    pub mime_type: String,
    pub preview_kind: String,
    pub content: Option<String>,
    pub truncated: bool,
    pub file_url: String,
    pub preview_url: String,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RagDocumentsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

fn build_admin_file_url(id: Uuid) -> String {
    format!("/admin/rag/documents/{}/file", id)
}

fn build_admin_preview_url(id: Uuid) -> String {
    format!("/admin/rag/documents/{}/preview", id)
}

fn build_public_file_url(id: Uuid) -> String {
    format!("/rag/sources/{}/file", id)
}

fn build_public_preview_url(id: Uuid) -> String {
    format!("/rag/sources/{}/preview", id)
}

impl From<RagDocumentRow> for RagDocumentItem {
    fn from(row: RagDocumentRow) -> Self {
        Self {
            id: row.id,
            filename: row.filename,
            storage_path: row.storage_path,
            mime_type: row.mime_type,
            size_bytes: row.size_bytes,
            chunk_count: row.chunk_count,
            embedding_model: row.embedding_model,
            status: row.status,
            error_message: row.error_message,
            retry_count: row.retry_count,
            next_retry_at: row.next_retry_at,
            processing_started_at: row.processing_started_at,
            last_error_at: row.last_error_at,
            uploaded_by: row.uploaded_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
            file_url: build_admin_file_url(row.id),
            preview_url: build_admin_preview_url(row.id),
        }
    }
}

fn allowed_ext(ext: &str) -> bool {
    matches!(ext, "pdf" | "txt" | "md" | "docx")
}

fn allowed_mime(mime: &str) -> bool {
    matches!(
        mime,
        "application/pdf"
            | "text/plain"
            | "text/markdown"
            | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            | "application/octet-stream"
    )
}

fn build_rag_storage_path(ext: &str) -> String {
    let now = chrono::Utc::now();
    format!(
        "rag_docs/{:04}/{:02}/{}.{}",
        now.year(),
        now.month(),
        Uuid::new_v4(),
        ext
    )
}

fn resolve_rag_root() -> PathBuf {
    env::var(ENVKey::RAG_DOCS_ROOT)
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("uploads"))
}

async fn fetch_rag_document_row(
    db: &sqlx::PgPool,
    id: Uuid,
) -> Result<Option<RagDocumentRow>, sqlx::Error> {
    sqlx::query_as::<_, RagDocumentRow>(
        r#"
        SELECT id, filename, storage_path, mime_type, size_bytes, chunk_count, embedding_model,
               status, error_message, retry_count, next_retry_at, processing_started_at, last_error_at,
               uploaded_by, created_at, updated_at
        FROM rag_documents
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await
}

async fn read_document_bytes(document: &RagDocumentRow) -> Result<Vec<u8>, std::io::Error> {
    let full_path = resolve_rag_root().join(&document.storage_path);
    tokio::fs::read(full_path).await
}

fn file_not_found_response() -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "RAG document not found".to_string(),
        }),
    )
}

fn make_file_response(document: &RagDocumentRow, file_bytes: Vec<u8>) -> Response {
    let mut response = Response::new(Body::from(file_bytes));
    *response.status_mut() = StatusCode::OK;
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_str(&document.mime_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );
    let disposition = format!(
        "inline; filename=\"{}\"",
        document.filename.replace('"', "")
    );
    if let Ok(value) = HeaderValue::from_str(&disposition) {
        response.headers_mut().insert(CONTENT_DISPOSITION, value);
    }
    response
}

fn build_preview_response(
    document: &RagDocumentRow,
    content: Option<String>,
    preview_kind: &str,
    truncated: bool,
    message: Option<String>,
    is_public: bool,
) -> RagDocumentPreviewResponse {
    RagDocumentPreviewResponse {
        document_id: document.id,
        filename: document.filename.clone(),
        mime_type: document.mime_type.clone(),
        preview_kind: preview_kind.to_string(),
        content,
        truncated,
        file_url: if is_public {
            build_public_file_url(document.id)
        } else {
            build_admin_file_url(document.id)
        },
        preview_url: if is_public {
            build_public_preview_url(document.id)
        } else {
            build_admin_preview_url(document.id)
        },
        message,
    }
}

fn make_text_preview(
    document: &RagDocumentRow,
    file_bytes: Vec<u8>,
    is_public: bool,
) -> RagDocumentPreviewResponse {
    const MAX_PREVIEW_CHARS: usize = 20_000;

    let text = String::from_utf8_lossy(&file_bytes).to_string();
    let char_count = text.chars().count();
    let truncated = char_count > MAX_PREVIEW_CHARS;
    let content = if truncated {
        text.chars().take(MAX_PREVIEW_CHARS).collect()
    } else {
        text
    };

    build_preview_response(document, Some(content), "text", truncated, None, is_public)
}

fn make_binary_preview(document: &RagDocumentRow, is_public: bool) -> RagDocumentPreviewResponse {
    let message = match document.mime_type.as_str() {
        "application/pdf" => {
            "Preview text is not generated yet for PDF. Use fileUrl to open the original file."
        }
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
            "Preview text is not generated yet for DOCX. Use fileUrl to open the original file."
        }
        _ => {
            "Preview is not available for this document type. Use fileUrl to open the original file."
        }
    };

    build_preview_response(
        document,
        None,
        "file",
        false,
        Some(message.to_string()),
        is_public,
    )
}

pub async fn admin_rag_documents_handler(
    _admin_user: AuthUser,
    Query(params): Query<RagDocumentsQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RagDocumentItem>>, (StatusCode, Json<ErrorResponse>)> {
    let limit = params.limit.unwrap_or(50).clamp(1, 200);
    let offset = params.offset.unwrap_or(0).max(0);

    let documents = sqlx::query_as::<_, RagDocumentRow>(
        r#"
        SELECT id, filename, storage_path, mime_type, size_bytes, chunk_count, embedding_model,
               status, error_message, retry_count, next_retry_at, processing_started_at, last_error_at,
               uploaded_by, created_at, updated_at
        FROM rag_documents
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("DB Error (RAG Documents List): {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
            }),
        )
    })?;

    Ok(Json(documents.into_iter().map(Into::into).collect()))
}

pub async fn admin_rag_document_detail_handler(
    _admin_user: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RagDocumentItem>, (StatusCode, Json<ErrorResponse>)> {
    let document = fetch_rag_document_row(&state.db, id)
        .await
        .map_err(|e| {
            error!("DB Error (RAG Document Detail): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            )
        })?
        .ok_or_else(file_not_found_response)?;

    Ok(Json(document.into()))
}

pub async fn admin_rag_upload_handler(
    admin_user: AuthUser,
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<RagDocumentItem>, (StatusCode, Json<ErrorResponse>)> {
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut original_filename: Option<String> = None;
    let mut mime_type: Option<String> = None;
    let mut embedding_model: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid multipart payload: {}", e),
            }),
        )
    })? {
        let field_name = field.name().unwrap_or_default().to_string();
        if field_name == "file" {
            original_filename = field.file_name().map(|n| n.to_string());
            mime_type = field.content_type().map(|m| m.to_string());
            let bytes = field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: format!("Unable to read upload file: {}", e),
                    }),
                )
            })?;
            file_bytes = Some(bytes.to_vec());
        } else if field_name == "embeddingModel" {
            let text = field.text().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: format!("Invalid embeddingModel field: {}", e),
                    }),
                )
            })?;
            let normalized = text.trim().to_string();
            if !normalized.is_empty() {
                embedding_model = Some(normalized);
            }
        }
    }

    let file_bytes = file_bytes.ok_or((
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "Missing file field".to_string(),
        }),
    ))?;

    let filename = original_filename.ok_or((
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "Missing file name".to_string(),
        }),
    ))?;
    let ext = StdPath::new(&filename)
        .extension()
        .and_then(|v| v.to_str())
        .map(|v| v.to_ascii_lowercase())
        .ok_or((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Unsupported file extension".to_string(),
            }),
        ))?;
    if !allowed_ext(&ext) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Unsupported file extension. Allowed: pdf, txt, md, docx".to_string(),
            }),
        ));
    }

    let mime_type = mime_type.unwrap_or_else(|| "application/octet-stream".to_string());
    if !allowed_mime(&mime_type) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Unsupported MIME type".to_string(),
            }),
        ));
    }

    if file_bytes.len() > MAX_UPLOAD_BYTES {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(ErrorResponse {
                error: "File too large. Max size is 20MB".to_string(),
            }),
        ));
    }

    let storage_path = build_rag_storage_path(&ext);
    let rag_root = resolve_rag_root();
    let full_path = rag_root.join(&storage_path);
    if let Some(parent) = full_path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            error!("File system error (create dir): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to prepare storage".to_string(),
                }),
            )
        })?;
    }
    tokio::fs::write(&full_path, &file_bytes)
        .await
        .map_err(|e| {
            error!("File system error (write file): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to store uploaded file".to_string(),
                }),
            )
        })?;

    let document = sqlx::query_as::<_, RagDocumentRow>(
        r#"
        INSERT INTO rag_documents (
            filename, storage_path, mime_type, size_bytes, chunk_count, embedding_model, status,
            error_message, retry_count, next_retry_at, processing_started_at, last_error_at, uploaded_by, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, NULL, $5, $6, NULL, 0, NULL, NULL, NULL, $7, now(), now())
        RETURNING id, filename, storage_path, mime_type, size_bytes, chunk_count, embedding_model,
                  status, error_message, retry_count, next_retry_at, processing_started_at, last_error_at,
                  uploaded_by, created_at, updated_at
        "#,
    )
    .bind(&filename)
    .bind(&storage_path)
    .bind(&mime_type)
    .bind(file_bytes.len() as i64)
    .bind(&embedding_model)
    .bind(RAG_STATUS_UPLOADED)
    .bind(admin_user.user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        error!("DB Error (Insert RAG Document): {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to save document metadata".to_string(),
            }),
        )
    })?;

    Ok(Json(document.into()))
}

pub async fn admin_rag_reindex_handler(
    _admin_user: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RagDocumentItem>, (StatusCode, Json<ErrorResponse>)> {
    let exists =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM rag_documents WHERE id = $1)")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("DB Error (RAG Reindex exists): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            )
        })?;

    if !exists {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "RAG document not found".to_string(),
            }),
        ));
    }

    enqueue_rag_document(&state.db, id).await.map_err(|e| {
        error!("DB Error (RAG Reindex enqueue): {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
            }),
        )
    })?;

    let document = sqlx::query_as::<_, RagDocumentRow>(
        r#"
        SELECT id, filename, storage_path, mime_type, size_bytes, chunk_count, embedding_model,
               status, error_message, retry_count, next_retry_at, processing_started_at, last_error_at,
               uploaded_by, created_at, updated_at
        FROM rag_documents
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        error!("DB Error (RAG Reindex): {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
            }),
        )
    })?;

    Ok(Json(document.into()))
}

pub async fn admin_rag_delete_handler(
    _admin_user: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RagDocumentItem>, (StatusCode, Json<ErrorResponse>)> {
    let document = fetch_rag_document_row(&state.db, id)
        .await
        .map_err(|e| {
            error!("DB Error (Get RAG Document Before Delete): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            )
        })?
        .ok_or_else(file_not_found_response)?;

    let full_path = resolve_rag_root().join(&document.storage_path);
    if tokio::fs::try_exists(&full_path).await.unwrap_or(false) {
        tokio::fs::remove_file(&full_path).await.map_err(|e| {
            error!("File system error (delete file): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to remove document file".to_string(),
                }),
            )
        })?;
    }

    sqlx::query("DELETE FROM rag_documents WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("DB Error (Delete RAG Document): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to delete document metadata".to_string(),
                }),
            )
        })?;

    Ok(Json(document.into()))
}

pub async fn admin_rag_document_file_handler(
    _admin_user: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let document = fetch_rag_document_row(&state.db, id)
        .await
        .map_err(|e| {
            error!("DB Error (RAG Document File): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            )
        })?
        .ok_or_else(file_not_found_response)?;

    let file_bytes = read_document_bytes(&document).await.map_err(|e| {
        error!("File system error (read file): {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to read document file".to_string(),
            }),
        )
    })?;

    Ok(make_file_response(&document, file_bytes).into_response())
}

pub async fn public_rag_document_file_handler(
    _auth_user: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let document = fetch_rag_document_row(&state.db, id)
        .await
        .map_err(|e| {
            error!("DB Error (Public RAG Document File): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            )
        })?
        .ok_or_else(file_not_found_response)?;

    let file_bytes = read_document_bytes(&document).await.map_err(|e| {
        error!("File system error (read public file): {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to read document file".to_string(),
            }),
        )
    })?;

    Ok(make_file_response(&document, file_bytes).into_response())
}

pub async fn admin_rag_document_preview_handler(
    _admin_user: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RagDocumentPreviewResponse>, (StatusCode, Json<ErrorResponse>)> {
    let document = fetch_rag_document_row(&state.db, id)
        .await
        .map_err(|e| {
            error!("DB Error (RAG Document Preview): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            )
        })?
        .ok_or_else(file_not_found_response)?;

    let preview = match document.mime_type.as_str() {
        "text/plain" | "text/markdown" => {
            let file_bytes = read_document_bytes(&document).await.map_err(|e| {
                error!("File system error (preview read file): {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to read document file".to_string(),
                    }),
                )
            })?;
            make_text_preview(&document, file_bytes, false)
        }
        _ => make_binary_preview(&document, false),
    };

    Ok(Json(preview))
}

pub async fn public_rag_document_preview_handler(
    _auth_user: AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RagDocumentPreviewResponse>, (StatusCode, Json<ErrorResponse>)> {
    let document = fetch_rag_document_row(&state.db, id)
        .await
        .map_err(|e| {
            error!("DB Error (Public RAG Document Preview): {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            )
        })?
        .ok_or_else(file_not_found_response)?;

    let preview = match document.mime_type.as_str() {
        "text/plain" | "text/markdown" => {
            let file_bytes = read_document_bytes(&document).await.map_err(|e| {
                error!("File system error (public preview read file): {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to read document file".to_string(),
                    }),
                )
            })?;
            make_text_preview(&document, file_bytes, true)
        }
        _ => make_binary_preview(&document, true),
    };

    Ok(Json(preview))
}

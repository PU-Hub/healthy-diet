use crate::{
    api::{
        model::{ErrorResponse, ROLE_OPERATOR, ROLE_SUPER_ADMIN},
        rag_document::send_json_request,
    },
    utils::jwt::AuthUser,
};
use axum::{
    Json,
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
};
use reqwest::Method;
use serde::Deserialize;
use serde_json::Value;

fn normalized_admin_user(admin_user: &AuthUser) -> AuthUser {
    let forwarded_role = match admin_user.role.as_str() {
        ROLE_SUPER_ADMIN => "admin",
        ROLE_OPERATOR => "nutritionist",
        other => other,
    };

    AuthUser {
        user_id: admin_user.user_id,
        email: admin_user.email.clone(),
        role: forwarded_role.to_string(),
    }
}

#[derive(Debug, Deserialize)]
pub struct KnowledgeGraphNodesQuery {
    pub limit: Option<i64>,
    pub node_type: Option<String>,
    pub query: Option<String>,
}

pub async fn knowledge_graph_status_handler(
    headers: HeaderMap,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<ErrorResponse>)> {
    let (status, response) =
        send_json_request(&headers, None, Method::GET, "/api/graph/status", None, None).await?;

    Ok((status, Json(response)))
}

pub async fn knowledge_graph_nodes_handler(
    headers: HeaderMap,
    Query(params): Query<KnowledgeGraphNodesQuery>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<ErrorResponse>)> {
    let mut query_pairs = Vec::new();

    if let Some(limit) = params.limit {
        query_pairs.push(("limit", limit.clamp(1, 500).to_string()));
    }
    if let Some(node_type) = params.node_type.filter(|value| !value.trim().is_empty()) {
        query_pairs.push(("node_type", node_type));
    }
    if let Some(query) = params.query.filter(|value| !value.trim().is_empty()) {
        query_pairs.push(("query", query));
    }

    let query_pairs = (!query_pairs.is_empty()).then_some(query_pairs);

    let (status, response) = send_json_request(
        &headers,
        None,
        Method::GET,
        "/api/graph/nodes",
        query_pairs,
        None,
    )
    .await?;

    Ok((status, Json(response)))
}

pub async fn knowledge_graph_query_handler(
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<ErrorResponse>)> {
    let (status, response) = send_json_request(
        &headers,
        None,
        Method::POST,
        "/api/graph/search",
        None,
        Some(payload),
    )
    .await?;

    Ok((status, Json(response)))
}

pub async fn knowledge_graph_node_detail_handler(
    headers: HeaderMap,
    Path(node_id): Path<String>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<ErrorResponse>)> {
    let (status, response) = send_json_request(
        &headers,
        None,
        Method::GET,
        &format!("/api/graph/nodes/{node_id}"),
        None,
        None,
    )
    .await?;

    Ok((status, Json(response)))
}

pub async fn knowledge_graph_relation_evidence_handler(
    headers: HeaderMap,
    Path(relation_id): Path<String>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<ErrorResponse>)> {
    let (status, response) = send_json_request(
        &headers,
        None,
        Method::GET,
        &format!("/api/graph/relations/{relation_id}/evidence"),
        None,
        None,
    )
    .await?;

    Ok((status, Json(response)))
}

pub async fn admin_knowledge_graph_rebuild_handler(
    admin_user: AuthUser,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<ErrorResponse>)> {
    let admin_user = normalized_admin_user(&admin_user);
    let (status, response) = send_json_request(
        &headers,
        Some(&admin_user),
        Method::POST,
        "/api/graph/extract-all",
        None,
        Some(payload),
    )
    .await?;

    Ok((status, Json(response)))
}

pub async fn admin_knowledge_graph_extract_handler(
    admin_user: AuthUser,
    headers: HeaderMap,
    Path(document_id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<ErrorResponse>)> {
    let admin_user = normalized_admin_user(&admin_user);
    let (status, response) = send_json_request(
        &headers,
        Some(&admin_user),
        Method::POST,
        &format!("/api/graph/documents/{document_id}/extract"),
        None,
        Some(payload),
    )
    .await?;

    Ok((status, Json(response)))
}

pub async fn admin_knowledge_graph_document_detail_handler(
    admin_user: AuthUser,
    headers: HeaderMap,
    Path(document_id): Path<String>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<ErrorResponse>)> {
    let admin_user = normalized_admin_user(&admin_user);
    let (status, response) = send_json_request(
        &headers,
        Some(&admin_user),
        Method::GET,
        &format!("/api/graph/documents/{document_id}"),
        None,
        None,
    )
    .await?;

    Ok((status, Json(response)))
}

import requests
from flask import Flask, Response, request
from flask_cors import CORS

app = Flask(__name__)
CORS(app)

# 您架設好的正式 API 伺服器位址
TARGET_API_SERVER = "http://120.110.113.111:3000"


# 捕捉所有路徑與 HTTP 方法 (GET, POST, PUT, DELETE 等)
@app.route(
    "/",
    defaults={"path": ""},
    methods=["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"],
)
@app.route("/<path:path>", methods=["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
def proxy(path):
    """
    代理(Proxy)路由：
    接收來自 React 的請求，轉發給目標 API Server，再把結果傳回給 React。
    """
    # 如果是 OPTIONS 請求 (CORS 預檢)，直接回覆 OK
    if request.method == "OPTIONS":
        return Response("", status=200)

    target_url = f"{TARGET_API_SERVER}/{path}"

    # 1. 複製前端傳來的 Headers (過濾掉 Host，避免目標伺服器解析錯誤)
    headers = {key: value for (key, value) in request.headers if key.lower() != "host"}

    # 2. 處理檔案上傳 (針對 /diet 圖片辨識的 multipart/form-data)
    if request.files:
        files = {}
        for key, file in request.files.items():
            # 將上傳的檔案讀取出來準備轉發
            files[key] = (file.filename, file.stream.read(), file.content_type)

        # requests 套件傳送檔案時會自動產生正確的 boundary，所以必須刪除舊的 Content-Type
        if "Content-Type" in headers:
            del headers["Content-Type"]

        # 轉發帶有檔案的請求
        resp = requests.request(
            method=request.method,
            url=target_url,
            headers=headers,
            data=request.form,
            files=files,
        )
    else:
        # 轉發一般的 JSON 或 GET 請求
        resp = requests.request(
            method=request.method,
            url=target_url,
            headers=headers,
            data=request.get_data(),
        )

    # 3. 構造要回傳給前端的 Response
    # 過濾掉會干擾瀏覽器的特定 Headers (例如分塊傳輸編碼等)
    excluded_headers = [
        "content-encoding",
        "content-length",
        "transfer-encoding",
        "connection",
    ]
    resp_headers = [
        (name, value)
        for (name, value) in resp.headers.items()
        if name.lower() not in excluded_headers
    ]

    return Response(resp.content, resp.status_code, resp_headers)


if __name__ == "__main__":
    # 啟動 Flask 伺服器在 5000 port
    app.run(port=5000, debug=True)

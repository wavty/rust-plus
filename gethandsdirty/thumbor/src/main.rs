use std::net::SocketAddr;

use axum::{extract::Path, http::StatusCode, routing::get, Router};
use pb::ImageSpec;
use percent_encoding::percent_decode_str;
use serde::Deserialize;
mod pb;

// 测试代码
// ./httpie get "http://127.0.0.1:3000/image/CgoKCAjYBBCgBiADCgY6BAgUEBQKBDICCAM/https%3A%2F%2Fimages.pexels.com%2Fphotos%2F2470905%2Fpexels-photo-2470905.jpeg"

// 参数实现 Deserialize trait，则 axum 会自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    // 定义路由
    let app = Router::new().route("/image/:spec/:url", get(generate));
    // 运行web服务器
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    tracing::info!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}

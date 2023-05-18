use std::{
    net::SocketAddr,
    num::NonZeroUsize,
    sync::{Arc, Mutex},
};

use axum::{extract::Path, http::StatusCode, routing::get, Router};
use bytes::Bytes;
use lru::LruCache;
use pb::ImageSpec;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

use crate::pb::{filter, resize, Spec};
mod pb;

// 测试代码
// ./httpie get "http://127.0.0.1:3000/image/CgoKCAjYBBCgBiADCgY6BAgUEBQKBDICCAM/https%3A%2F%2Fimages.pexels.com%2Fphotos%2F2470905%2Fpexels-photo-2470905.jpeg"

// 参数实现 Deserialize trait，则 axum 会自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(1024).unwrap())));
    // 定义路由
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );
    // 运行web服务器
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    tracing::info!("listening on http://{}", addr);
    print_test_url("https://img-blog.csdnimg.cn/7ab510402f3e4f8295d76764a3639eed.png");
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
    Ok(format!("url: {},\nspec: {:#?}", url, spec))
}

fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    tracing::info!("test url: http://127.0.0.1:3000/image/{}/{}", s, test_image);
}

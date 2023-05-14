# 设计 thumbor

## 一、架构

![thumbor-server](../images/thumbor-server.png)

## 二、基础

### 1. Axum

Axum 是一个基于 Rust 语言的 Web 框架，其设计初衷是提供一种高性能、可扩展的异步编程模型，使得构建高吞吐量的 Web 应用程序变得更加容易。具体来说，Axum 的使用场景包括但不限于以下几种：

1. 构建高性能的 Web 服务：Axum 采用异步编程模型，可利用 Rust 的内存安全和高性能的特性来构建高吞吐量的 Web 服务。

2. 支持大规模并发：Axum 基于 Tokio 异步运行时，可以有效地支持大规模并发请求。

3. 容易扩展：Axum 采用模块化设计，使得开发人员可以轻松添加自定义功能，并与其他 Rust 库和工具进行集成。

在底层的实现原理方面，Axum 主要依赖于 Tokio 异步运行时和 hyper HTTP 库。它提供了一个基于 async/await 的编程模型，通过 Rust 的 futures 和 async/await 语法来处理异步任务。Axum 还支持基于中间件的管道设计，开发人员可以通过中间件来添加额外的功能或对请求和响应进行修改。

下面是一个简单的 Axum 应用程序示例，它创建了一个 HTTP 服务器，并处理了 GET 请求：

```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

在上面的代码中，我们首先创建了一个 Router 对象，它定义了一个根路由"/"，它使用 get()处理器来处理 GET 请求并返回一个字符串"Hello, Axum!"。然后，我们将这个 Router 对象绑定到地址 127.0.0.1:3000 上，并启动 HTTP 服务器。当我们在浏览器中访问 http://localhost:3000/时，我们将看到"Hello, Axum!"的响应。

use http_body_util::Full;
use hyper::{
    body::Bytes,
    header::CONTENT_TYPE,
    Method, Request, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio;

static INDEX: &str = "app/index.html";
static PACKAGE: &str = "app/pkg/";
static RES: &str = "app/res/";

pub async fn handle_conection(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match (request.method(), request.uri().path()) {
        (&Method::GET, path) => {
            if path.starts_with("/pkg") {
                let path = path.replace("/pkg/", "");
                let filename = PACKAGE.to_string() + path.as_str();

                return Ok(send_file(filename.as_str()).await);
            } else if path.starts_with("/res") {
                let path = path.replace("/res/", "");
                let filename = RES.to_string() + path.as_str();

                return Ok(send_file(filename.as_str()).await);
            }

            Ok(send_file(INDEX).await)
        },
        _ => Ok(not_found().await),
    }
}

async fn index_body() -> Vec<u8> {
    tokio::fs::read(INDEX).await.unwrap()
}

async fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::from(index_body().await))
        .unwrap()
}

async fn send_file(filename: &str) -> Response<Full<Bytes>> {
    if let Ok(contents) = tokio::fs::read(filename).await {
        let mut builder = Response::builder();

        if filename.ends_with(".js") {
            builder = builder.header(CONTENT_TYPE, "text/javascript");
        } else if filename.ends_with(".woff2") {
            builder = builder.header(CONTENT_TYPE, "font/woff")
        } else if filename.ends_with(".css") {
            builder = builder.header(CONTENT_TYPE, "text/css")
        } else if filename.ends_with(".json") {
            builder = builder.header(CONTENT_TYPE, "text/json")
        } else if filename.ends_with(".wasm") {
            builder = builder.header(CONTENT_TYPE, "application/wasm")
        }

        return builder.body(contents.into()).unwrap();
    }

    not_found().await
}

#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
    email: String,
    subject: String,
    message: String,
}

use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::{
    body::{Body, Bytes},
    header::{CONTENT_TYPE, LOCATION},
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
        }
        (&Method::POST, _) => match get_body(request).await {
            Ok(body) => {
                let Ok(message): Result<IncomingMessage, _> = serde_qs::from_bytes(&body as &[u8]) else {
                        return Ok(bad_request("Message was malformed"));
                    };

                dbg!(message);

                Ok(recieved_msg().await)
            }
            Err(GetBodyError::TooBig) => Ok(payload_too_large()),
            Err(GetBodyError::ConnectionFailed) => todo!(),
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

fn payload_too_large() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::PAYLOAD_TOO_LARGE)
        .body(Full::new("request too large, must be <= 64kb".into()))
        .unwrap()
}

fn bad_request(message: &'static str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(message.into()))
        .unwrap()
}

async fn recieved_msg() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::CREATED)
        .header(LOCATION, "/submitted")
        .body(Full::new(index_body().await.into()))
        .unwrap()
}

#[derive(Debug)]
enum GetBodyError {
    TooBig,
    ConnectionFailed,
}

async fn get_body(request: Request<hyper::body::Incoming>) -> Result<Vec<u8>, GetBodyError> {
    let upper = request.body().size_hint().upper().unwrap_or(u64::MAX);
    if upper > 1024 * 64 {
        return Err(GetBodyError::TooBig);
    }

    if let Ok(bytes) = request.collect().await {
        Ok(bytes.to_bytes().iter().cloned().collect())
    } else {
        Err(GetBodyError::ConnectionFailed)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
    email: String,
    subject: String,
    message: String,
}

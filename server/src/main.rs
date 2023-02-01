use http_body_util::{Full, BodyExt};
use hyper::{body::{Bytes, Body}, server::conn::http1, service::service_fn, Request, Response, StatusCode, Method, header::{CONTENT_TYPE, LOCATION}};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socket2::{Domain, Type, Socket};
use std::{
    convert::Infallible, error::Error, fs, io::Write, net::{TcpListener, SocketAddr}, thread, time::Duration,
};
use tokio::{process::Command};

const PAGES: &[&str] = &["open-source", "projects"];
static INDEX: &str = "app/index.html";
static PACKAGE: &str = "app/pkg/";
static RES: &str = "app/res/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    bulid().await;

    tokio::spawn(sync_github_data());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));

    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;

    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;

    socket.bind(&addr.into())?;
    socket.listen(128)?;

    let listener: TcpListener = socket.into();
    loop {
        let (stream, _) = listener.accept()?;

        let tokio_stream = tokio::net::TcpStream::from_std(stream)?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(tokio_stream, service_fn(handle_conection))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn handle_conection(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => Ok(send_file(INDEX).await),
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

            Ok(not_found().await)
        }
        (&Method::POST, _) => {
            match  get_body(request).await {
                Ok(body) => {
                    let Ok(message): Result<IncomingMessage, _> = serde_qs::from_bytes(&body as &[u8]) else {
                        return Ok(bad_request("Message was malformed"));
                    };

                    dbg!(message);

                    Ok(recieved_msg().await)
                },
                Err(GetBodyError::TooBig) =>  Ok(payload_too_large()),
                Err(GetBodyError::ConnectionFailed) => todo!(),
            }
        }
        _ => Ok(not_found().await)
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

fn payload_too_large() ->Response<Full<Bytes>> {
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
    ConnectionFailed
}

async fn get_body(request: Request<hyper::body::Incoming>) -> Result<Vec<u8>, GetBodyError> {
    let upper = request.body().size_hint().upper().unwrap_or(u64::MAX);
    if upper > 1024 * 64 {
        return Err(GetBodyError::TooBig);
    }

    if let Ok(bytes) =  request.collect().await {
        Ok(bytes.to_bytes().iter().cloned().collect())
    } else {
        Err(GetBodyError::ConnectionFailed)
    }
}

async fn bulid() {
    println!("compiling app");

    let mut cmd = Command::new("wasm-pack");

    let cmd = cmd
        .current_dir(fs::canonicalize("app").expect("app folder does not exist"))
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("web");

    cmd.spawn()
        .expect("failed to spawn command")
        .wait()
        .await
        .expect("failed to compile app");
}

async fn sync_github_data() {
    loop {
        println!("getting updated data from github...");

        let octocrab = octocrab::instance();

        for i in PAGES {
            let Ok(input_file) = fs::read_to_string(fs::canonicalize(format!("app/res/{}.csv", i)).unwrap()) else {
                println!("failed to load {}.csv", i);
                continue;
            };

            let Ok(mut file) = fs::File::create(format!("app/res/{}.json", i)) else {
                println!("failed to create {}.json", i);
                continue;
            };

            let mut contents: Vec<RepoInfo> = vec![];

            for line in input_file.lines().skip(1) {
                let mut split = line.split(",");

                let (Some(owner), Some(repo)) = (split.next(), split.next()) else {
                    println!("Incorrectly formatted line in {}: {}", i, line);
                    continue;
                };

                println!("{}/{}", &owner, &repo);

                let repo = match octocrab.repos(owner, repo).get().await {
                    Ok(repository) => repository,
                    Err(err) => {
                        println!("Error connecting to Github: {}", err);
                        continue;
                    }
                };

                contents.push(RepoInfo {
                    url: repo.html_url.expect("Project didn't have a url for some reason.").to_string(),
                    name: repo.name,
                    desc: repo.description.unwrap_or("Project contains no description".to_string()),
                    lang: repo.language.unwrap_or(Value::from("Project does not report primary language")).to_string().trim_matches('"').to_string(),
                });
            }

            file.write(serde_json::to_string(&contents).unwrap().as_bytes())
                .expect("Failed to write to file");
        }

        println!("done");

        thread::sleep(Duration::from_secs(86400));
    }
}

#[derive(Serialize, Deserialize)]
struct RepoInfo {
    url: String,
    name: String,
    desc: String,
    lang: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
    email: String,
    subject: String,
    message: String,
}
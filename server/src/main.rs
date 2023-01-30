use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response, StatusCode, Method, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
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

            Ok(not_found())
        }
        _ => Ok(not_found())
    }
}

fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new("Not Found".into()))
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

    not_found()
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
                    name: repo.full_name.unwrap(),
                    desc: repo.description.unwrap(),
                    lang: repo.language.unwrap().to_string(),
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
    name: String,
    desc: String,
    lang: String,
}

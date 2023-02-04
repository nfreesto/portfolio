use hyper::{server::conn::http1, service::service_fn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use server::handle_conection;
use socket2::{Domain, Socket, Type};
use std::{
    error::Error,
    fs,
    io::Write,
    net::{SocketAddr, TcpListener},
    thread,
    time::Duration,
};
use tokio::process::Command;


const PAGES: &[&str] = &["open-source", "projects"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // bulid().await;

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
                    url: repo
                        .html_url
                        .expect("Project didn't have a url for some reason.")
                        .to_string(),
                    name: repo.name,
                    desc: repo
                        .description
                        .unwrap_or("Project contains no description".to_string()),
                    lang: repo
                        .language
                        .unwrap_or(Value::from("Project does not report primary language"))
                        .to_string()
                        .trim_matches('"')
                        .to_string(),
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

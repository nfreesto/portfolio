use std::{fs, io::Write, thread, time::Duration, net::TcpListener};
use serde::{Serialize, Deserialize};
use tokio::{process::Command};
use warp::{Filter};

const PAGES: &[&str] = &["open-source", "projects"];

#[tokio::main]
async fn main() {
    bulid().await;

    tokio::spawn(sync_github_data());

    let index = warp::get().and(warp::fs::file("app/index.html"));

    let core = warp::path("pkg").and(warp::fs::dir("app/pkg"));

    let resources = warp::path("res").and(warp::fs::dir("app/res"));

    let server_res = warp::path("server_res").and(warp::fs::dir("server/res"));

    let routes = core.or(resources).or(server_res).or(index);
    
    println!("Serving on localhost:3030");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
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

    cmd.spawn().expect( "failed to spawn command").wait().await.expect("failed to compile app");
}

async fn sync_github_data(){
    loop {
        println!("getting updated data from github...");

        let octocrab = octocrab::instance();

        for i in PAGES {
            let Ok(input_file) = fs::read_to_string(fs::canonicalize(format!("server/res/{}.csv", i)).unwrap()) else {
                println!("failed to load {}.csv", i);
                continue;
            };

            let Ok(mut file) = fs::File::create(format!("server/res/{}.json", i)) else {
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
                
                contents.push(
                    RepoInfo {
                        name: repo.full_name.unwrap(),
                        desc: repo.description.unwrap(),
                        lang: repo.language.unwrap().to_string()
                    }
                );
            }

            file.write(serde_json::to_string(&contents).unwrap().as_bytes()).expect("Failed to write to file");
        }

        println!("done");

        thread::sleep(Duration::from_secs(86400));
    }
}


#[derive(Serialize, Deserialize)]
struct RepoInfo {
    name: String,
    desc: String,
    lang: String
}
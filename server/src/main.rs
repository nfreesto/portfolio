use std::{fs, io::Write, thread, time::Duration};
use tokio::process::Command;
use warp::Filter;

const PAGES: &[&str] = &["open-source", "projects"];

#[tokio::main]
async fn main() {
    bulid().await;

    tokio::spawn(sync_github_data());

    let index = warp::get().and(warp::fs::file("app/index.html"));

    let core = warp::path("pkg").and(warp::fs::dir("app/pkg"));

    let resources = warp::path("res").and(warp::fs::dir("app/res"));

    let server_res = warp::path("res").and(warp::fs::dir("server/res"));

    let routes = core.or(resources).or(server_res).or(index);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn bulid() {
    let mut cmd = Command::new("wasm-pack");
    let cmd = cmd
        .current_dir("app")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("web");

    cmd.spawn().expect("failed to spawn command");
}

async fn sync_github_data() {
    loop {
        let octocrab = octocrab::instance();

        for i in PAGES {
            let Ok(input_file) = fs::read_to_string(format!("server/resources/{}.csv", i)) else {
                println!("failed to load {} csv", i);
                continue;
            };

            let Ok(mut file) = fs::File::create(format!("server/resources/{}", i)) else {
                println!("failed to create {}", i);
                continue;
            };

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

                let Ok(_) = file.write(
                    format!(
                        "<div class=\"repo-info-card\">
    <h2>{{ \"{}\" }}</h2>
    <p class=\"small\">{{ \"{}\" }}</p>
    <p class=\"small\">{{ \"{}\" }}</p>
</div>
                        ",
                        repo.full_name.unwrap(),
                        repo.description.unwrap(),
                        repo.language.unwrap()
                    )
                    .as_bytes(),
                ) else {
                    println!("Failed to write {}", line);
                    continue;
                };
            }
        }

        thread::sleep(Duration::from_secs(86400));
    }
}

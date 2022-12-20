use warp::Filter;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    bulid().await;

    let index = warp::get()
        .and(warp::fs::file("../app/index.html"));

    let core = warp::path("pkg")
        .and(warp::fs::dir("../app/pkg"));

    let node_modules = warp::path("node_modules")
        .and(warp::fs::dir("../app/node_modules"));

    let routes = core.or(index);


    warp::serve(routes)
        .run(([127,0,0,1], 3030)).await;
}

async fn bulid() {
    let wasm_path = "../app";
    let mut cmd = Command::new("wasm-pack");
    let cmd = cmd
        .current_dir(wasm_path)
        .arg("build");

    let mut child = cmd.spawn().expect("failed to spawn command");
}
use serde::{Deserialize, Serialize};
use yew::{html, Html};

pub async fn get_open_source() -> Vec<RepoInfo> {
    let json = reqwest::get("localhost:8000/server_res/open-source.json")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str(json.as_str()).unwrap()
}

#[allow(unused)]
pub async fn get_projects() -> String {
    reqwest::get("localhost:3030/res/projects")
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct RepoInfo {
    name: String,
    desc: String,
    lang: String,
}

impl RepoInfo {
    pub fn to_html(&self) -> Html {
        html! {
            <div class="repo">
                <p>{ &self.name }</p>
                <p>{ &self.desc }</p>
                <p>{ &self.lang }</p>
            </div>
        }
    }
}

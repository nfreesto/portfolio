use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::{html, Html};

#[wasm_bindgen]
extern "C" {
    static OPENSOURCE: JsValue;
    static PROJECTS: JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn get_open_source() -> Vec<RepoInfo> {
    serde_wasm_bindgen::from_value(OPENSOURCE.clone()).unwrap()
}

pub fn get_projects() -> Vec<RepoInfo> {
    serde_wasm_bindgen::from_value(PROJECTS.clone()).unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct RepoInfo {
    name: String,
    desc: String,
    lang: String,
}

impl RepoInfo {
    pub fn to_html(&self) -> Html {
        let href = format!("https://github.com/{}", self.name);

        html! {
            <a href={href} class="repo" target="_blank" rel="noopener noreferrer">
                <h1 href="">{ &self.name }</h1>
                <p class="small">{ &self.desc }</p>
                <h2>{ &self.lang }</h2>
            </a>
        }
    }
}

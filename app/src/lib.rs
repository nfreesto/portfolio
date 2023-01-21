mod app;
mod pages;

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn run() {
    yew::Renderer::<App>::new().render();
}
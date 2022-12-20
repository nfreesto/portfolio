mod app;

use app::App;
use wasm_bindgen::prelude::*;
use yew::prelude::*;


#[wasm_bindgen]
pub fn run() {
    yew::Renderer::<App>::new().render();
}
use std::vec;

use futures::FutureExt;
use yew::prelude::*;

use super::content_fetcher::{get_open_source, RepoInfo};

pub struct OpenSource;



impl Component for OpenSource {
    type Message = ();
    type Properties = ();


    fn view(&self, _ctx: &Context<Self>) -> Html {
        let content: Vec<Html> = get_open_source().iter().map(|x| x.to_html()).collect();
        html! {
            <div class="content">
                <p>{ "Open Source Content" }</p>
                { for content }
            </div>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }
}

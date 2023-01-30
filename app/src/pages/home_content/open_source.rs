use std::vec;

use futures::FutureExt;
use yew::prelude::*;

use super::content_fetcher::{get_open_source, RepoInfo};

pub struct OpenSource {
    content: Vec<Html>,
}

pub enum Msg {
    ContentLoaded(Vec<RepoInfo>),
}

impl Component for OpenSource {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let future = get_open_source();

        ctx.link().send_future(future.map(Msg::ContentLoaded));

        Self { content: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContentLoaded(info) => {
                self.content = info.iter().map(|repo_info| repo_info.to_html()).collect();

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="content">
                <p>{ "Open Source Content" }</p>
                { for self.content.clone() }
            </div>
        }
    }
}

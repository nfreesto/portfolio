use futures::FutureExt;
use yew::prelude::*;

use super::content_fetcher::get_open_source;

pub struct OpenSource {
    content: String
}

pub enum Msg {
    ContentLoaded(String),
}

impl Component for OpenSource {

    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let future = get_open_source();

        ctx.link().send_future(future.map(Msg::ContentLoaded));

        Self {
            content: "".to_string()
         }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContentLoaded(html) => {
                self.content = html;
                true
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div class="content">
                <p>{ "Open Source Content" }</p>
                { self.content.clone() }
            </div>
        }
    }
}
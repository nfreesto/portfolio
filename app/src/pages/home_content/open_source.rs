use super::content_fetcher::get_open_source;
use yew::prelude::*;

pub struct OpenSource;

impl Component for OpenSource {
    type Message = ();
    type Properties = ();

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let content: Vec<Html> = get_open_source().iter().map(|x| x.to_html()).collect();
        html! {
            <div class="content">
                <p>{ "Open Source projects I've contributed to:" }</p>
                <div class="flexbox">
                    if !content.is_empty() {
                        { for content }
                    } else {
                        <p>{ "Oops, looks like this content failed to load! Please try again later." }</p>
                    }
                </div>
            </div>
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
}

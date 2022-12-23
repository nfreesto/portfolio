use yew::prelude::*;

pub struct OpenSource;

impl Component for OpenSource {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div class="content">
                <p>{ "Open Source Content" }</p>
            </div>
        }
    }
}
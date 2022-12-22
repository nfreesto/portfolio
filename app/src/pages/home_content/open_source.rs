use yew::prelude::*;

pub struct OpenSource;

impl Component for OpenSource {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <p>{ "Open Source Content" }</p>
            </div>
        }
    }
}
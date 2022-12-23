use yew::prelude::*;

pub struct DefaultContent;

impl Component for DefaultContent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div class="content">
                <p>{ "Default Content" }</p>
            </div>
        }
    }
}
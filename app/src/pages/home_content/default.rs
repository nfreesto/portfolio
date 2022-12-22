use yew::prelude::*;

pub struct DefaultContent;

impl Component for DefaultContent {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <p>{ "Default Content" }</p>
            </div>
        }
    }
}
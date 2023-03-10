use yew::prelude::*;

pub struct NotFound;

impl Component for NotFound {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1 class="error">{ "404 Not Found" }</h1>
        }
    }
}

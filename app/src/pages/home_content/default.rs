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
                <p id="default-content">{ "I'm a " }
                <span class="purple-emph">{ "developer " }</span>
                { "based in Northern Utah. I like to write code, and I'd love to work with you." }
                </p>
            </div>
        }
    }
}
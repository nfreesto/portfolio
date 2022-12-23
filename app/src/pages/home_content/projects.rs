use yew::prelude::*;

pub struct Projects;

impl Component for Projects {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div class="content">
                <p>{ "Projects Content" }</p>
            </div>
        }
    }
}
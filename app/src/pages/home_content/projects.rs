use yew::prelude::*;

pub struct Projects;

impl Component for Projects {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <p>{ "Projects Content" }</p>
            </div>
        }
    }
}
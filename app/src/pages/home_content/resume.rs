use yew::prelude::*;

pub struct Resume;

impl Component for Resume {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <p>{ "Resume Content" }</p>
            </div>
        }
    }
}
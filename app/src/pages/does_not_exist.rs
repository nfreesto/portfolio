use yew::prelude::*;

pub struct DoesNotExist;

impl Component for DoesNotExist {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Oops! This page doesn't exist yet!" }</h1>
        }
    }
}
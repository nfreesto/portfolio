use yew::prelude::*;

pub struct DoesNotExist;

impl Component for DoesNotExist {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Oops! This page doesn't exist yet!" }</h1>
        }
    }
}
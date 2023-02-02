use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Submitted;

impl Component for Submitted {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <h1>{ "Form sucessfully submitted" }</h1>
            <Link<Route> to={Route::Home}>{ "Return Home" }</Link<Route>>
            <Link<Route> to={Route::Contact}>{ "Send another message" }</Link<Route>>
            </>
        }
    }
}

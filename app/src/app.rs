use crate::pages::{Home, NotFound};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!(<Home />),
        Route::NotFound => html!(<NotFound />),
        Route::Contact => html!(<NotFound />),
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}
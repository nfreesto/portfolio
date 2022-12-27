use crate::pages::{Home, NotFound};
use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::Global;

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
        let root_style = stylist::css!{
            "
            font-family: JetBrainsMono;
            color: #ffffff;
            background-color: #1e1e1e;
            --button-width: 130px;
            --header-min-width: calc(6*var(--button-width));
            --default-position: min(10vw, calc((100vw - 6*var(--button-width))/2));
            --open-source-position: calc(var(--default-position) + var(--button-width));
            --projects-position: calc(var(--default-position) + (var(--button-width) * 2));
            --resume-position: calc(var(--default-position) + (var(--button-width) * 3));
            --header-right: min(10vw, calc((100vw - 6*var(--button-width))/2));
            "
        };

        let other_style = stylist::css!{
            "
            button {
                font-family: inherit;
                font-size: inherit;
                background-color: inherit;
                border: 0px;
                color: inherit;
                padding: 0px;
                line-height: 20px;
            }

            hr {
                color: #414141;
            }
            "
        };

        html!{
            <BrowserRouter>
                <Global css={root_style} />
                <Global css={other_style} />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}
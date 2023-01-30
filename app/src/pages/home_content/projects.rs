use yew::prelude::*;

use super::content_fetcher::get_projects;

const _REPOS: &[&str] = &["https://github.com/nfreesto/portfolio"];
pub struct Projects;

impl Component for Projects {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let content: Vec<Html> = get_projects().iter().map(|x| x.to_html()).collect();
        html! {
            <div class="content">
                <p>{ "Projects Content" }</p>
                { for content }
            </div>
        }
    }
}

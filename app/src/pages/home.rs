use yew::prelude::*;
use super::home_content::{DefaultContent, OpenSource, Projects, Resume};

#[derive(PartialEq)]
enum State {
    Default,
    OpenSource,
    Projects,
    Resume
}

fn default_content() -> Html {
    html!{
        <div>
            {"Hello"}
        </div>
    }
}

pub enum Msg {
    GotoDefault,
    GotoOpenSource,
    GotoProjects,
    GotoResume
}

pub struct Home {
    state: State
}


impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: State::Default
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotoDefault => {
                self.state = State::Default;
                true
            },
            Msg::GotoOpenSource => {
                self.state = State::OpenSource;
                true
            },
            Msg::GotoProjects => {
                self.state = State::Projects;
                true
            },
            Msg::GotoResume => {
                self.state = State::Resume;
                true
            },
        }

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div>
                <h1>{"Welcome Home"}</h1>
                <button onclick={ctx.link().callback(|_| Msg::GotoDefault)}>{ "Default" }</button>
                <button onclick={ctx.link().callback(|_| Msg::GotoOpenSource)}>{ "Open Source" }</button>
                <button onclick={ctx.link().callback(|_| Msg::GotoProjects)}>{ "Projects" }</button>
                <button onclick={ctx.link().callback(|_| Msg::GotoResume)}>{ "Resume" }</button>
                { self.content() }
            </div>
        )
    }
}

impl Home {
    fn content(&self) -> Html {
        match self.state {
            State::Default => html!(<DefaultContent />),
            State::OpenSource => html!(<OpenSource />),
            State::Projects => html!(<Projects />),
            State::Resume => html!(<Resume />),
        }
    }
}
use yew::prelude::*;
use super::home_content::{DefaultContent, OpenSource, Projects, Resume};

#[derive(PartialEq)]
enum State {
    Default,
    OpenSource,
    Projects,
    Resume
}

pub enum Msg {
    Goto(State)
}

pub struct Home {
    state: State,
    button_class: String,
    selector_class: String,
    hr_class: String,
}


impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: State::Default,
            button_class: String::from(""),
            selector_class: String::from(""),
            hr_class: String::from(""),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Goto(state) => {
                let animate_class: String = format!("animate-{}-to-{}",
                    match self.state {
                        State::Default => "default",
                        State::OpenSource => "open",
                        State::Projects => "projects",
                        State::Resume => "resume",
                    },
                    match state {
                        State::Default => "default",
                        State::OpenSource => "open",
                        State::Projects => "projects",
                        State::Resume => "resume",
                    }
                );

                if self.state != state {
                    self.button_class = animate_class.clone();
                    self.hr_class = animate_class.clone();
                    self.selector_class = animate_class;
                }

                self.state = state;
                true
            }
        }

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div>
                { self.header(ctx) }
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

    fn header(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <button id="moving-button" class={classes!(&self.button_class)} onclick={ctx.link().callback(|_| Msg::Goto(State::Default))}>
                { "Nathan" }<br />
                { "Freestone" }<br />
                { "-" }<br />
                { "Developer" }
            </button>
            <div id="header">
                <div>
                    <div id="selection-container">
                        <button class={classes!(&self.selector_class)} id="open-source-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::OpenSource))}>{ "Open Source" }</button>
                        <button class={classes!(&self.selector_class)} id="projects-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::Projects))}>{ "Projects" }</button>
                        <button class={classes!(&self.selector_class)} id="resume-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::Resume))}>{ "Resume" }</button>
                    </div>
                    <div id="links-container">
                        <div><a href="https://github.com/nfreesto">{ "Github" }</a></div>
                        <div><a href="nfreesto@gmail.com">{ "Contact" }</a></div>
                    </div>
                </div>
                <hr class={classes!(&self.hr_class)}/>
            </div>
            </div>
        }
    }

}
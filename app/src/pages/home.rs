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
    animate_class: String
}


impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: State::Default,
            animate_class: String::from(""),
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
                    self.animate_class = animate_class;
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
            <button id="moving-button" class={classes!(&self.animate_class)} onclick={ctx.link().callback(|_| Msg::Goto(State::Default))}>
                { "Nathan" }<br />
                { "Freestone" }<br />
                { "-" }<br />
                { "Developer" }
            </button>
            <div id="selection-container">
                        <button class={classes!(&self.animate_class)} id="open-source-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::OpenSource))}>{ "Open Source" }</button>
                        <button class={classes!(&self.animate_class)} id="projects-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::Projects))}>{ "Projects" }</button>
                        <button class={classes!(&self.animate_class)} id="resume-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::Resume))}>{ "Resume" }</button>
                    </div>
            <div id="header">
                <div>
                    // This is the wrong solution, but I'm not sure how else to do a placeholder like this
                    <div id="selection-container-copy">
                        <button>{ "Open Source" }</button>
                        <button>{ "Projects" }</button>
                        <button>{ "Resume" }</button>
                    </div>
                    <div id="links-container">
                        <div><a target="_blank" rel="noopener noreferrer" href="https://github.com/nfreesto">{ "Github" }</a></div>
                        <div><a target="_blank" href="./contact">{ "Contact" }</a></div>
                    </div>
                </div>
                <hr class={classes!(&self.animate_class)}/>
            </div>
            </div>
        }
    }

}
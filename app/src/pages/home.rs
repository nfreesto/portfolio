use super::home_content::{DefaultContent, OpenSource, Projects, Resume};
use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq)]
pub enum State {
    Default,
    OpenSource,
    Projects,
    Resume,
}

#[derive(PartialEq)]
pub enum HeaderState {
    Open,
    Closed,
    Initial,
}

pub enum Msg {
    Goto(State),
    HeaderGoto(HeaderState),
}

pub struct Home {
    state: State,
    header_state: HeaderState,
    animate_class: String,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: State::Default,
            header_state: HeaderState::Initial,
            animate_class: String::from(""),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Goto(state) => {
                let animate_class: String = format!(
                    "animate-{}-to-{}",
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
                if self.header_state != HeaderState::Initial {
                    self.header_state = HeaderState::Closed;
                }
                self.state = state;
                true
            }
            Msg::HeaderGoto(header_state) => {
                self.header_state = header_state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <>
                { self.header(ctx) }
                { self.skinny_header(ctx) }
                { self.content() }
            </>
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
            <div id="header-container">
            <button id="moving-button" class={classes!(&self.animate_class, "animated".to_string())} onclick={ctx.link().callback(|_| Msg::Goto(State::Default))}>
                { "Nathan" }<br />
                { "Freestone" }<br />
                { "-" }<br />
                <span class="purple-emph">{ "Developer" }</span>
            </button>
            <div id="selection-container">
                        <button class={classes!(&self.animate_class, "animated".to_string())} id="projects-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::Projects))}>{ "Projects" }</button>
                        <button class={classes!(&self.animate_class, "animated".to_string())} id="resume-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::Resume))}>{ "Resume" }</button>
                        <button class={classes!(&self.animate_class, "animated".to_string())} id="open-source-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::OpenSource))}>{ "Open Source" }</button>
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
                        <div><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></div>
                    </div>
                </div>
                <hr class={classes!(&self.animate_class, "animated".to_string())}/>
            </div>
            </div>
        }
    }

    fn skinny_header(&self, ctx: &Context<Self>) -> Html {
        let animation_class = match self.header_state {
            HeaderState::Open => "open-header",
            HeaderState::Closed => "close-header",
            HeaderState::Initial => "",
        }
        .to_string();

        html! {
            <div id="skinny-header-container">
                <div id="skinny-header" class={classes!("animated", animation_class)}>
                    <div id="skinny-header-top">
                        <button onclick={ctx.link().callback(|_| Msg::Goto(State::Default))}>{ "Nathan Freestone" }</button>
                        if self.header_state == HeaderState::Open {
                            <button onclick={ctx.link().callback(|_| Msg::HeaderGoto(HeaderState::Closed))}>{ "Close Menu" }</button>
                        } else {
                            <button onclick={ctx.link().callback(|_| Msg::HeaderGoto(HeaderState::Open))}>{ "Open Menu" }</button>
                        }
                    </div>
                    <div id="skinny-header-content">
                        <button class={classes!(&self.animate_class, "animated".to_string())} id="open-source-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::OpenSource))}>{ "Open Source" }</button>
                        <button class={classes!(&self.animate_class, "animated".to_string())} id="projects-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::Projects))}>{ "Projects" }</button>
                        <button class={classes!(&self.animate_class, "animated".to_string())} id="resume-selector" onclick={ctx.link().callback(|_| Msg::Goto(State::Resume))}>{ "Resume" }</button>
                    </div>
                    <hr />
                </div>
            </div>
        }
    }
}

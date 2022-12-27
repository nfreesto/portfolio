use yew::prelude::*;
use super::home_content::{DefaultContent, OpenSource, Projects, Resume};

#[derive(PartialEq)]
pub enum State {
    Default,
    OpenSource,
    Projects,
    Resume
}

#[derive(PartialEq)]
pub enum HeaderState{
    Open,
    Closed,
    Initial
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
                if self.header_state != HeaderState::Initial {
                    self.header_state = HeaderState::Closed;
                }
                self.state = state;
                true
            }
            Msg::HeaderGoto(header_state) => {
                self.header_state = header_state;
                true
            },
        }

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <>
                { self.header(ctx) }
                // { self.skinny_header(ctx) }
                // { self.content() }
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

        fn moving_button(_component: &Home, ctx: &Context<Home>) -> Html {
            let style = stylist::css!{
                "
                position: fixed;
                left: var(--default-position);
                top: 100px;
                "
            };

            let purple = stylist::css!("color: #804c9e;");

            html! {
                <button class={classes!(style)} onclick={ctx.link().callback(|_| Msg::Goto(State::Default))}>
                    { "Nathan" }<br />
                    { "Freestone" }<br />
                    { "-" }<br />
                    <span class={classes!(purple)}>{ "Developer" }</span>
                </button>
            }
        }

        fn selections(_component: &Home, ctx: &Context<Home>) -> Html {

            let open_source = stylist::css!{
                "
                position: fixed;
                left: var(--open-source-position);
                top: 160px;
                "
            };

            let projects = stylist::css!{
                "
                position: fixed;
                left: var(--projects-position);
                top: 160px;
                "
            };

            let resume = stylist::css!{
                "
                position: fixed;
                left: var(--resume-position);
                top: 160px;
                "
            };


            html! {
                <>
                    <button class={classes!(open_source)} onclick={ctx.link().callback(|_| Msg::Goto(State::OpenSource))}>{ "Open Source" }</button>
                    <button class={classes!(projects)} onclick={ctx.link().callback(|_| Msg::Goto(State::Projects))}>{ "Projects" }</button>
                    <button class={classes!(resume)} onclick={ctx.link().callback(|_| Msg::Goto(State::Resume))}>{ "Resume" }</button>
                </>
            }
        }

        fn links(_component: &Home, _ctx: &Context<Home>) -> Html {
            let style = stylist::css!{
                "
                display: flex;
                position:fixed;
                top: 160px;
                right: var(--header-right);
                "
            };

            let other_style = stylist::css!{
                "
                a {
                    width: var(--button-width);
                    text-align: right;
                }
                "
            };

            html! {
                <div class={classes!(style, other_style)}>
                    <a target="_blank" rel="noopener noreferrer" href="https://github.com/nfreesto">{ "Github" }</a>
                    <a href="./contact">{ "Contact" }</a>
                </div>
            }
        }

        fn hr(_component: &Home, _ctx: &Context<Home>) -> Html {
            let style = stylist::css!{
                "
                position: fixed;
                left: var(--default-position);
                right: var(--header-right);
                top: 180px;
                "
            };

            html!(<hr class={classes!(style)} />)
        }

        let style = stylist::css!(
            "@media only screen and (max-width: 780px) {
                display: none;
            }"
        );

        let other_style = stylist::css!{
            "
            hr {
                margin-bottom: 0px;
            }

            button {
                width: var(--button-width);
                text-align: left;
            }
            "
        };

        html! {
            <nav class={classes!(style, other_style)}>
                { moving_button(&self, ctx) }
                { selections(&self, ctx) }
                { links(&self, ctx) }
                { hr(&self, ctx) }
            </nav>
        }
    }

    fn skinny_header(&self, ctx: &Context<Self>) -> Html {
        let animation_class = match self.header_state {
            HeaderState::Open => "open-header",
            HeaderState::Closed => "close-header",
            HeaderState::Initial => "",
        }.to_string();

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
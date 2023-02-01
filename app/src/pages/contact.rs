use yew::prelude::*;

pub struct Contact;

impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <form action="/submitted" method="post">
                <h1>{ "Contact" }</h1><br/>
                <label for="email">{ "Email Address" }</label><br/>
                <input type="email" id="email" name="email" maxlength="1000" required=true/><br/>
                <label for="subject">{ "Subject" }</label><br/>
                <input type="text" id="email" name="subject" maxlength="1000"/><br/>
                <label for="message">{ "Message" }</label><br/>
                <input type="text" id="message" name="message" maxlength="14000" required=true/><br/>
                <input type="submit" value="Submit" />
            </form>
        }
    }
}
use yew::prelude::*;

pub struct Resume;

impl Component for Resume {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="content">
                <h1>{ "Resume" }</h1>
                <h2>{ "Education" }</h2>
                <h3>{ "Computer Science Student at USU" }</h3>
                <p>{ "Expected to graduate somtime May 2024-2025" }</p>
                <h3>{ "Graduated Copper Hills High School" }</h3>
                <p>{ "Graduated 2019 with an honor chord in STEM" }</p>
                <h2>{ "Skills" }</h2>
                <ul>
                    <li>
                        { "CS" }
                        <ul>
                            <li>{ "Rust" }</li>
                            <li>{ "Kotlin" }</li>
                            <li>{ "Python" }</li>
                            <li>{ "Java" }</li>
                            <li>{ "HTML, CSS, JavaScript" }</li>
                            <li>{ "Git" }</li>
                        </ul>
                    </li>
                    <li>
                        { "Soft Skills" }
                        <ul>
                            <li>{ "Work well with a team" }</li>
                            <li>{ "Strong communication skills" }</li>
                            <li>{ "Easy to work with" }</li>
                        </ul>
                    </li>
                    <li>
                        { "Other" }
                        <ul>
                            <li>{ "Solidworks" }</li>
                            <li>{ "3D Printing" }</li>
                            <li>{ "Microsoft Office and all that" }</li>
                        </ul>
                    </li>
                </ul>
                <h2>{ "Work Experience" }</h2>
                <p>{ "I don't have any work experience that is relevant to Computer Science at this time, As soon as I get some I'll populate it here. Since I graduated I've been working consistiently, with the major exception of during the Coronavirus pandemic. Most of my experience is in customer service, especially in the food service industry. I don't think it's really important to list the half dozen jobs I've worked since that point, and I don't feel the need to prove it because it's not exactly impressive either." }</p>
            </div>
        }
    }
}

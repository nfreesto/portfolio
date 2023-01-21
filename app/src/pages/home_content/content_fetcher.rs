use futures::{Stream, StreamExt};
use scraper::{Html, Selector};
use yew::{html};

pub async fn get_open_source() -> String {
    reqwest::get("localhost:3030/res/open-source").await.unwrap().text().await.unwrap()
}

pub async fn get_projects() -> String {
    reqwest::get("localhost:3030/res/projects").await.unwrap().text().await.unwrap()
}

// async fn api_to_html(owner: String, repo: String) -> yew::Html {
//     let repo = octocrab::instance().repos(owner, repo).get().await.unwrap();

//     html! {
//         <div class="repo-info-card">
//             <h2>{ repo.full_name.unwrap() }</h2>
//             <p class="small">{ repo.description.unwrap() }</p>
//             <p class="small">{ repo.language.unwrap() }</p>
//         </div>
//     }
// }

async fn _scrape_to_html(url: String) -> yew::Html {
        //send request to url, get the html out of that text

        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        //parse html
        let document = Html::parse_document(response.as_str());

        //initalize selectors
        let title_selector = Selector::parse("title").unwrap();
        let div_selector = Selector::parse("div").unwrap();
        let ul_selector = Selector::parse("ul").unwrap();
        let li_selector = Selector::parse("li").unwrap();
        let span_selector = Selector::parse("span").unwrap();

        //get the page title to extract the repo name and description
        let page_title = document
            .select(&title_selector)
            .next()
            .unwrap()
            .inner_html();
        let mut page_title_split = page_title.split(": ");

        let repo_name = page_title_split
            .next()
            .unwrap()
            .split("/")
            .last()
            .unwrap()
            .to_string();
        let repo_desc = page_title_split.next().unwrap().to_string();

        // get sidebar to extract primary language
        let divs = document.select(&div_selector);
        let sidebar = divs
            .into_iter()
            .filter(|element| element.value().attr("class") == Some("Layout-sidebar"))
            .next()
            .unwrap();

        let primary_lang = sidebar
            .select(&div_selector)
            .next()
            .unwrap()
            .select(&ul_selector)
            .last()
            .unwrap()
            .select(&li_selector)
            .next()
            .unwrap()
            .select(&span_selector)
            .next()
            .unwrap()
            .select(&span_selector)
            .next()
            .unwrap()
            .inner_html();

        html! {
            <div class="repo-info-card">
                <h2>{ repo_name }</h2>
                <p class="small">{ repo_desc }</p>
                <p class="small">{ primary_lang }</p>
            </div>
        }
    }
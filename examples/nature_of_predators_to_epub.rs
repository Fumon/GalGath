use std::error::Error;

use reqwest::header::USER_AGENT;

const FIRST_CHAPTER: &str = "https://www.reddit.com/r/HFY/comments/u19xpa/the_nature_of_predators/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let fullname = dbg!(galgath::get_reddit_link_fullname(FIRST_CHAPTER));
    let api_url = dbg!(galgath::make_link_url(fullname));

    let client = reqwest::Client::new();
    
    let resp = client
        .get(api_url)
        .header(USER_AGENT, galgath::USER_AGENT_STRING)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let link = &resp["data"]["children"][0]["data"].as_object().ok_or("no_data")?;

    let title = link["title"].as_str().unwrap_or_default();
    let author = link["author"].as_str().unwrap_or_default();
    let markdown = link["selftext"].as_str().ok_or("no link text")?;

    let mut book = crowbook::Book::new();
    book.set_options(&[
                     ("title", title),
                     ("author", author),
                     ("lang", "en")]);
    book.add_chapter_from_source(crowbook::Number::Default, markdown.as_bytes(), false)?;

    book.render_format_to_file("epub", format!("{}.epub", title))?;
    
    Ok(())
}

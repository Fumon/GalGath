
use anyhow::anyhow;
use crowbook::Book;

pub const USER_AGENT_STRING: &str = "linux:GalGath:v0.0.1 (by /u/fumon; personal epub converter)";

pub fn get_reddit_link_fullname(url: impl AsRef<str>) -> String {
    let parsed = url::Url::parse(url.as_ref()).unwrap();

    // Assume we're looking at a comment link for now
    // Like this: https://www.reddit.com/r/HFY/comments/u19xpa/the_nature_of_predators/

    let name = parsed.path_segments().unwrap().nth_back(2).unwrap();

    format!("t3_{}", name)
}

pub fn make_link_url(fullname: impl AsRef<str>) -> url::Url {
    url::Url::parse(format!("https://reddit.com/by_id/{}/.json", fullname.as_ref()).as_ref())
        .unwrap()
}

pub async fn retrieve_book_stuff_from_reddit_text_post(url: url::Url) -> Result<(String, String, String), anyhow::Error> {
    let client = reqwest::Client::new();
    
    let resp = client
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT_STRING)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let link = &resp["data"]["children"][0]["data"].as_object().ok_or(anyhow!("no_data"))?;

    let title = link["title"].as_str().unwrap_or_default();
    let author = link["author"].as_str().unwrap_or_default();
    let markdown = link["selftext"].as_str().ok_or(anyhow!("no link text"))?;

    Ok((title.to_owned(), author.to_owned(), markdown.to_owned()))
}

pub fn convert_book_stuff_to_crowbook((title, author, markdown): (String, String, String)) -> Result<Book, anyhow::Error> {

    let mut book = crowbook::Book::new();
    
    book.set_options(&[
                     ("title", title.as_ref()),
                     ("author", author.as_ref()),
                     ("lang", "en")]);
    book.add_chapter_from_source(crowbook::Number::Default, markdown.as_bytes(), false)?;

    Ok(book)
}

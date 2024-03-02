use std::error::Error;

use reqwest::Url;



const FIRST_CHAPTER: &str = "https://www.reddit.com/r/HFY/comments/u19xpa/the_nature_of_predators/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let fc_url = dbg!(Url::parse(FIRST_CHAPTER)?.join(".json")?);
    let resp = dbg!(reqwest::get(fc_url).await?).json::<std::collections::HashMap<String,String>>().await?;

    dbg!(resp);

    Ok(())
}
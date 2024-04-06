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

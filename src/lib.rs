extern crate reqwest;
extern crate serde;

use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Default)]
struct RedditAuthJson {
    access_token: String,
    _token_type: String,
    expires_in: u32,
    _scope: String,
}

pub fn get_access_token(
    username: &str,
    password: &str,
    app_id: &str,
    app_token: &str,
) -> Result<(String, u32), Box<dyn std::error::Error>> {
    let params = [
        ("grant_type", "password"),
        ("username", username),
        ("password", password),
    ];
    let client = reqwest::Client::new();
    let mut response = client
        .post("https://www.reddit.com/api/v1/access_token")
        .form(&params)
        .header(USER_AGENT, "ruddit 0.1.0 by serjor")
        .basic_auth(app_id, Some(app_token))
        .send()?;

    let value: RedditAuthJson = response.json()?;
    Ok((value.access_token, value.expires_in))
}

pub fn submit(
    access_token: &str,
    subreddit: String,
    url: String,
    title: String,
    text: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut post = HashMap::new();
    post.insert("kind", "link");
    post.insert("sr", &subreddit);
    post.insert("url", &url);
    post.insert("title", &title);
    post.insert("text", &text);

    let client = reqwest::Client::new();
    let _response = client
        .post("https://oauth.reddit.com/api/submit")
        .header(USER_AGENT, "ruddit 0.1.0 by serjor")
        .bearer_auth(access_token)
        .form(&post)
        .send()?;

    Ok(())
}
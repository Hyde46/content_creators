use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct RedditTextThread {
    pub author: String,
    pub post_title: String,
    pub post_text: String,
    pub ups: i32,
    pub responses: Option<Vec<RedditTextResponse>>,
    pub permalink: String,
    pub audio_path: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RedditTextResponse {
    pub author: String,
    pub response_text: String,
    pub ups: i32,
    pub permalink: String,
    pub audio_path: Option<String>,
}

//TODO: much later
#[derive(Serialize, Deserialize)]
struct RedditMediaThread {
    post_title: String,
    post_text: String,
    media: Option<String>,
    ups: usize,
    responses: Vec<RedditTextResponse>,
    permalink: String,
    speach_file_path: Option<String>,
}

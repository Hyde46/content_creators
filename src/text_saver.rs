use super::model::*;
use rsreddit::model::listing::Child;
use rsreddit::model::listing::Listing;

pub fn save_listings(posts: Listing, path: String) -> Result<(), String> {
    // Saves post title of listing as json file

    //Iterate through listings, ignore pinned posts
    let mut option_text_posts: Vec<RedditTextThread> = posts
        .data
        .children
        .iter()
        .filter(|post| !post.data.stickied)
        .map(|post| extract_post_from_listing(&post))
        .collect();
    println!("{:?}", option_text_posts);
    Ok(())
}

fn extract_post_from_listing(post: &Child) -> RedditTextThread {
    println!("{:?}", post);

    RedditTextThread {
        author: post.data.author.clone(),
        post_title: post.data.title.as_ref().unwrap().to_string(),
        post_text: String::from("post text here"),
        ups: post.data.ups,
        responses: None,
        permalink: post.data.permalink.clone(),
        audio_path: None,
    }
}

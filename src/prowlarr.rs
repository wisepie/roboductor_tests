use dotenv::dotenv;
use reqwest::{Client, Url};
use serde_json::Value;

//TODO: Change env to a parameter for easier testing in main. Clean term (remove non alphabetic and numeric
//symbols)
pub async fn search_prowlarr(term: &str) -> anyhow::Result<()> {
    dotenv().ok();

    const PROWLARR_URL: &str = "http://localhost:9696/api/v1/search";
    const CATEGORIES: &str = "[2000]";
    let api_key: String = std::env::var("APIKEY").expect("Error");

    let url = Url::parse_with_params(
        PROWLARR_URL,
        [
            ("apikey", api_key),
            ("query", term.to_string()),
            ("categories[]", CATEGORIES.to_string()),
        ],
    )?;

    let response = Client::new().get(url).send().await?;
    //TODO: Turn response into json item. Return get_best_torrent(torrents)
    Ok(())
}
//TODO: Points system where 1080p, freeleech, high seeders torrents get boosted. Return downloadUrl
pub async fn get_best_torrent(torrents: Value) {}

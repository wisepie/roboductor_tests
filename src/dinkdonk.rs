use std::os::linux::raw::stat;

use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
const URL: &str = "https://dinkdonk.mov/api/create";
const TMDB_SEARCH_URL: &str = "https://api.themoviedb.org/3/search/movie";

#[derive(Serialize)]
pub struct TMDbSearch {
    api_key: String,
    query: String,
    year: Option<String>,
}

#[derive(Serialize)]
pub struct Movie {
    title: String,
    release_date: Option<String>,
    tmdb_id: Option<u32>,
    imdb_id: Option<String>,
    runtime: Option<u16>,
    genres: Option<Vec<String>>,
    poster_path: Option<String>,
    vote_average: Option<f32>,
    vote_count: Option<u32>,
    last_watched: Option<String>,
}
impl Movie {
    pub fn make_generic_movie(title: String, tmdb_id: Option<u32>, runtime: Option<u16>) -> Self {
        Self {
            title,
            release_date: None,
            tmdb_id,
            imdb_id: None,
            runtime,
            genres: None,
            poster_path: None,
            vote_average: None,
            vote_count: None,
            last_watched: None,
        }
    }
    pub fn make_generic_list() -> CreateOption {
        //NOTE: use same movie for each test. 307124 - love on a leash
        let mut list = Vec::new();

        list.push(Movie::make_generic_movie(
            "Test".to_owned(),
            Some(307124),
            Some(86),
        ));
        list.push(Movie::make_generic_movie(
            "Test1".to_owned(),
            Some(307124),
            Some(86),
        ));
        list.push(Movie::make_generic_movie(
            "Test3".to_owned(),
            Some(307124),
            Some(86),
        ));
        CreateOption::MovieObjects(list)
    }
    //TODO: Finish this function. TMDb doesn't return optimized searches so something
    //to filter and find the closest match is needed. Use popularity, title match, and year if
    //possible. Should return Result<Movie>
    pub async fn search_tmdb(api_key: String, title: String, year: Option<String>) -> Result<Self> {
        let mut params = vec![("api_key", api_key), ("query", title)];
        if let Some(year) = year {
            params.push(("year", year));
        }

        let client = Client::new();
        let res = client
            .get(TMDB_SEARCH_URL)
            .query(&params)
            .send()
            .await
            .expect("TMDb Response Error");

        let status = res.status();
        let text = res.text().await.expect("Text error");

        println!("{}", status);
        println!("{}", text);
	// TODO: Finish implementation. Adding this so project builds.
	Err(anyhow!("Method stub."))
    }
}

pub enum CreateOption {
    StringOptions(Vec<String>),
    MovieObjects(Vec<Movie>),
}

#[derive(Serialize)]
pub struct Dinkdonk {
    title: String,
    options: String,
    #[serde(rename = "type")]
    poll_type: String,
    duration: u8,
    randomize: bool,
    broadcast: bool,
    #[serde(rename = "apiKey")]
    api_key: String,
}

impl Dinkdonk {
    pub async fn create_rating_poll(
        title: String,
        api_key: String,
        randomize: bool,
        broadcast: bool,
        duration: u8,
    ) -> Result<String> {
        let mut poll_options: Vec<String> = Vec::new();
        for i in 1..11 {
            let val: f32 = (i as f32) / 2.0;
            poll_options.push(format!("{}/5", val))
        }
        poll_options.push("Didn't Watch".to_owned());

        match Dinkdonk::create_poll(
            title,
            api_key,
            randomize,
            broadcast,
            duration,
            CreateOption::StringOptions(poll_options),
        )
        .await
        {
            Ok(id) => Ok(id),
            Err(_) => Err(anyhow!("")),
        }
    }
    pub async fn create_poll(
        title: String,
        api_key: String,
        randomize: bool,
        broadcast: bool,
        duration: u8,
        options: CreateOption,
    ) -> Result<String> {
        let options = match options {
            CreateOption::StringOptions(options) => options,
            CreateOption::MovieObjects(options) => {
                let mut opts = Vec::new();

                for opt in options.iter() {
                    opts.push(serde_json::to_string(opt).expect("Error"));
                }
                opts
            }
        };

        match serde_json::to_string(&options) {
            Ok(options) => {
                let payload = Dinkdonk {
                    title,
                    api_key,
                    randomize,
                    broadcast,
                    options,
                    poll_type: String::from("single_choice"),
                    duration: duration,
                };
                let client = Client::new();
                let res = client.post(URL).form(&payload).send().await?;

                let status = res.status();
                let text = res.text().await?;
                let data: Value = serde_json::from_str(&text)?;

                println!("Status: {}", status);
                println!("Text: {}", text);

                //TODO: Clean this json match up a bit. maybe make structs based on dinkdonk.mov
                //api
                match data
                    .get("data")
                    .and_then(|data| data.get("id"))
                    .and_then(|id| id.as_str())
                {
                    Some(id) => Ok(id.to_string()),
                    None => Err(anyhow!("Error Fetching Poll ID")),
                }
            }

            Err(e) => Err(anyhow!("{e}")),
        }
    }
    //TODO: Return maybe Vec<(votes, Movie)>
    pub async fn get_results(poll_id: String) {}
}

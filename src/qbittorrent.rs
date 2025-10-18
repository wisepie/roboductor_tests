use anyhow::{Result, anyhow};
use dotenv::{dotenv, var};
use qbit_rs::Qbit;
use qbit_rs::model::Credential;

pub struct QbitClient {
    client: Qbit,
}
//TODO: Change env variables to parameters for easier testing in main.
impl QbitClient {
    pub async fn connect() -> Result<Self> {
        dotenv().ok();

        let username = var("QBIT_USER").expect("Error");
        let password = var("QBIT_PASS").expect("Error");

        let credential = Credential::new(username, password);
        let client = Qbit::new("http://localhost:8080", credential);

        match client.get_version().await {
            Ok(_) => Ok(Self { client }),
            Err(e) => Err(anyhow!("{e}")),
        }
    }
    pub async fn get_torrents(&self) -> Result<()> {
        let args = qbit_rs::model::GetTorrentListArg {
            filter: None,
            category: None,
            tag: None,
            sort: None,
            reverse: None,
            limit: None,
            offset: None,
            hashes: None,
        };

        match self.client.get_torrent_list(args).await {
            Ok(list) => {
                let list = serde_json::to_value(&list)?;
                println!("{}", list);
                Ok(())
            }
            Err(e) => Err(anyhow!("{e}")),
        }
    }
    //TODO: Qbit disconnects clients after an hour by default. Return Result.
    pub async fn reconnect(&self) {}

    //TODO: Either change search term to serde_json::Value or call prowlarr::search_prowlarr
    //inside. Important to search for preexisting torrents first. Return video path when download
    //is completed.
    pub async fn download_torrent(&self, search_term: String) {}

    //TODO: Attempt ~10 searches in 10 seconds. Return Result.
    pub async fn get_hash(&self, title: String, movie_path: String) {}

    //TODO: Search for video formats in torrent and pick the longest one. Return String video
    //path
    pub async fn get_main_video_path(&self, info_hash: String, movie_path: String) {}
}

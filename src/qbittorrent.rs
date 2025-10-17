use anyhow::{Result, anyhow};
use dotenv::{dotenv, var};
use qbit_rs::Qbit;
use qbit_rs::model::Credential;

pub struct QbitClient {
    client: Qbit,
}
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
}

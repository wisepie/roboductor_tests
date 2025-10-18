use anyhow::{Result, anyhow};
use obws::{Client, responses::scenes::Scenes};
use std::{
    process::{Command, Stdio},
    time::Duration,
};
use tokio::time::sleep;

//TODO: Add videoplayer and waiting scene fields and a type default ("no subs, subs, dub")
pub struct ObsClient {
    obs: Client,
    port: u16,
    password: Option<String>,
}
impl ObsClient {
    pub async fn connect(port: u16, password: Option<String>) -> Result<Self> {
        match Client::connect("localhost", port, password.as_deref()).await {
            Ok(obs) => Ok(Self {
                obs,
                port,
                password,
            }),
            Err(_) => {
                println!("Failed to connect to OBS websocket, attempting to launch obs instance");
                let mut args = vec![format!("--websocket_port={}", port)];

                if let Some(pass) = &password {
                    args.push(format!("--websocket_password={}", pass));
                }
                match Command::new("obs")
                    .args(args.as_slice())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                {
                    Ok(_) => {
                        sleep(Duration::from_secs(5)).await;
                        match Client::connect("localhost", port, password.as_deref()).await {
                            Ok(obs) => Ok(Self {
                                obs,
                                port,
                                password,
                            }),
                            Err(e) => Err(anyhow!("{e}")),
                        }
                    }
                    Err(e) => Err(anyhow!("{e}")),
                }
            }
        }
    }
    pub async fn connect_default() -> Result<Self> {
        let port: u16 = 4455;
        let password: Option<String> = None;
        match ObsClient::connect(port, password.clone()).await {
            Ok(obs) => Ok(obs),
            Err(e) => Err(anyhow!("{e}")),
        }
    }
    pub async fn switch_scene(&self, scene_name: &str) -> Result<()> {
        self.obs
            .scenes()
            .set_current_program_scene(scene_name)
            .await?;
        Ok(())
    }
    pub async fn start_streaming(&self) -> Result<()> {
        match self.obs.streaming().start().await {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow!("{e}")),
        }
    }

    pub async fn stop_streaming(&self) -> Result<()> {
        match self.obs.streaming().stop().await {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow!("{e}")),
        }
    }

    pub async fn get_scenes(&self) -> Result<Scenes> {
        match self.obs.scenes().list().await {
            Ok(scenes) => Ok(scenes),
            Err(e) => Err(anyhow!("{e}")),
        }
    }
}

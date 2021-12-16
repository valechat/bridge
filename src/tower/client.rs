use reqwest::Client;
use crate::{GetUserResponseData, RegisterBeaconRequestData, RegisterBeaconResponseData};

pub struct TowerClient {
    client: Client,
    url: String
}

impl TowerClient {
    pub fn new(url: String, client: Option<Client>) -> Self {
        Self {
            url,
            client: client.unwrap_or(Client::new())
        }
    }

    pub fn get_endpoint(&self, path: &str) -> String {
        format!("{}{}", self.url, path)
    }

    pub async fn register_beacon(&self, data: RegisterBeaconRequestData) -> Result<RegisterBeaconResponseData, reqwest::Error> {
        self.client.post(self.get_endpoint("/beacons"))
            .json(&data)
            .send()
            .await?
            .json::<RegisterBeaconResponseData>()
            .await
    }

    pub async fn get_users(&self) -> Result<Vec<GetUserResponseData>, reqwest::Error> {
        self.client.get(self.get_endpoint("/users"))
            .send()
            .await?
            .json::<Vec<GetUserResponseData>>()
            .await
    }

    pub async fn get_user(&self, user_id: i64) -> Result<GetUserResponseData, reqwest::Error> {
        self.client.get(self.get_endpoint(&format!("/users/{}", user_id)))
            .send()
            .await?
            .json::<GetUserResponseData>()
            .await
    }
}
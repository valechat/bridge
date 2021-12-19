use reqwest::Client;
use crate::{RegisterBeaconCallbackRequestData, RegisterBeaconCallbackResponseData};

pub struct BeaconClient {
    client: Client,
    url: String
}

impl BeaconClient {
    pub fn new(url: String, client: Option<Client>) -> Self {
        Self {
            url,
            client: client.unwrap_or_default()
        }
    }

    pub fn get_endpoint(&self, path: &str) -> String {
        format!("{}{}", self.url, path)
    }

    pub async fn register_beacon_callback(&self, data: RegisterBeaconCallbackRequestData) -> Result<RegisterBeaconCallbackResponseData, reqwest::Error> {
        self.client.post(self.get_endpoint("/beacons"))
            .json(&data)
            .send()
            .await?
            .json::<RegisterBeaconCallbackResponseData>()
            .await
    }
}
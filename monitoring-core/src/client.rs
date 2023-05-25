use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::{ErrorLog, models::SystemInformation};

pub struct SysInfoClient {
    config: ClientConfig,
}

#[derive(Debug)]
pub enum ClientError {
    Reqwest(reqwest::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientConfig {
    pub api_key: String,
    pub profile_key: String,
    pub profile_id: u32,
    pub server_url: String,
}

impl SysInfoClient {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    pub async fn get_version(&self) -> Result<(String, StatusCode), ClientError> {
        let resp = reqwest::Client::new()
            .get(format!("{}/", self.config.server_url))
            .header("x-api-key", self.config.api_key.clone())
            .header("x-profile-key", self.config.profile_key.clone())
            .send()
            .await
            .map_err(|x| ClientError::Reqwest(x))?;

        let status = resp.status().clone();
        let version = resp.text().await.map_err(|x| ClientError::Reqwest(x))?;

        Ok((version, status))
    }

    pub async fn post_sys_info(&self, data: SystemInformation) -> Result<StatusCode, ClientError> {
        self.post("system-info", data).await
    }

    pub async fn post_error_log(&self, data: ErrorLog) -> Result<StatusCode, ClientError> {
        self.post("error", data).await
    }

    async fn post<T: Serialize>(&self, sub_path: &str, data: T) -> Result<StatusCode, ClientError> {
        let resp = reqwest::Client::new()
            .post(format!(
                "{}/{}/{}",
                self.config.server_url, sub_path, self.config.profile_id
            ))
            .header("x-api-key", self.config.api_key.clone())
            .header("x-profile-key", self.config.profile_key.clone())
            .json(&data)
            .send()
            .await
            .map_err(|x| ClientError::Reqwest(x))?;

        Ok(resp.status())
    }
}
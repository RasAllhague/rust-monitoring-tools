use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{api::models::InsertDeviceProfile, models::SystemInformation, ErrorLog};

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
    pub server_url: String,
}

impl ClientConfig {
    pub fn new(api_key: &str, server_url: &str) -> Self {
        Self {
            api_key: String::from(api_key),
            server_url: String::from(server_url),
        }
    }
}

impl SysInfoClient {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    pub async fn get_version(&self) -> Result<(String, StatusCode), ClientError> {
        let resp = reqwest::Client::new()
            .get(format!("{}/", self.config.server_url))
            .header("x-api-key", self.config.api_key.clone())
            .send()
            .await
            .map_err(|x| ClientError::Reqwest(x))?;

        let status = resp.status().clone();
        let version = resp.text().await.map_err(|x| ClientError::Reqwest(x))?;

        Ok((version, status))
    }

    pub async fn get_profiles(&self, read_key: &str) -> Result<(String, StatusCode), ClientError> {
        let resp = reqwest::Client::new()
            .get(format!("{}/", self.config.server_url))
            .header("x-api-key", self.config.api_key.clone())
            .header("x-read-key", read_key)
            .send()
            .await
            .map_err(|x| ClientError::Reqwest(x))?;

        let status = resp.status().clone();
        let version = resp.text().await.map_err(|x| ClientError::Reqwest(x))?;

        Ok((version, status))
    }

    pub async fn post_profile(
        &self,
        data: InsertDeviceProfile,
    ) -> Result<StatusCode, ClientError> {
        let resp = reqwest::Client::new()
            .post(format!("{}/profiles", self.config.server_url))
            .header("x-api-key", self.config.api_key.clone())
            .json(&data)
            .send()
            .await
            .map_err(|x| ClientError::Reqwest(x))?;

        Ok(resp.status())
    }

    pub async fn post_sys_info(
        &self,
        profile_id: i32,
        profile_key: &str,
        data: SystemInformation,
    ) -> Result<StatusCode, ClientError> {
        self.post_with_profile_key(profile_id, profile_key, "system-info", data)
            .await
    }

    pub async fn post_error_log(
        &self,
        profile_id: i32,
        profile_key: &str,
        data: ErrorLog,
    ) -> Result<StatusCode, ClientError> {
        self.post_with_profile_key(profile_id, profile_key, "error", data)
            .await
    }

    async fn post_with_profile_key<T: Serialize>(
        &self,
        profile_id: i32,
        profile_key: &str,
        sub_path: &str,
        data: T,
    ) -> Result<StatusCode, ClientError> {
        let resp = reqwest::Client::new()
            .post(format!(
                "{}/{}/{}",
                self.config.server_url, sub_path, profile_id
            ))
            .header("x-api-key", self.config.api_key.clone())
            .header("x-profile-key", profile_key)
            .json(&data)
            .send()
            .await
            .map_err(|x| ClientError::Reqwest(x))?;

        Ok(resp.status())
    }
}

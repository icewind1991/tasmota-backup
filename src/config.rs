use anyhow::{Context, Result};
use secretfile::load;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use tasmota_mqtt_client::TasmotaClient;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub device: DeviceConfig,
    pub output: OutputConfig,
    #[serde(default = "default_discovery_time")]
    pub discovery_time: u64,
}

fn default_discovery_time() -> u64 {
    1
}

impl Config {
    pub fn load(path: &Path) -> Result<Config> {
        let raw = read_to_string(path)
            .with_context(|| format!("Failed to load config from {}", path.display()))?;
        toml::from_str(&raw)
            .with_context(|| format!("Failed to parse config file {} as toml", path.display()))
    }
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    pub target: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub hostname: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub username: Option<String>,
    #[serde(flatten)]
    pub password: Option<PasswordConfig>,
}

fn default_port() -> u16 {
    1883
}

impl MqttConfig {
    pub async fn connect(&self) -> Result<TasmotaClient> {
        let password = self
            .password
            .as_ref()
            .map(|password| password.get())
            .transpose()?;

        Ok(TasmotaClient::connect(
            &self.hostname,
            self.port,
            self.username.as_deref().zip(password.as_deref()),
        )
        .await?)
    }
}

#[derive(Debug, Deserialize)]
pub struct DeviceConfig {
    #[serde(flatten)]
    pub password: PasswordConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PasswordConfig {
    Raw {
        password: String,
    },
    File {
        #[serde(rename = "password-file")]
        password_file: String,
    },
}

impl PasswordConfig {
    /// Get the token either directly from the config or through the token file
    pub fn get(&self) -> Result<String> {
        match self {
            PasswordConfig::Raw { password } => Ok(password.clone()),
            PasswordConfig::File { password_file } => Ok(load(password_file)?),
        }
    }
}

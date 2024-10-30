use crate::config::Config;
use anyhow::{Context, Result};
use clap::Parser;
use hex_fmt::HexFmt;
use md5::{Digest, Md5};
use std::fs::{write, File};
use std::io::copy;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tasmota_mqtt_client::TasmotaClient;
use tokio::time::{sleep, timeout};
use tracing::{error, info};

mod config;

#[derive(Debug, Parser)]
struct Args {
    config: PathBuf,
    device: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let config = Config::load(&args.config)?;
    let device_password = config.device.password.get()?;

    let client = config.mqtt.connect().await?;

    info!("waiting for device discovery");

    // wait for discovery messages from mqtt
    sleep(Duration::from_secs(config.discovery_time)).await;

    let devices = client.current_devices();
    info!("found {} devices", devices.len());
    for device in devices {
        if args.device.is_none() || args.device.as_deref() == Some(device.as_str()) {
            let result = timeout(
                Duration::from_secs(120),
                download(&client, &device, &config.output.target, &device_password),
            )
            .await;
            let result = result
                .with_context(|| format!("Timeout while downloading config for {device}"))
                .and_then(|res| res);
            if let Err(e) = result {
                let error = format!("{e:#}");
                error!(device, error, "Failed to download config for {device}");
            }
        }
    }

    Ok(())
}

async fn download(
    client: &TasmotaClient,
    device: &str,
    target_dir: &Path,
    device_password: &str,
) -> Result<()> {
    let file = client.download_config(device, device_password).await?;
    let target_path = target_dir.join(&file.name);
    let existing_hash = target_path
        .exists()
        .then(|| {
            hash_file(&target_path).with_context(|| {
                format!(
                    "failed to calculate checksum of existing file {}",
                    target_path.display()
                )
            })
        })
        .transpose()?;
    if existing_hash != Some(file.md5) {
        write(&target_path, file.data.as_ref()).with_context(|| {
            format!("failed save downloaded config to {}", target_path.display())
        })?;
        info!(device = device, file = file.name, hash = %HexFmt(file.md5), "device config saved")
    } else {
        info!(device = device, file = file.name, hash = %HexFmt(file.md5), "device config unchanged")
    }
    Ok(())
}

#[tracing::instrument]
fn hash_file(path: &Path) -> Result<[u8; 16]> {
    let mut file = File::open(path)?;
    let mut hasher = Md5::new();
    copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize().into())
}

use anyhow::{anyhow, Result};
use std::env;

#[tokio::main]
pub async fn main() -> Result<()> {
    if let Err(err) = env::var("RUST_LOG") {
        match err {
            env::VarError::NotPresent => {
                env::set_var("RUST_LOG", "info");
            }
            env::VarError::NotUnicode(_) => {
                return Err(anyhow!(
                    "The value of 'RUST_LOG' does not contain valid unicode \
                     data."
                ));
            }
        }
    }
    env_logger::init();
    let config_path = "config.toml";
    let config = klick_backend::Config::from_file(config_path)
        .map_err(|err| {
            log::info!(
                "Could not read config from {config_path} ({err}): use default config instead."
            )
        })
        .unwrap_or_default();
    log::debug!("Run with this config: {config:#?}");
    klick_backend::run(&config).await
}

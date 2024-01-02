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
    let addr = "0.0.0.0:3000".parse()?;
    klick_backend::run(addr).await
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let addr = "0.0.0.0:3000".parse()?;
    klick_backend::run(addr).await
}

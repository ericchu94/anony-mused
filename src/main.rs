use std::env;

use anyhow::Result;

mod anony_muse_client;
mod http;

fn port() -> Result<u16> {
    Ok(env::var("PORT")?.parse()?)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    http::run(port().unwrap_or(8080)).await?;

    Ok(())
}

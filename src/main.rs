use anyhow::Result;
use dotenv::dotenv;

mod converter;
mod error;
mod files;
mod helper;
mod job;
mod queue;
mod server;

use crate::server::server;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();

    server().await
}

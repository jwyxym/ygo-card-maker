use fantoccini::{
    Client, Locator
};
use std::{
    thread::sleep,
    time::Duration
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client: Client = Client::new("http://localhost:9515").await?;
    
    client.goto("http://localhost:5173/").await?;
    loop {
        if client.find(Locator::Css(".fixed")).await.is_ok() {
            break;
        }
        sleep(Duration::from_secs(1));
    }
    client.close().await?;
    Ok(())
}

mod jobkorea;
mod posting;
mod saramin;

use reqwest;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mstdn = posting::mastodon::Mastodon::new();
    let http_client = reqwest::Client::new();
    let mut latest_saramin_id = saramin::init(&http_client).await?;

    loop {
        println!("[log] latest_saramin_id={}", latest_saramin_id);
        sleep(Duration::from_millis(30_000)).await;

        latest_saramin_id = saramin::cycle(latest_saramin_id, &http_client, &mstdn).await?;
    }
}

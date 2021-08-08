mod jobkorea;
mod posting;
mod saramin;

use reqwest;
use std::env;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mstdn_enable = env::var("MASTODON_ENABLE").is_ok();
    let mstdn = posting::mastodon::Mastodon::new(
        env::var("MASTODON_URL")?,
        env::var("MASTODON_BEARER_TOKEN")?,
    );
    let http_client = reqwest::Client::new();
    let mut latest_saramin_id = saramin::init(&http_client).await?;

    loop {
        println!("[log] latest_saramin_id={}", latest_saramin_id);
        sleep(Duration::from_millis(120_000)).await;

        latest_saramin_id = saramin::cycle(latest_saramin_id, &http_client, &mstdn).await?;
    }
}

use tokio::time::{sleep, Duration};

pub struct Mastodon {
    instance_url: String,
    bearer_token: String,
}

impl Mastodon {
    pub fn new(instance_url: String, bearer_token: String) -> Mastodon {
        Mastodon {
            instance_url,
            bearer_token,
        }
    }
}

impl Mastodon {
    pub async fn posting<T>(
        &self, http_client: &reqwest::Client, msg: &T,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: ToString,
    {
        sleep(Duration::from_millis(5_000)).await;

        let status = [
            ("status", msg.to_string()),
            ("visibility", "unlisted".to_string()),
        ];

        http_client
            .post(format!("https://{}/api/v1/statuses", &self.instance_url))
            .bearer_auth(&self.bearer_token)
            .form(&status)
            .send()
            .await?;

        Ok(())
    }
}

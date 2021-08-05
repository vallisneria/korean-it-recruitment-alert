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

pub trait MastodonUpload {
    fn title(&self) -> String;
    fn statuses(&self) -> String;
}

impl Mastodon {
    pub async fn posting<T>(
        &self, http_client: &reqwest::Client, msg: &T,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: MastodonUpload,
    {
        sleep(Duration::from_millis(5_000)).await;

        let status = [
            ("spoiler_text", msg.title()),
            ("status", msg.statuses()),
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

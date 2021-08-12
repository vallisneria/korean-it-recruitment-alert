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
    fn spoiler_text(&self) -> String;
    fn status(&self) -> String;
}

impl Mastodon {
    pub async fn posting<T>(
        &self, http_client: &reqwest::Client, msg: &T,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: MastodonUpload,
    {
        println!("[log] UPLOAD MASTODON {}", msg.spoiler_text());

        let status = [
            ("spoiler_text", msg.spoiler_text()),
            ("status", msg.status()),
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

use reqwest;
use select::document::Document;

pub async fn fetch(http_client: &reqwest::Client) -> Result<Document, Box<dyn std::error::Error>> {
    const SARAMIN_URL: &str =
        "https://www.saramin.co.kr/zf_user/jobs/list/job-category?cat_bcd=4&sort=MD";
    let response = http_client.get(SARAMIN_URL).send().await?.text().await?;
    let document = Document::from_read(response.as_bytes())?;

    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn it_runs() {
        use reqwest::Client;

        let client = Client::new();
        fetch(&client).await.unwrap();
    }
}

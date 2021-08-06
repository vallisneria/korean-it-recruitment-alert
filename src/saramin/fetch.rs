use reqwest;
use select::document::Document;

pub async fn fetch(http_client: &reqwest::Client) -> Result<Document, Box<dyn std::error::Error>> {
    // 웹개발전체, 응용프로그램개발전체, 시스템개발전체, 서버네트워크보안전체, 게임일부
    // 정규직, 병역특례, 인턴직
    const SARAMIN_URL: &str = concat!(
        "https://www.saramin.co.kr/zf_user/jobs/list/job-category",
        "?cat_cd=404%2C407%2C408%2C402",
        "&cat_key=40511%2C40503%2C40530%2C40537%2C40536%2C40527",
        "&job_type=1%2C3%2C4",
        "&sort=RD"
    );

    println!("[log] REQUEST {}", &SARAMIN_URL);
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

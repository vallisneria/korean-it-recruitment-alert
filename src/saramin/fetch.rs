use reqwest;
use select::document::Document;

pub async fn fetch() -> Result<Document, Box<dyn std::error::Error>> {
    const SARAMIN_URL: &str = "https://www.saramin.co.kr/zf_user/jobs/list/job-category?cat_bcd=4&sort=MD";
    let response = reqwest::get(SARAMIN_URL).await?;

    Document::from_read(response)?
}

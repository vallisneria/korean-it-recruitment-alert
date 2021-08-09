use reqwest::Client;
use select::document::Document;
use std::error::Error;

pub async fn fetch(http_client: &Client) -> Result<Document, Box<dyn Error>> {
    const URL: &str = "https://www.jobkorea.co.kr/Recruit/Home/_GI_List/";
    #[rustfmt::skip]
    let body = [
        ("isDefault", "true"),
        ("condition[duty]", "1000100,1000101,1000102,1000109,1000094,1000097",),
        ("condition[jobtype]", "1,3,9"),
        ("condition[menucode]", ""),
        ("page", "1"),
        ("direct", "0"),
        ("order", "2"),
        ("pagesize", "20"),
        ("tabindex", "0"),
        ("fulltime", "0"),
        ("confirm", "0"),
    ];

    #[rustfmt::skip]
    let res = http_client
        .post(URL)
        .header("Accept", "text/html, */*; q=0.01")
        .header("Referer", "https://www.jobkorea.co.kr/recruit/joblist?menucode=duty&dutyCtgr=10016",)
        .header("X-Requested-With", "XMLHttpRequest")
        .header("DNT", "1")
        .form(&body)
        .send()
        .await?;

    let document = Document::from_read(res.text().await?.as_bytes())?;
    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn fetch_test() {
        let http_client = Client::new();
        fetch(&http_client).await.unwrap();
    }
}

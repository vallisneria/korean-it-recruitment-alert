use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use select::document::Document;
use std::error::Error;

pub async fn fetch(http_client: &Client) -> Result<Document, Box<dyn Error>> {
    const COOKIE: &str = r#"CookieNo=1284391680; PCID=16201107319712787862745; mainContents=0; MainLayer=done; MainRcntlyData=%3c%6c%69%3e%3c%61%20%68%72%65%66%3d%22%2f%72%65%63%72%75%69%74%2f%6a%6f%62%6c%69%73%74%3f%6d%65%6e%75%63%6f%64%65%3d%64%75%74%79%22%3e%c1%f7%b9%ab%ba%b0%3c%2f%61%3e%20%26%67%74%3b%20%3c%61%20%68%72%65%66%3d%22%2f%72%65%63%72%75%69%74%2f%6a%6f%62%6c%69%73%74%3f%6d%65%6e%75%63%6f%64%65%3d%64%75%74%79%26%64%75%74%79%43%74%67%72%3d%31%30%30%31%36%22%20%63%6c%61%73%73%3d%22%63%61%74%65%22%3e%49%54%a1%a4%c0%ce%c5%cd…4,1000109","Employ_Type":"1,2,3,9","Reg_Dt":"2021-08-08T17:28:15.383876+09:00","IsKeep":true},{"CookieIndex":"20210504161249","Cndt_No":0,"M_Id":"","Employ_Type":"9","Reg_Dt":"2021-05-04T16:12:49.899369+09:00","IsKeep":true}]; DirectStat=ON; ASP.NET_SessionId=lkvttvjp113mcxwwx20dq5ku; jobkorea=Site_Oem_Code=C1; ECHO_SESSION=5301628411212157; Main_Top_Banner_Seq=1; rstn_item=%7B%22duty%22%3A%7B%22item1%22%3A%20%7B%22code%22%3A%2210016%22%2C%22name%22%3A%22IT%B7%uC778%uD130%uB137%22%2C%22sort%22%3A1%7D%7D%7D"#;
    const URL: &str = "https://www.jobkorea.co.kr/Recruit/Home/_GI_List/";

    http_client
        .post(URL)
        .header("Cookie", COOKIE)
        .send()
        .await?
        .text()
        .await?;

    let document = Document::from_read(String::new().as_bytes())?;
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

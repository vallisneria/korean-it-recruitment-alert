pub mod data_extraction;
pub mod fetch;
pub mod jobkorea;

use crate::posting::mastodon::Mastodon;
use std::error::Error;

/// 함수를 호출한 시점에서 가장 최근의 채용 공고 id를 가져온다.
pub async fn init(http_client: &reqwest::Client) -> Result<u32, Box<dyn Error>> {
    let jobkorea_document = fetch::fetch(&http_client).await?;
    let data = data_extraction::data_extract(&jobkorea_document);

    Ok(data[0].id)
}

/// 채용 공고를 가져오고 업로드한다.
/// 가장 최근의 채용 공고 id를 반환한다.
pub async fn cycle(
    latest_id: u32, http_client: &reqwest::Client, mstdn: &Mastodon,
) -> Result<u32, Box<dyn Error>> {
    // 웹 사이트에서 목록을 가져옴
    let jobkorea_document = fetch::fetch(&http_client).await?;
    let data = data_extraction::data_extract(&jobkorea_document);
    let fetch_latest_id = data[0].id;

    for i in data.iter() {
        if i.id > latest_id {
            println!("[{}] {}", i.id, i.title);
            mstdn.posting(&http_client, i).await?;
        } else {
            break;
        }
    }

    if fetch_latest_id > latest_id {
        Ok(fetch_latest_id)
    } else {
        Ok(latest_id)
    }
}

pub mod data_extraction;
pub mod fetch;
pub mod saramin;

use super::posting::mastodon::Mastodon;
use std::error::Error;

/// 함수를 호출한 시점에서 가장 최근의 채용 공고 id를 가져온다.
pub async fn init(http_client: &reqwest::Client) -> Result<u32, Box<dyn Error>> {
    let saramin_document = fetch::fetch(&http_client).await?;
    let data = data_extraction::data_extract(&saramin_document);

    Ok(data[0].id)
}

/// 채용 공고를 가져오고 업로드함
/// 가장 최근의 채용 공고 id를 반환한다.
pub async fn cycle(
    latest_id: u32, http_client: &reqwest::Client, mstdn: &Option<Mastodon>,
) -> Result<u32, Box<dyn Error>> {
    // 웹 사이트에서 목록을 가져옴
    let saramin_document = fetch::fetch(&http_client).await?;
    let data = data_extraction::data_extract(&saramin_document);
    let fetch_latest_id = data[0].id;

    // 가져온 공고를 처리함
    for i in data.iter() {
        if i.id > latest_id {
            println!("[saramin/{}] {}", i.id, i.title);

            match mstdn {
                Some(m) => {
                    m.posting(&http_client, i).await?;
                }
                None => {}
            }
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

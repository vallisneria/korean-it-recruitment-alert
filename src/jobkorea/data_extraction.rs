use super::jobkorea::JobKorea;
use scraper::{ElementRef, Html, Selector};
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};

pub fn data_extract(document: &Html) -> Vec<JobKorea> {
    let mut result: Vec<JobKorea> = Vec::new();
    let selector = Selector::parse("tr.devloopArea").unwrap();

    for element in document.select(&selector) {
        let data = JobKorea::new(
            get_id(element),
            get_company_name(element),
            get_title(element),
            get_info(element),
            get_deadline(element),
            get_link(element),
        );

        result.push(data);
    }

    result
}

fn get_id(element: ElementRef) -> u32 {
    let selector = Selector::parse("button.lstBtn").unwrap();
    element
        .select(&selector)
        .next()
        .unwrap()
        .value()
        .attr("data-gno")
        .unwrap()
        .parse()
        .unwrap()
}

fn get_company_name(element: ElementRef) -> String {
    let selector = Selector::parse(".tplCo a").unwrap();
    element
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn get_title(element: ElementRef) -> String {
    let selector = Selector::parse(".tplTit a").unwrap();
    element
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn get_info(element: ElementRef) -> Vec<String> {
    let selector = Selector::parse(".cell").unwrap();
    element
        .select(&selector)
        .map(|x| x.text().next().unwrap_or("").parse().unwrap())
        .collect()
}

fn get_deadline(element: ElementRef) -> String {
    let selector = Selector::parse(".odd .date").unwrap();
    element
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn get_link(element: ElementRef) -> String {
    let selector = Selector::parse(".tplTit a").unwrap();
    let link = element
        .select(&selector)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    format!("https://www.jobkorea.co.kr{}", link)
}

#[cfg(test)]
mod tests {
    use super::super::fetch::fetch;
    use super::*;
    use reqwest::Client;
    use scraper::Html;

    #[test]
    fn data_extract_test() {
        let html = include_str!("jobkorea_job_list_for_test.html");
        let doc = Html::parse_document(html);
        let data = data_extract(&doc);

        assert_ne!(data.len(), 0);
    }

    #[tokio::test]
    async fn fetch_and_extract() {
        let http_client = Client::new();
        let doc = fetch(&http_client).await.unwrap();
        let data = data_extract(&doc);

        assert_ne!(data.len(), 0);
    }
}

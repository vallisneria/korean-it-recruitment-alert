use super::saramin::Saramin;
use scraper::{ElementRef, Html, Selector};

pub fn data_extract(document: &Html) -> Vec<Saramin> {
    let mut result: Vec<Saramin> = Vec::new();
    let item_selector = Selector::parse(".common_recruilt_list .list_item").unwrap();

    for element in document.select(&item_selector) {
        let data = Saramin::new(
            get_id(element),
            get_title(element),
            get_company_name(element),
            get_career(element),
            get_education(element),
            get_employment_type(element),
            get_work_place(element),
            get_salary(element),
            get_deadline(element),
            get_link(element),
        );

        result.push(data);
    }

    result
}

fn get_id(element: ElementRef) -> u32 {
    let selector = Selector::parse(".idx_chk").unwrap();
    element
        .select(&selector)
        .next()
        .unwrap()
        .value()
        .attr("value")
        .unwrap()
        .parse()
        .unwrap()
}

fn get_company_name(element: ElementRef) -> String {
    let selector = Selector::parse(".company_nm a").unwrap();
    element
        .select(&selector)
        .next()
        .unwrap()
        .value()
        .attr("title")
        .unwrap()
        .parse()
        .unwrap()
}

fn get_title(element: ElementRef) -> String {
    let selector = Selector::parse(".notification_info a").unwrap();
    element
        .select(&selector)
        .next()
        .unwrap()
        .value()
        .attr("title")
        .unwrap()
        .parse()
        .unwrap()
}

fn get_career(element: ElementRef) -> Option<String> {
    let selector = Selector::parse(".career").unwrap();
    match element.select(&selector).next() {
        Some(e) => Some(e.text().next().unwrap().parse().unwrap()),
        None => None,
    }
}

fn get_education(element: ElementRef) -> Option<String> {
    let selector = Selector::parse(".education").unwrap();
    match element.select(&selector).next() {
        Some(e) => Some(e.text().next().unwrap().parse().unwrap()),
        None => None,
    }
}

fn get_employment_type(element: ElementRef) -> Option<String> {
    let selector = Selector::parse(".employment_type").unwrap();
    match element.select(&selector).next() {
        Some(e) => Some(e.text().next().unwrap().parse().unwrap()),
        None => None,
    }
}

fn get_work_place(element: ElementRef) -> Option<String> {
    let selector = Selector::parse(".work_place").unwrap();
    match element.select(&selector).next() {
        Some(e) => Some(e.text().next().unwrap().parse().unwrap()),
        None => None,
    }
}

fn get_salary(element: ElementRef) -> Option<String> {
    let selector = Selector::parse(".salary").unwrap();
    match element.select(&selector).next() {
        Some(e) => Some(e.text().next().unwrap().parse().unwrap()),
        None => None,
    }
}

fn get_deadline(element: ElementRef) -> Option<String> {
    let selector = Selector::parse(".deadlines").unwrap();
    match element.select(&selector).next() {
        Some(e) => Some(e.text().collect::<Vec<_>>()[0].parse().unwrap()),
        None => None,
    }
}

fn get_link(element: ElementRef) -> String {
    let selector = Selector::parse(".notification_info a").unwrap();
    let path = element
        .select(&selector)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    format!("https://saramin.co.kr{}", path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_and_extract() {
        use super::super::fetch::fetch;
        use reqwest::Client;

        let http_client = Client::new();
        let doc = fetch(&http_client).await.unwrap();
        let data = data_extract(&doc);

        assert_ne!(data.len(), 0);
    }

    #[test]
    fn data_extract_test() {
        use scraper::Html;
        let html = include_str!("saramin_job_list_for_test.html");
        let doc = Html::parse_document(html);
        let data = data_extract(&doc);

        assert_ne!(data.len(), 0);
    }
}

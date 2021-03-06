use super::jobkorea::JobKorea;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

pub fn data_extract(document: &Html) -> Vec<JobKorea> {
    let mut result: Vec<JobKorea> = Vec::new();
    let selector = Selector::parse("tr.devloopArea").unwrap();

    for element in document.select(&selector) {
        let info = get_info(element);
        let data = JobKorea::new(
            get_id(element),
            get_company_name(element),
            get_title(element),
            get_career(&info),
            get_education(&info),
            get_work_place(&info),
            get_employment_type(&info),
            get_salary(&info),
            get_position(&info),
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
        .trim()
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
        .trim()
        .parse()
        .unwrap()
}

fn get_info(element: ElementRef) -> Vec<&str> {
    let selector = Selector::parse(".cell").unwrap();
    element
        .select(&selector)
        .map(|x| x.text().next().unwrap_or(""))
        .collect()
}

fn _reg_filter(reg: Regex, info: &Vec<&str>) -> Vec<String> {
    info.iter()
        .filter(|x| reg.is_match(x))
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>()
}

fn get_career(info: &Vec<&str>) -> Option<String> {
    let career_reg = Regex::new(r"(??????\S*|??????\S*)").unwrap();
    let filter = _reg_filter(career_reg, info);

    match filter.len() {
        1 => Some(filter[0].clone()),
        _ => None,
    }
}

fn get_education(info: &Vec<&str>) -> Option<String> {
    let edu_reg = Regex::new(r"(????????????|(???|???????)???|[??????]???)\S*").unwrap();
    let filter = _reg_filter(edu_reg, info);

    match filter.len() {
        1 => Some(filter[0].clone()),
        _ => None,
    }
}

fn get_work_place(info: &Vec<&str>) -> Option<String> {
    let place_reg = Regex::new(
        r"(??????|??????|??????|??????|??????|??????|??????|??????|??????|??????|???[??????]|???[??????]|???[??????]|??????|??????|??????|?????????|??????|??????|[??????]????????????|???????????????|????????????|??????).*",
    ).unwrap();
    let filter = _reg_filter(place_reg, info);

    match filter.len() {
        1 => Some(filter[0].clone()),
        _ => None,
    }
}

fn get_employment_type(info: &Vec<&str>) -> Option<String> {
    let employment_reg = Regex::new(r"((??????|??????|??????|??????)???|(??????|??????)???|??????|??????|????????????|???????????????|????????????|???????????????).*").unwrap();
    let filter = _reg_filter(employment_reg, info);

    match filter.len() {
        1 => Some(filter[0].clone()),
        _ => None,
    }
}

fn get_salary(info: &Vec<&str>) -> Option<String> {
    let salary_reg = Regex::new(r"^[0-9,~]+???????.*").unwrap();
    let filter = _reg_filter(salary_reg, info);

    match filter.len() {
        1 => Some(filter[0].clone()),
        _ => None,
    }
}

fn get_position(info: &Vec<&str>) -> Option<String> {
    let position_reg = Regex::new(r"(??????|??????|??????|??????|??????|??????|??????|CEO).*").unwrap();
    let filter = _reg_filter(position_reg, info);

    match filter.len() {
        1 => Some(filter[0].clone()),
        _ => None,
    }
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

    #[test]
    fn amp() {
        let html = r#"
        <!DOCTYPE HTML>
        <body>
            <p>&amp;</p>
        </body>
        "#;
        let doc = Html::parse_document(html);
        let sel = Selector::parse("p").unwrap();

        let amp: &str = doc.select(&sel).next().unwrap().text().next().unwrap();
        assert_eq!(amp, "&");
    }
}

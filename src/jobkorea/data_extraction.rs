use super::jobkorea::JobKorea;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};

pub fn data_extract(document: &Document) -> Vec<JobKorea> {
    let mut result: Vec<JobKorea> = Vec::new();
    let nodes = document.find(Name("tr").and(Class("devLoopArea")));

    for node in nodes {
        let data = JobKorea::new(
            get_id(node),
            get_company_name(node),
            get_title(node),
            get_info(node),
            get_deadline(node),
            get_link(node),
        );

        result.push(data);
    }

    result
}

fn get_id(node: Node) -> u32 {
    node.find(Name("button").and(Class("lstbtn")))
        .next()
        .unwrap()
        .attr("data-gno")
        .unwrap()
        .parse()
        .unwrap()
}

fn get_company_name(node: Node) -> String {
    node.find(Class("tplCo"))
        .next()
        .unwrap()
        .find(Name("a"))
        .next()
        .unwrap()
        .text()
}

fn get_title(node: Node) -> String {
    node.find(Class("tplTit"))
        .next()
        .unwrap()
        .find(Name("a"))
        .next()
        .unwrap()
        .text()
}

fn get_info(node: Node) -> Vec<String> {
    node.find(Class("cell")).map(|x| x.text()).collect()
}

fn get_deadline(node: Node) -> String {
    node.find(Class("odd"))
        .next()
        .unwrap()
        .find(Class("date"))
        .next()
        .unwrap()
        .text()
}

fn get_link(node: Node) -> String {
    let link = node
        .find(Class("tplCo"))
        .next()
        .unwrap()
        .find(Name("a"))
        .next()
        .unwrap()
        .attr("href")
        .unwrap();

    format!("https://www.jobkorea.co.kr{}", link)
}

#[cfg(test)]
mod tests {
    use super::super::fetch::fetch;
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn data_extract_test() {
        let http_client = Client::new();
        let doc = fetch(&http_client).await.unwrap();
        data_extract(&doc);
    }
}

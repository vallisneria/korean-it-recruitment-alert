use super::saramin::Saramin;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};

pub fn data_extract(document: &Document) -> Vec<Saramin> {
    let mut result: Vec<Saramin> = Vec::new();
    let nodes = document.find(Class("common_recruilt_list").descendant(Class("list_item")));

    for node in nodes {
        let data = Saramin::new(
            get_id(node),
            get_title(node),
            get_company_name(node),
            get_career(node),
            get_education(node),
            get_employment_type(node),
            get_work_place(node),
            get_salary(node),
            get_deadline(node),
            get_link(node),
        );

        result.push(data);
    }

    result
}

fn get_id(node: Node) -> u32 {
    node.find(Class("idx_chk"))
        .next()
        .unwrap()
        .attr("value")
        .unwrap()
        .parse()
        .unwrap()
}

fn get_company_name(node: Node) -> String {
    node.find(Class("company_nm").descendant(Name("a")))
        .next()
        .unwrap()
        .attr("title")
        .unwrap()
        .parse()
        .unwrap()
}

fn get_title(node: Node) -> String {
    node.find(Class("notification_info").descendant(Name("a")))
        .next()
        .unwrap()
        .attr("title")
        .unwrap()
        .parse()
        .unwrap()
}

fn get_career(node: Node) -> Option<String> {
    match node.find(Class("career")).next() {
        Some(item) => Some(item.text()),
        None => None,
    }
}

fn get_education(node: Node) -> Option<String> {
    match node.find(Class("education")).next() {
        Some(item) => Some(item.text()),
        None => None,
    }
}

fn get_employment_type(node: Node) -> Option<String> {
    match node.find(Class("employment_type")).next() {
        Some(item) => Some(item.text()),
        None => None,
    }
}

fn get_work_place(node: Node) -> Option<String> {
    match node.find(Class("work_place")).next() {
        Some(item) => Some(item.text()),
        None => None,
    }
}

fn get_salary(node: Node) -> Option<String> {
    match node.find(Class("salary")).next() {
        Some(item) => Some(item.text()),
        None => None,
    }
}

fn get_deadline(node: Node) -> Option<String> {
    match node.find(Class("deadlines")).next() {
        Some(item) => Some(item.find(Name("span").not()).next()?.text()),
        None => None,
    }
}

fn get_link(node: Node) -> String {
    let path = node
        .find(Class("notification_info").descendant(Name("a")))
        .next()
        .unwrap()
        .attr("href")
        .unwrap();

    format!("https://saramin.co.kr{}", path)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn data_extracting_test() {
        use select::document::Document;
        let html = include_str!("saramin_job_list_for_test.html");
        let doc = Document::from_read(html.as_bytes()).unwrap();

        data_extract(&doc);
    }
}

use super::saramin::Saramin;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};

pub fn data_extract(document: Document) -> Vec<Saramin> {
    let mut result: Vec<Saramin> = Vec::new();

    for node in document.find(Class("list_item")) {
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
    node.find(Class("btn_scrap"))
        .next()
        .unwrap()
        .attr("rec_idx")
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

fn get_career(node: Node) -> String {
    node.find(Class("career")).next().unwrap().text()
}

fn get_education(node: Node) -> String {
    node.find(Class("education")).next().unwrap().text()
}

fn get_employment_type(node: Node) -> Option<String> {
    match node.find(Class("employment_type")).next() {
        Some(employment_type) => Some(employment_type.text()),
        _ => None,
    }
}

fn get_work_place(node: Node) -> Option<String> {
    match node.find(Class("work_place")).next() {
        Some(work_place) => Some(work_place.text()),
        _ => None,
    }
}

fn get_salary(node: Node) -> Option<String> {
    match node.find(Class("salary")).next() {
        Some(salary) => Some(salary.text()),
        _ => None,
    }
}

fn get_deadline(node: Node) -> String {
    node.find(Class("deadlines")).next().unwrap().text()
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

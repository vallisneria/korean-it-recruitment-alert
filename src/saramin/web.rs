use crate::saramin::saramin::Saramin;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};

pub fn get_job_list(latest_id: u32) -> Result<Vec<Saramin>, Box<dyn std::error::Error>> {
    let mut result: Vec<Saramin> = Vec::new();
    const SARAMIN_URL: &str =
        "https://www.saramin.co.kr/zf_user/jobs/list/job-category?cat_bcd=4&sort=MD";
    let response = reqwest::blocking::get(SARAMIN_URL)?;
    let document = Document::from_read(response)?;

    for node in document.find(Class("list_item")) {
        let id = get_id(node);
        if id == latest_id {
            break;
        }

        let data = Saramin::new(
            id,
            get_company_name(node),
            get_title(node),
            get_career(node),
            get_education(node),
            get_employment_type(node),
            get_work_place(node),
            get_deadline(node),
            get_link(node),
        );

        result.push(data);
    }

    Ok(result)
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
        .to_string()
}

fn get_title(node: Node) -> String {
    node.find(Class("notification_info").descendant(Name("a")))
        .next()
        .unwrap()
        .attr("title")
        .unwrap()
        .to_string()
}

fn get_career(node: Node) -> String {
    node.find(Class("career"))
        .next()
        .unwrap()
        .text()
        .to_string()
}

fn get_education(node: Node) -> String {
    node.find(Class("education"))
        .next()
        .unwrap()
        .text()
        .to_string()
}

fn get_employment_type(node: Node) -> String {
    node.find(Class("employment_type"))
        .next()
        .unwrap()
        .text()
        .to_string()
}

fn get_work_place(node: Node) -> String {
    node.find(Class("work_place"))
        .next()
        .unwrap()
        .text()
        .to_string()
}

fn get_deadline(node: Node) -> String {
    node.find(Class("deadlines"))
        .next()
        .unwrap()
        .text()
        .to_string()
}

fn get_link(node: Node) -> String {
    format!(
        "https://saramin.co.kr{path}",
        path = node
            .find(Class("notification_info").descendant(Name("a")))
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
    )
}

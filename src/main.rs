mod saramin;
use saramin::web::get_job_list as get_saramin;

fn main() {
    // 데이터 가져오기
    let saramin_list = get_saramin(0).unwrap();

    // 가져온 데이터 처리
    for data in saramin_list.iter() {
        println!("{}\n\n", data.to_string());
    }

    // 가장 최근 데이터를 저장하기
    format!(
        "INSERT INTO saramin(id, company_name, title) VALUES ({id}, {company_name}, {title})",
        id = &saramin_list[0].id,
        company_name = &saramin_list[0].company_name,
        title = &saramin_list[0].title
    );
}

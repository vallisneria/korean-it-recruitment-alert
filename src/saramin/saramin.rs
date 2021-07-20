pub struct Saramin {
    // 공고 ID
    id: u32,

    // 회사명
    company_name: String,

    // 공고 제목
    title: String,

    // 링크
    link: String,

    // 경력
    career: String,

    // 학력
    education: String,

    // 근로형태
    employment_type: Option<String>,

    // 근무지
    work_place: Option<String>,

    // 임금
    salary: Option<String>,

    // 모집기한
    deadline: Option<String>,
}

impl ToString for Saramin {
    fn to_string(&self) -> String {
        let mut result = format!(
r"{title}
🏢 {company_name}

👨‍💼 {career}
🏫 {education}
👨‍💻 {employment_type}
🗺️ {work_place}
💰 {salary}
🕑 {deadline}

{link}
",
            title = &self.title,
            company_name = &self.company_name,
            career = &self.career,
            education = &self.education,
            employment_type = &self.employment_type.unwrap_or(String::from("-"))),
            work_place = &self.work_place.unwrap_or(String::from("-")),
            salary = &self.salary.unwrap_or(String::from("-")),
            deadline = &self.deadline,
            link = &self.link
        )
    }
}

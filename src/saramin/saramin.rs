pub struct Saramin {
    // 공고 ID
    pub id: u32,

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
    employment_type: String,

    // 근무지
    work_place: String,

    // 임금
    salary: String,

    // 모집기한
    deadline: String,
}

impl ToString for Saramin {
    fn to_string(&self) -> String {
        format!(
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
            employment_type = &self.employment_type,
            work_place = &self.work_place,
            salary = &self.salary,
            deadline = &self.deadline,
            link = &self.link
        )
    }
}

impl Saramin {
    pub fn new(
        id: u32, title: String, company_name: String, career: String, education: String,
        employment_type: String, work_place: String, salary: String, deadline: String,
        link: String,
    ) -> Saramin {
        Saramin {
            id,
            title,
            company_name,
            career,
            education,
            employment_type,
            work_place,
            salary,
            deadline,
            link,
        }
    }
}

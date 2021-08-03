pub struct Saramin {
    // 공고 ID
    pub id: u32,

    // 회사명
    pub company_name: String,

    // 공고 제목
    pub title: String,

    // 링크
    pub link: String,

    // 경력
    career: Option<String>,

    // 학력
    education: Option<String>,

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
        let is_none = String::from("-");

        format!(
            r"{title}
🏢 {company_name}

👨‍💼 {career}
🏫 {education}
👨‍💻 {employment_type}
🗺️ {work_place}
💰 {salary}
🕑 {deadline}

🔗 {link}
            ",
            title = &self.title,
            company_name = &self.company_name,
            career = &self.career.as_ref().unwrap_or(&is_none),
            education = &self.education.as_ref().unwrap_or(&is_none),
            employment_type = &self.employment_type.as_ref().unwrap_or(&is_none),
            work_place = &self.work_place.as_ref().unwrap_or(&is_none),
            salary = &self.salary.as_ref().unwrap_or(&is_none),
            deadline = &self.deadline.as_ref().unwrap_or(&is_none),
            link = &self.link
        )
    }
}

impl Saramin {
    pub fn new(
        id: u32, title: String, company_name: String, career: Option<String>,
        education: Option<String>, employment_type: Option<String>, work_place: Option<String>,
        salary: Option<String>, deadline: Option<String>, link: String,
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

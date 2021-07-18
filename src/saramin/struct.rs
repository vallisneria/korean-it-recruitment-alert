pub struct Saramin {
    // 공고 ID
    id: u32,

    // 회사명
    company_name: &str,

    // 공고 제목
    title: &str,

    // 링크
    link: &str,

    // 경력
    career: Option<&str>,

    // 학력
    education: Option<&str>,

    // 근로형태
    employment_type: Option<&str>,

    // 임금
    salary: Option<&str>,

    // 모집기한
    deadline: Option<&str>,
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

{link}",
            title=self.title,
            company_name=self.company_name,
            career=self.career.unwrap_or("")
        )
    }
}

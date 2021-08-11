use crate::posting::mastodon::MastodonUpload;

/// 잡코리아 채용 공고
pub struct JobKorea {
    /// 공고 id
    pub id: u32,

    /// 회사명
    company_name: String,

    /// 공고 제목
    pub title: String,

    /// 경력
    career: Option<String>,

    /// 학력
    education: Option<String>,

    /// 근무지
    work_place: Option<String>,

    /// 근로형태
    employment_type: Option<String>,

    /// 임금
    salary: Option<String>,

    /// 직급
    position: Option<String>,

    /// 모집기한
    deadline: String,

    /// 링크
    link: String,
}

impl JobKorea {
    pub fn new(
        id: u32, company_name: String, title: String, career: Option<String>,
        education: Option<String>, work_place: Option<String>, employment_type: Option<String>,
        salary: Option<String>, position: Option<String>, deadline: String, link: String,
    ) -> JobKorea {
        JobKorea {
            id,
            company_name,
            title,
            career,
            education,
            work_place,
            employment_type,
            salary,
            position,
            deadline,
            link,
        }
    }
}

impl MastodonUpload for JobKorea {
    fn spoiler_text(&self) -> String {
        self.title.clone()
    }

    fn status(&self) -> String {
        let none = String::from("-");
        format!(
            r"🏢 {company_name}
⌛ {career}
🏫 {education}
💼 {employment_type}
🗺️ {work_place}
💰 {salary}
👔 {position}
🕑 {deadline}

🔗 {link}",
            company_name = &self.company_name,
            career = &self.career.as_ref().unwrap_or(&none),
            education = &self.education.as_ref().unwrap_or(&none),
            employment_type = &self.employment_type.as_ref().unwrap_or(&none),
            work_place = &self.work_place.as_ref().unwrap_or(&none),
            salary = &self.salary.as_ref().unwrap_or(&none),
            position = &self.position.as_ref().unwrap_or(&none),
            deadline = &self.deadline,
            link = &self.link
        )
    }
}

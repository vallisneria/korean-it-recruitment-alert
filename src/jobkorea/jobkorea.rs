use crate::posting::mastodon::MastodonUpload;

/// 잡코리아 채용 공고
pub struct JobKorea {
    /// 공고 id
    pub id: u32,

    /// 회사명
    company_name: String,

    /// 공고 제목
    pub title: String,

    /// 경력, 학력, 근무지 등 정보
    info: Vec<String>,

    /// 모집기한
    deadline: String,

    /// 링크
    link: String,
}

impl JobKorea {
    pub fn new(
        id: u32, company_name: String, title: String, info: Vec<String>, deadline: String,
        link: String,
    ) -> JobKorea {
        JobKorea {
            id,
            company_name,
            title,
            info,
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
        format!(
            r"🏢 {company_name}

{info}

🕑 {deadline}

🔗 {link}",
            company_name = &self.company_name,
            info = &self.info.join(" / "),
            deadline = &self.deadline,
            link = &self.link
        )
    }
}

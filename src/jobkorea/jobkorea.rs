use crate::posting::mastodon::MastodonUpload;

pub struct JobKorea {
    id: u32,
    company_name: String,
    title: String,
    info: Vec<String>,
    deadline: String,
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

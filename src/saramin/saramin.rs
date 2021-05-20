pub struct Saramin {
    pub id: u32,
    pub company_name: String,
    pub title: String,
    career: String,
    education: String,
    employment_type: String,
    work_place: String,
    salary: String,
    deadline: String,
    link: String,
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
            title = self.title,
            company_name = self.company_name,
            career = self.career,
            education = self.education,
            employment_type = self.employment_type,
            work_place = self.work_place,
            salary = self.salary,
            deadline = self.deadline,
            link = self.link
        )
    }
}

impl Saramin {
    pub fn new(
        id: u32,
        company_name: String,
        title: String,
        career: String,
        education: String,
        employment_type: String,
        work_place: String,
        salary: String,
        deadline: String,
        link: String,
    ) -> Saramin {
        Saramin {
            id,
            company_name,
            title,
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

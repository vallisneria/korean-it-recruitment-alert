use super::saramin::Saramin;
use rusqlite::{params, Connection};
use std::error::Error;

/// 데이터베이스가 없다면 새로 초기화한다.
pub fn db_init(conn: Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS saramin (
            id              INTEGER PRIMARY KEY,
            title           TEXT,
            company_name    TEXT,
            url             TEXT,
            add_date        DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )?;

    Ok(())
}

/// 데이터베이스 내의 모든 값을 지운다.
pub fn delete_all(conn: Connection) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM saramin WHERE *;", [])?;

    Ok(())
}

/// Saramin 구조체 하나를 받아 데이터베이스에 추가한다.
pub fn insert(conn: Connection, data: Saramin) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "INSERT INTO saramin (id, company_name, title, url) VALUES (?1, ?2, ?3, ?4);",
        params![data.id, data.company_name, data.title, data.url],
    )?;

    Ok(())
}

/// 가장 마지막으로 작성된 데이터를 가져온다.
pub fn latest(conn: Connection) -> Result<Saramin, Box<dyn Error>> {
    let stmt = conn.prepare("SELECT * FROM saramin ORDER BY add_date LIMIT 1;")?;
    let data = stmt.query_map([], |row| {
        Saramin::new(
            id: row.get(0)?,
            title: row.get(1)?,
            company_name: row.get(2)?,
            career: None,
            education: None,
            employment_type: None,
            work_place: None,
            salary: None,
            deadline: None,
            link: row.get(3)?,
        )
    })?;

    Ok(data[0])
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use super::*;

    #[test]
    fn db_init_run_test() {
        let conn = Connection::open_in_memory().unwrap();
        db_init(conn: Connection)?;
    }
}

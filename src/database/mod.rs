use rusqlite::{params, Connection};

pub fn table_init(client: &Connection, db_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    client.execute(
        "CREATE TABLE IF NOT EXISTS ?1 (id integer NOT NULL, company_name text, title text)",
        params![db_name],
    )?;

    Ok(())
}

pub fn get_last_id(client: &Connection, db_name: &str) -> Result<u32, Box<dyn std::error::Error>> {
    #![rustfmt::skip]
    let result: u32 = client.query_row(
        "SELECT id FROM ?1 LIMIT 1",
        params![db_name],
        |row| row.get(0)
    )?;

    Ok(result)
}

pub fn all_delete(client: &Connection, db_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    client.execute("DELETE FROM ?1", &[&db_name])?;

    Ok(())
}

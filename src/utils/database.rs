extern crate rusqlite;

use rusqlite::{Connection, Result};

use crate::models::base::Info;

pub fn create_task(conn: &Connection) -> Result<()> {
	conn.execute(r#"
		CREATE TABLE IF NOT EXISTS task (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
            title VARCHAR NOT NULL,
            is_completed BOOLEAN,
            book_id INTEGER NOT NULL,
            FOREIGN KEY (book_id) REFERENCES book (id) ON DELETE CASCADE
		)
	"#, [])?;
	Ok(())
}

pub fn create_book(conn: &Connection) -> Result<()> {
	conn.execute(r#"
		CREATE TABLE IF NOT EXISTS book (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			label VARCHAR NOT NULL UNIQUE
		)
	"#, [])?;
	Ok(())
}

pub fn get_info(conn: &Connection) -> Result<Info> {
    let books = conn.query_row("SELECT COUNT(*) FROM book", [], |row| row.get(0))?;
    let tasks = conn.query_row("SELECT COUNT(*) FROM task", [], |row| row.get(0))?;
    let completed = conn.query_row("SELECT COUNT(*) FROM task WHERE is_completed = true", [], |row| row.get(0))?;
    let pending = conn.query_row("SELECT COUNT(*) FROM task WHERE is_completed = false", [], |row| row.get(0))?;
    Ok(Info { books, tasks, completed, pending })
}

pub fn drop_database(conn: &Connection) -> Result<()> {
	conn.execute("DROP TABLE IF EXISTS book", [])?;
	conn.execute("DROP TABLE IF EXISTS task", [])?;
	Ok(())
}

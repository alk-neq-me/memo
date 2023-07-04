use rusqlite::Connection;

use crate::utils::colors::Color;
use super::database::{create_book, create_task};


// Create Book table and Task table
pub fn table_initialize(conn: &Connection) {
    match create_book(&conn) {
        Ok(_) => {},
        Err(err) => {
            let colorized = Color::Red("Failed");
            println!("[ {colorized} ] failed create/reload table, {err}");
        }
    }
    match create_task(&conn) {
        Ok(_) => {},
        Err(err) => {
            let colorized = Color::Red("Failed");
            println!("[ {colorized} ] failed create/reload task, {err}");
        }
    }
}

use rusqlite::{Connection, Result};

use crate::models::book::Book;

pub fn get_books(conn: &Connection) -> Result<Vec<Book>> {
	let mut stmt = conn.prepare("SELECT * FROM book")?;
	let book_iter = stmt.query_map([], |row| {
		Ok(Book { 
            id: row.get(0).unwrap(),
            label: row.get(1).unwrap() 
        })
	})?;
    
    let mut books: Vec<Book> = vec![];

	for book in book_iter {
        books.push(book.unwrap());
	}
	Ok(books)
}

pub fn add_book<'a>(conn: &Connection, book: &'a Book) -> Result<&'a Book> {
	conn.execute(r#"
        INSERT INTO book(label) VALUES(?1)
	"#, [&book.label])?;
	Ok(book)
}

pub fn remove_book(conn: &Connection, id: &u32) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON", [])?;
	conn.execute(r#"
        DELETE FROM book
        WHERE id = ?1
	"#, [&id])?;
	Ok(())
}



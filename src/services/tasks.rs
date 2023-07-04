use rusqlite::{Connection, Result, Statement};

use crate::models::{task::Task, book::Book};


pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
	let mut stmt = conn.prepare("SELECT * FROM task")?;
	let task_iter = stmt.query_map([], |row| {
		Ok(Task {
            id: row.get(0).unwrap(),
            title: row.get(1).unwrap(),
            is_completed: row.get(2).unwrap(),
            book_id: row.get(3).unwrap()
        })
	})?;

    let mut tasks: Vec<Task> = vec![];

	for task in task_iter {
        tasks.push(task.unwrap());
	}
	Ok(tasks)
}

pub fn get_tasks(conn: &Connection, book: &str, completed: bool) -> Result<Vec<Task>> {
    let mut stmt: Statement;
    if !completed {
        stmt = conn.prepare("SELECT * FROM task INNER JOIN book ON task.book_id = book.id WHERE book.label = ?")?;
    } else {
        stmt = conn.prepare("SELECT * FROM task INNER JOIN book ON task.book_id = book.id WHERE book.label = ? AND is_completed = true")?;
    }
	let task_iter = stmt.query_map([book], |row| {
		Ok(Task {
            id: row.get(0).unwrap(),
            title: row.get(1).unwrap(),
            is_completed: row.get(2).unwrap(),
            book_id: row.get(3).unwrap()
        })
	})?;

    let mut tasks: Vec<Task> = vec![];

	for task in task_iter {
        tasks.push(task.unwrap());
	}
	Ok(tasks)
}

// pub fn get_single_task(conn: &Connection, id: &u32) -> Result<Task> {
// 	let mut stmt = conn.prepare("SELECT * FROM task WHERE id = ?")?;
// 	let task = stmt.query_row([id], |row| {
// 		Ok(Task {
//             id: row.get(0).unwrap(),
//             title: row.get(1).unwrap(),
//             is_completed: row.get(2).unwrap(),
//             book_id: row.get(3).unwrap()
//         })
// 	})?;
// 	Ok(task)
// }

pub fn add_task<'a>(conn: &Connection, task: &'a Task, book_label: &str) -> Result<&'a Task> {
    let find_book = conn.query_row("SELECT * FROM book WHERE label = ?", [&book_label], |row| {
        Ok(Book {
            id: row.get(0).unwrap(),
            label: row.get(1).unwrap()
        })
    })?; //.expect(&format!("Not found book with label {}", book_label));

	conn.execute(r#"
        INSERT INTO task(title, is_completed, book_id) 
        VALUES(?1, false, ?2)
	"#, [&task.title, &find_book.id.unwrap().to_string()])?;
	Ok(task)
}

// pub fn update_task(conn: &Connection, task: &Task, id: &u32) -> Result<()> {
//     conn.execute(r#"
//         UPDATE task
//         SET title=?1, is_completed=?2
//         WHERE id=?3
//     "#, [&task.title, &(task.is_completed.to_string()), &id.to_string()])?;
//     Ok(())
// }

pub fn make_completed(conn: &Connection, id: &u32) -> Result<()> {
    conn.execute(r#"
        UPDATE task
        SET is_completed = 1
        WHERE id=?
    "#, [&id])?;
    Ok(())
}

pub fn remove_task(conn: &Connection, id: &u32) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON", [])?;
	conn.execute(r#"
        DELETE FROM task
        WHERE id = ?1
	"#, [&id])?;
	Ok(())
}


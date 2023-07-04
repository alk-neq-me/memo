extern crate rusqlite;

mod utils;
mod args;
mod services;
mod models;

use std::io::{stdin, stdout, Write};

use rusqlite::Connection;

use args::parse_args;
use services::tasks::toggle_completed;

use crate::{
    utils::{
        table_initialize::table_initialize, 
        colors::Color, database::{
            drop_database, get_info, 
        }, clean_console::clean_console
    }, 
    models::{
        book::Book, 
        task::Task
    }, 
    services::{
        books::{
            add_book, 
            remove_book, 
            get_books
        }, 
        tasks::{
            get_all_tasks, 
            add_task, 
            remove_task, 
            get_tasks, 
            make_completed, calculate_completion_percentage
        }
    }
};

fn main() {
    // let conn = Connection::open_in_memory().unwrap();
    let conn = Connection::open("memo.db").unwrap();
    let args = parse_args();

    table_initialize(&conn);

    if let Some(label) = &args.add_book {
        let book = Book { label: label.to_owned(), id: Some(0) };
        let book_added = add_book(&conn, &book);
        match book_added {
            Ok(book) => {
                let colorize = Color::Green("Created 🌱");
                println!("\n{}", format!("[ {} ] Book `{}` added successfully.", colorize, book.label));
            },
            Err(err) => {
                let colorize = Color::Red("Failed 💔");
                println!("\n{}", format!("[ {} ] Failed to add the book. Error: {}", colorize, err));
            }
        }
    }

    // book remove with id
    if let Some(book_id) = &args.remove_book {
        print!("Are you sure you want to delete the book? [y/n]: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to reead input.");
        let choose = input.trim().to_lowercase();

        match choose.as_str() {
            "y" | "yes" => {
                match remove_book(&conn, book_id) {
                    Ok(_) => {
                        let colorize = Color::Green("Deleted 🗑️");
                        println!("\n{}", format!("[ {} ] Book `{}` deleted successfully.", colorize, book_id));
                    },
                    Err(err) => {
                        let colorize = Color::Red("Failed 💔");
                        println!("\n{}", format!("[ {} ] Failed to delete the book. Error: {}", colorize, err));
                    }
                }
            },
            "n" | "no" => println!("Canceled to delete book"),
            _ => println!("Invalid input. Please enter 'y' or 'n'.")
        }
    }

    if args.list_book {
        let books: Vec<Book> = get_books(&conn).expect(&format!("[ {} ] failed get book", Color::Red("FAILED")));
        for book in books {
            println!("{:?} 📚 {}", book.id, book.label);
        }
    }

    if args.list_task {
        if args.completed {
            let tasks: Vec<Task> = get_all_tasks(&conn, true).unwrap();
            for task in tasks {
                let colorize = Color::Delete(&task.title);
                println!("{:?} ✅ {}", task.id, colorize);
            }
        } else {
            let tasks: Vec<Task> = get_all_tasks(&conn, false).unwrap();
            for task in tasks {
                if task.is_completed {
                    let colorize = Color::Delete(&task.title);
                    println!("{:?} ✅ {}", task.id, colorize);
                } else {
                    println!("{:?} 🚀 {}", task.id, task.title);
                }
            }
        }
    }

    if let Some(title) = &args.add_task {
        if let Some(book_label) = &args.book {
            let new_task = Task {
                id: None,
                title: title.to_owned(),
                is_completed: false,
                book_id: None
            };
            match add_task(&conn, &new_task, book_label) {
                Ok(_) => {
                    let colorize = Color::Green("Created 🌱");
                    println!("\n{}", format!("[ {} ] Task added successfully.", colorize));
                },
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    let colorize = Color::Red("Not Found 🤔");
                    println!("\n{}", format!("[ {} ] Failed to add the task not found book", colorize));
                },
                Err(err) => {
                    let colorize = Color::Red("Failed 💔");
                    println!("\n{}", format!("[ {} ] Failed to add the task. Error: {}", colorize, err));
                },
            }
        } else {
            println!("If you add task, must need book");
        }
    }

    // remove task
    if let Some(task_id) = &args.remove_task {
        print!("Are you sure you want to delete the task? [y/n]: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to reead input.");
        let choose = input.trim().to_lowercase();

        match choose.as_str() {
            "y" | "yes" => {
                match remove_task(&conn, task_id) {
                    Ok(_) => {
                        let colorize = Color::Green("Deleted 🗑️");
                        println!("\n{}", format!("[ {} ] Task `{}` deleted successfully.", colorize, task_id));
                    },
                    Err(err) => {
                        let colorize = Color::Red("Failed 💔");
                        println!("\n{}", format!("[ {} ] Failed to delete the task. Error: {}", colorize, err));
                    }
                }
            },
            "n" | "no" => println!("Canceled to delete task"),
            _ => println!("Invalid input. Please enter 'y' or 'n'.")
        }

        
    }

    // get task in a book
    if let Some(book) = &args.book {
        clean_console();
        if args.completed {
            let tasks = get_tasks(&conn, &book, true).expect("Faied");
            for task in tasks {
                let colorize = Color::Delete(&task.title);
                println!("{:?} ✅ {}", task.id, colorize);
            }
        } else {
            let tasks = get_tasks(&conn, &book, false).expect("Faied");
            for task in tasks {
                if task.is_completed {
                    let colorize = Color::Delete(&task.title);
                    println!("{:?} ✅ {}", task.id, colorize);
                } else {
                    println!("{:?} 🚀 {}", task.id, task.title);
                }
            }
        }
    }

    // make complete task or update
    if let Some(task_id) = &args.complete_task {
        let updated = make_completed(&conn, task_id);
        match updated {
            Ok(task) => {
                let colorize = Color::Purple("Updated 💥");
                println!("\n{}", format!("[ {} ] Task completed successfully.", colorize));
                if task.is_completed {
                    let colorize = Color::Delete(&task.title);
                    println!("{:?} ✅ {}", task.id, colorize);
                } else {
                    println!("{:?} 🚀 {}", task.id, task.title);
                }
            },
            Err(err) => {
                let colorize = Color::Red("Failed 💔");
                println!("\n{}", format!("[ {} ] Task complet failed. Error: {err:?}", colorize));
            },
        }
    }

    // toggle task
    if let Some(task_id) = &args.toggle_task {
        let updated = toggle_completed(&conn, task_id);
        match updated {
            Ok(task) => {
                let colorize = Color::Purple("Updated 💥");
                println!("\n{}", format!("[ {} ] Task completed successfully.", colorize));
                if task.is_completed {
                    let colorize = Color::Delete(&task.title);
                    println!("{:?} ✅ {}", task.id, colorize);
                } else {
                    println!("{:?} 🚀 {}", task.id, task.title);
                }
            },
            Err(err) => {
                let colorize = Color::Red("Failed 💔");
                println!("\n{}", format!("[ {} ] Task complet failed. Error: {err:?}", colorize));
            },
        }
    }

    // clean all
    if args.clean_all {
        print!("Are you sure you want to delete the task? [y/n]: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to reead input.");
        let choose = input.trim().to_lowercase();

        match choose.as_str() {
            "y" | "yes" => {
                match drop_database(&conn) {
                    Ok(_) => {
                        let colorize = Color::Purple("Clean All 🗑️");
                        println!("\n{}", format!("[ {} ] Clean all successfully.", colorize));
                    },
                    Err(err) => {
                        let colorize = Color::Red("Failed 💔");
                        println!("\n{}", format!("[ {} ] Failed to delete the task. Error: {}", colorize, err));
                    }
                }
            },
            "n" | "no" => println!("Canceled to delete task"),
            _ => println!("Invalid input. Please enter 'y' or 'n'.")
        }
    }

    if args.info {
        let info = get_info(&conn).expect("FAILED count");
        let percentage = calculate_completion_percentage(&conn).unwrap();
        println!("\n\n{:.0}% of all tasks complete.", percentage);

        println!("{} done {} pending {} books", Color::Green(&info.completed.to_string()), Color::Purple(&info.pending.to_string()), Color::Blue(&info.books.to_string()));
    }
}

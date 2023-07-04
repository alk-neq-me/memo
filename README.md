# Memo - 메모

Seedling is a command-line application designed for managing memos and notes efficiently from your terminal. It provides a simple and lightweight interface to create, view, update, and delete memos using text-based commands.

## Usage

```shell
memo [OPTIONS]
```

## Options

- `--book`: Open the memo book.
- `--list-book`: List all available memo books.
- `--list-task`: List all tasks in the current memo book.
- `--add-book <ADD_BOOK>`: Create a new memo book with the specified name.
- `--remove-book <REMOVE_BOOK>`: Remove the memo book with the specified book ID.
- `--add-task <ADD_TASK>`: Add a new task to the current memo book.
- `--remove-task <REMOVE_TASK>`: Remove the task with the specified task ID from the current memo book.
- `--complete-task <COMPLETE_TASK>`: Mark the task with the specified task ID as completed in the current memo book.
- `--count`: Display the total number of tasks in the current memo book.
- `--completed`: List all completed tasks in the current memo book.
- `--clean-all`: Remove all memo books and tasks.
- `-h, --help`: Print the help information.
- `-V, --version`: Print the version of the application.

## Getting Started

1. Clone the repository.
2. Build the project using `cargo build`.
3. Run the application using `cargo run` or `./target/debug/memo`.
4. Use the provided options and commands to manage your memos and tasks.

Feel free to explore the functionalities of Seedling and make it your go-to tool for organizing your memos and notes directly from the command line.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

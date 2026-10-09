# TaskManagerCLI

A simple command-line task manager written in Rust. This project is being developed as a learning project to practice Rust fundamentals, error handling, testing, and code organization.

## Features

- Create tasks with a title, description, and priority.
- List all tasks and their completion statuses.
- Display detailed information about a task.
- Mark tasks as completed.
- Delete tasks by ID.
- Validate task titles and priority values.
- Handle invalid commands and task-related errors.
- Run automated tests with Cargo.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with the Rust toolchain)

The project uses Rust edition 2024.

## Installation

Clone the repository:

```bash
git clone https://github.com/JayStorm228/TaskManagerCLI.git
cd TaskManagerCLI
```

Switch to the development branch if you want to use the task-priority feature:

```bash
git switch feature/task-priority
```

Build the project:

```bash
cargo build
```

## Usage

Run the application with:

```bash
cargo run
```

The application accepts commands interactively. Enter a command and press Enter to execute it.

### Available commands

| Command                                | Description                    |
| -------------------------------------- | ------------------------------ |
| `add <title> <priority> [description]` | Create a task                  |
| `list`                                 | Display all tasks              |
| `show <id>`                            | Display task details           |
| `complete <id>`                        | Mark a task as completed       |
| `delete <id>`                          | Delete a task                  |
| `help`                                 | Display the available commands |
| `exit`                                 | Exit the application           |

### Examples

Create a task:

```text
add LearnRust high Read the Rust Book
```

Create a task without a description:

```text
add Exercise medium
```

Display all tasks:

```text
list
```

Display a specific task:

```text
show 0
```

Mark a task as completed:

```text
complete 0
```

Delete a task:

```text
delete 0
```

Exit the application:

```text
exit
```

In the `add` command, the title is the first argument, the priority is the second argument, and the remaining arguments form the description.

## Task priorities

Each task can have one of four priority values:

| Priority | Description          |
| -------- | -------------------- |
| `low`    | Low priority         |
| `medium` | Medium priority      |
| `high`   | High priority        |
| `none`   | No assigned priority |

Priority names are case-insensitive. For example, `high`, `HIGH`, and `High` are accepted.

If the priority argument is omitted, the parser defaults to `none`.

## Validation and errors

- Task titles must contain at least three Unicode characters.
- Priority values must match one of the supported options.
- Task IDs must be valid unsigned 32-bit integers (`u32`).
- An operation targeting a nonexistent task returns an error.
- A task cannot be marked as completed more than once.

Task IDs start at `0` and increase after each successful task creation. Deleted IDs are not reused during the current session.

## Development

Format the code:

```bash
cargo fmt
```

Check the project for compilation errors:

```bash
cargo check
```

Run the test suite:

```bash
cargo test
```

Generate the Rust documentation:

```bash
cargo doc --no-deps
```

## Current limitations

- Tasks are stored in memory and are lost when the application exits.
- The command parser uses whitespace to separate arguments, so task titles cannot currently contain spaces.
- The application does not yet provide persistent storage or a configuration system.

## License

No license has been specified yet.

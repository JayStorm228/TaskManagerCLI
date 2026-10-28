# TaskManagerCLI

A command-line task manager written in Rust. It allows you to create tasks, inspect them, track their completion status, and delete them.

## Features

- Create tasks with a title, optional description, and priority.
- List all tasks and their completion statuses.
- Display detailed information about a task by its ID.
- Mark tasks as completed.
- Delete tasks by their IDs.
- Parse commands using named arguments.
- Handle quoted values and spaces in command-line input.

> **Note:** Task data is currently stored in memory and is lost when the application exits.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with the Rust toolchain)

## Getting started

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/JayStorm228/TaskManagerCLI.git
cd TaskManagerCLI
```

Build the project:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

## Commands

The application accepts commands followed by named arguments.

| Command    | Arguments        | Description                           |
| ---------- | ---------------- | ------------------------------------- |
| `add`      | `--title <text>` | Create a task.                        |
| `list`     | None             | Display all tasks and their statuses. |
| `show`     | `--id <number>`  | Display a task's details.             |
| `complete` | `--id <number>`  | Mark a task as completed.             |
| `delete`   | `--id <number>`  | Delete a task.                        |
| `help`     | None             | Display available commands.           |
| `exit`     | None             | Exit the application.                 |

### Adding a task

A title is required. Description and priority are optional.

```text
add --title "Buy groceries"
add --title "Buy groceries" --description "Milk and bread"
add --title "Finish project" --priority high
add --title "Prepare presentation" --description "Slides for the demo" --priority medium
```

Default values:

- `--description`: an empty string.
- `--priority`: `none`.

Values containing spaces should be enclosed in double quotes.

### Working with tasks

Display all tasks:

```text
list
```

Show a task by its ID:

```text
show --id 1
```

Mark a task as completed:

```text
complete --id 1
```

Delete a task:

```text
delete --id 1
```

Display help:

```text
help
```

Exit the application:

```text
exit
```

Task IDs are unsigned integers.

## Development

Build the project:

```bash
cargo build
```

Run the test suite:

```bash
cargo test --all-targets
```

Check formatting:

```bash
cargo fmt --check
```

Run Clippy:

```bash
cargo clippy --all-targets -- -D warnings
```

Build the Rust documentation:

```bash
cargo doc --no-deps
```

To open the generated documentation in a browser:

```bash
cargo doc --no-deps --open
```

## Branching and CI

The project uses two main branches:

- **`unstable`** — integration branch for completed features and ongoing development. Intermediate commits are not necessarily expected to represent a fully working product.
- **`stable`** — branch for a consolidated, working state of the product.

Feature branches should normally be based on `unstable`, and completed features should be proposed for merging into `unstable` through pull requests.

CI runs for pull requests targeting both `unstable` and `stable`. These checks help validate changes before integration; they do not imply that every intermediate commit on a development branch is a release-ready version.

The goal is for **each completed feature to be functional**, even though the development process may include intermediate states that are not yet fully functional.

## Project status

TaskManagerCLI is under active development. Commands, argument parsing, and other behavior may evolve as features are added and integrated.

See [CHANGELOG.md](CHANGELOG.md) for a record of notable changes.

## License

See the repository for license information.

# Changelog

All notable changes to TaskManagerCLI are documented in this file.

The format is inspired by [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows semantic versioning when releases are assigned.

## [Unreleased]

### Added

- Named arguments for task commands.
- Quoted values for arguments that contain spaces.
- A command-line parsing layer that separates tokenization from command interpretation.
- Parsing and validation for task IDs supplied through `--id`.
- Automated tests for command-line tokenization, argument parsing, and error handling.
- Rustdoc documentation for the command-line parsing and command types.

### Changed

- The syntax for creating tasks now uses named arguments instead of positional arguments.
- Task-related commands now accept `--id` to identify the target task.
- The `add` command requires `--title`; `--description` and `--priority` are optional.
- Omitted descriptions default to an empty string.
- Omitted priorities default to `none`.

### Fixed

- Improved handling and reporting of malformed command-line input, including unterminated quotes, invalid characters immediately following a closing quote, missing argument values, and invalid task IDs.

### Development

- The parser refactor is developed on a feature branch based on `unstable`.
- Completed features are integrated into `unstable` through pull requests.
- CI checks are triggered for pull requests targeting both `unstable` and `stable`.

---

_This section records changes currently under development. It does not represent a released version._

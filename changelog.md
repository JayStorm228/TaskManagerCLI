# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Task priorities: `low`, `medium`, `high`, and `none`.
- Priority validation when creating tasks.
- Automated tests covering task-priority functionality.
- Rustdoc documentation for the crate's public API.

### Changed

- Refactored the application structure and separated tests into dedicated modules.
- Added a code formatter to improve consistency across the codebase.
- Updated the CLI help output to document the priority argument and the `exit` command.

### Fixed

- Fixed priority display in the detailed task view.
- Fixed Clippy and linting issues in the CLI.
- Fixed the test-only task manager accessor to avoid dead-code warnings.

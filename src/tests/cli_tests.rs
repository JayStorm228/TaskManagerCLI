// tests/cli_tests.rs — только публичный API

use crate::cli::Cli;
use crate::command::Command;
use crate::errors::TaskError;

fn create_cli() -> Cli {
    Cli::new()
}

// ===== execute: Add =====

#[test]
fn test_cli_execute_add_command() {
    let mut cli = create_cli();
    let cmd = Command::Add {
        title: "FromCommand".to_string(),
        description: "Desc".to_string(),
    };
    let res = cli.execute(cmd);
    assert!(res.is_ok());
    assert_eq!(cli.task_mgr().tasks().len(), 1);
}

#[test]
fn test_cli_execute_add_empty_description() {
    let mut cli = create_cli();
    let cmd = Command::Add {
        title: "Title".to_string(),
        description: "".to_string(),
    };
    let res = cli.execute(cmd);
    assert!(res.is_ok());
    assert_eq!(cli.task_mgr().tasks()[0].description, "");
}

// ===== execute: Show =====

#[test]
fn test_cli_execute_show_existing() {
    let mut cli = create_cli();
    cli.execute(Command::Add {
        title: "ShowTask".to_string(),
        description: "Desc".to_string(),
    })
    .unwrap();

    let res = cli.execute(Command::Show { id: 0 });
    assert!(res.is_ok());
}

#[test]
fn test_cli_execute_show_not_found() {
    let mut cli = create_cli();
    let res = cli.execute(Command::Show { id: 99 });
    assert_eq!(res, Err(TaskError::IDNotFound(99)));
}

// ===== execute: Delete =====

#[test]
fn test_cli_execute_delete_existing() {
    let mut cli = create_cli();
    cli.execute(Command::Add {
        title: "ToDelete".to_string(),
        description: "Desc".to_string(),
    })
    .unwrap();

    let res = cli.execute(Command::Delete { id: 0 });
    assert!(res.is_ok());
    assert!(cli.task_mgr().tasks().is_empty());
}

#[test]
fn test_cli_execute_delete_not_found() {
    let mut cli = create_cli();
    cli.execute(Command::Add {
        title: "Keep".to_string(),
        description: "Desc".to_string(),
    })
    .unwrap();

    let res = cli.execute(Command::Delete { id: 42 });
    assert_eq!(res, Err(TaskError::IDNotFound(42)));
}

// ===== execute: Complete =====

#[test]
fn test_cli_execute_complete_existing() {
    let mut cli = create_cli();
    cli.execute(Command::Add {
        title: "ToComplete".to_string(),
        description: "Desc".to_string(),
    })
    .unwrap();

    let res = cli.execute(Command::Complete { id: 0 });
    assert!(res.is_ok());
    assert!(cli.task_mgr().tasks()[0].is_completed);
}

#[test]
fn test_cli_execute_complete_already_completed() {
    let mut cli = create_cli();
    cli.execute(Command::Add {
        title: "Task".to_string(),
        description: "Desc".to_string(),
    })
    .unwrap();
    cli.execute(Command::Complete { id: 0 }).unwrap();

    let res = cli.execute(Command::Complete { id: 0 });
    assert_eq!(res, Err(TaskError::AlreadyCompleted));
}

#[test]
fn test_cli_execute_complete_not_found() {
    let mut cli = create_cli();
    cli.execute(Command::Add {
        title: "Task".to_string(),
        description: "Desc".to_string(),
    })
    .unwrap();

    let res = cli.execute(Command::Complete { id: 99 });
    assert_eq!(res, Err(TaskError::IDNotFound(99)));
}

// ===== execute: List =====

#[test]
fn test_cli_execute_list_empty() {
    let mut cli = create_cli();
    let res = cli.execute(Command::List);
    assert!(res.is_ok());
}

#[test]
fn test_cli_execute_list_with_tasks() {
    let mut cli = create_cli();
    cli.execute(Command::Add {
        title: "Task 1".to_string(),
        description: "Desc".to_string(),
    })
    .unwrap();
    cli.execute(Command::Add {
        title: "Task 2".to_string(),
        description: "Desc".to_string(),
    })
    .unwrap();

    let res = cli.execute(Command::List);
    assert!(res.is_ok());
    assert_eq!(cli.task_mgr().tasks().len(), 2);
}

// ===== execute: Help =====

#[test]
fn test_cli_execute_help() {
    let mut cli = create_cli();
    let res = cli.execute(Command::Help);
    assert!(res.is_ok());
}

// ===== execute: Exit (не тестируем, потому что exit(0) завершает процесс) =====
// Если нужно тестировать, можно сделать exit() инжектируемым через trait или closure.

// ===== integration scenarios =====

#[test]
fn test_cli_full_workflow() {
    let mut cli = create_cli();

    // Add
    cli.execute(Command::Add {
        title: "Task 1".to_string(),
        description: "Desc 1".to_string(),
    })
    .unwrap();
    cli.execute(Command::Add {
        title: "Task 2".to_string(),
        description: "Desc 2".to_string(),
    })
    .unwrap();
    assert_eq!(cli.task_mgr().tasks().len(), 2);

    // Complete
    cli.execute(Command::Complete { id: 0 }).unwrap();
    assert!(cli.task_mgr().tasks()[0].is_completed);

    // Delete
    cli.execute(Command::Delete { id: 1 }).unwrap();
    assert_eq!(cli.task_mgr().tasks().len(), 1);

    // Show
    let res = cli.execute(Command::Show { id: 0 });
    assert!(res.is_ok());
}

#[test]
fn test_cli_operations_after_delete() {
    let mut cli = create_cli();
    cli.execute(Command::Add {
        title: "Task".to_string(),
        description: "Desc".to_string(),
    })
    .unwrap();
    cli.execute(Command::Delete { id: 0 }).unwrap();

    assert_eq!(
        cli.execute(Command::Show { id: 0 }),
        Err(TaskError::IDNotFound(0))
    );
    assert_eq!(
        cli.execute(Command::Complete { id: 0 }),
        Err(TaskError::IDNotFound(0))
    );
    assert_eq!(
        cli.execute(Command::Delete { id: 0 }),
        Err(TaskError::IDNotFound(0))
    );
}

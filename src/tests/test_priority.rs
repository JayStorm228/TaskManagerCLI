use std::str::FromStr;

use crate::command::Command;
use crate::errors::TaskError;
use crate::task::TaskPriority;
use crate::taskmanager::TaskManager;

// ===== TaskPriority::from_str =====

#[test]
fn test_parse_priority_low() {
    assert_eq!(TaskPriority::from_str("low"), Ok(TaskPriority::Low));
}

#[test]
fn test_parse_priority_medium() {
    assert_eq!(TaskPriority::from_str("medium"), Ok(TaskPriority::Medium));
}

#[test]
fn test_parse_priority_high() {
    assert_eq!(TaskPriority::from_str("high"), Ok(TaskPriority::High));
}

#[test]
fn test_parse_priority_none() {
    assert_eq!(TaskPriority::from_str("none"), Ok(TaskPriority::None));
}

#[test]
fn test_parse_priority_is_case_insensitive() {
    assert_eq!(TaskPriority::from_str("HIGH"), Ok(TaskPriority::High));

    assert_eq!(TaskPriority::from_str("Medium"), Ok(TaskPriority::Medium));

    assert_eq!(TaskPriority::from_str("LoW"), Ok(TaskPriority::Low));
}

#[test]
fn test_parse_priority_trims_whitespace() {
    assert_eq!(TaskPriority::from_str("  high  "), Ok(TaskPriority::High));
}

#[test]
fn test_parse_priority_invalid_value() {
    assert_eq!(
        TaskPriority::from_str("urgent"),
        Err(TaskError::ParseTaskPriorityError("urgent".to_string()))
    );
}

#[test]
fn test_parse_priority_empty_string() {
    assert_eq!(TaskPriority::from_str(""), Err(TaskError::ParseTaskPriorityError(String::new())));
}

// ===== TaskManager::add with priority =====

#[test]
fn test_add_task_with_low_priority() {
    let mut manager = TaskManager::new();

    let result = manager.add(
        "Low priority task".to_string(),
        "Description".to_string(),
        "low".to_string()
    );

    assert!(result.is_ok());
    assert_eq!(manager.tasks().len(), 1);
    assert_eq!(manager.tasks()[0].priority, TaskPriority::Low);
}

#[test]
fn test_add_task_with_medium_priority() {
    let mut manager = TaskManager::new();

    manager
        .add("Medium priority task".to_string(), "Description".to_string(), "medium".to_string())
        .unwrap();

    assert_eq!(manager.tasks()[0].priority, TaskPriority::Medium);
}

#[test]
fn test_add_task_with_high_priority() {
    let mut manager = TaskManager::new();

    manager
        .add("High priority task".to_string(), "Description".to_string(), "high".to_string())
        .unwrap();

    assert_eq!(manager.tasks()[0].priority, TaskPriority::High);
}

#[test]
fn test_add_task_with_none_priority() {
    let mut manager = TaskManager::new();

    manager
        .add("Task without priority".to_string(), "Description".to_string(), "none".to_string())
        .unwrap();

    assert_eq!(manager.tasks()[0].priority, TaskPriority::None);
}

#[test]
fn test_add_task_with_invalid_priority_fails() {
    let mut manager = TaskManager::new();

    let result = manager.add(
        "Invalid priority task".to_string(),
        "Description".to_string(),
        "urgent".to_string()
    );

    assert_eq!(result, Err(TaskError::ParseTaskPriorityError("urgent".to_string())));

    assert!(manager.tasks().is_empty());
}

#[test]
fn test_invalid_priority_does_not_consume_task_id() {
    let mut manager = TaskManager::new();

    let result = manager.add(
        "Invalid priority task".to_string(),
        "Description".to_string(),
        "urgent".to_string()
    );

    assert!(result.is_err());

    manager
        .add("Valid priority task".to_string(), "Description".to_string(), "high".to_string())
        .unwrap();

    assert_eq!(manager.tasks().len(), 1);
    assert_eq!(manager.tasks()[0].id, 0);
}

#[test]
fn test_multiple_tasks_keep_individual_priorities() {
    let mut manager = TaskManager::new();

    manager
        .add("First task".to_string(), "First description".to_string(), "low".to_string())
        .unwrap();

    manager
        .add("Second task".to_string(), "Second description".to_string(), "high".to_string())
        .unwrap();

    manager
        .add("Third task".to_string(), "Third description".to_string(), "none".to_string())
        .unwrap();

    assert_eq!(manager.tasks().len(), 3);

    assert_eq!(manager.tasks()[0].priority, TaskPriority::Low);
    assert_eq!(manager.tasks()[1].priority, TaskPriority::High);
    assert_eq!(manager.tasks()[2].priority, TaskPriority::None);
}

// ===== Command::parse with priority =====

#[test]
fn test_parse_add_command_with_priority() {
    let command = Command::parse("add Report high").unwrap();

    assert_eq!(command, Command::Add {
        title: "Report".to_string(),
        description: String::new(),
        priority: "high".to_string(),
    });
}

#[test]
fn test_parse_add_command_with_priority_and_description() {
    let command = Command::parse("add Report high Finish the weekly report").unwrap();

    assert_eq!(command, Command::Add {
        title: "Report".to_string(),
        description: "Finish the weekly report".to_string(),
        priority: "high".to_string(),
    });
}

#[test]
fn test_parse_add_command_without_priority_uses_none() {
    let command = Command::parse("add Report").unwrap();

    assert_eq!(command, Command::Add {
        title: "Report".to_string(),
        description: String::new(),
        priority: "None".to_string(),
    });
}

#[test]
fn test_parse_add_command_preserves_priority_for_validation() {
    let command = Command::parse("add Report urgent").unwrap();

    assert_eq!(command, Command::Add {
        title: "Report".to_string(),
        description: String::new(),
        priority: "urgent".to_string(),
    });
}

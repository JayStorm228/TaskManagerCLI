use crate::errors::TaskError;
use crate::taskmanager::TaskManager;

// ===== new / tasks =====

#[test]
fn test_taskmanager_new_empty() {
    let mgr = TaskManager::new();
    assert!(mgr.tasks().is_empty());
}

#[test]
fn test_taskmanager_tasks_returns_slice() {
    let mut mgr = TaskManager::new();
    mgr.add("Task".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    let tasks = mgr.tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title, "Task");
}

// ===== add =====

#[test]
fn test_add_task_success() {
    let mut mgr = TaskManager::new();
    let res = mgr.add("Buy milk".to_string(), "2 liters".to_string(), "None".to_string());
    assert!(res.is_ok());
    assert_eq!(mgr.tasks().len(), 1);
    assert_eq!(mgr.tasks()[0].title, "Buy milk");
}

#[test]
fn test_add_task_short_title_error() {
    let mut mgr = TaskManager::new();
    let res = mgr.add("No".to_string(), "Desc".to_string(), "None".to_string());
    assert_eq!(res, Err(TaskError::ShortTitle(3)));
    assert!(mgr.tasks().is_empty());
}

#[test]
fn test_add_task_empty_title_error() {
    let mut mgr = TaskManager::new();
    let res = mgr.add("".to_string(), "Desc".to_string(), "None".to_string());
    assert_eq!(res, Err(TaskError::ShortTitle(3)));
}

#[test]
fn test_add_task_empty_description_allowed() {
    let mut mgr = TaskManager::new();
    let res = mgr.add("ValidTitle".to_string(), "".to_string(), "None".to_string());
    assert!(res.is_ok());
    assert_eq!(mgr.tasks().len(), 1);
    assert_eq!(mgr.tasks()[0].description, "");
}

#[test]
fn test_add_multiple_tasks_incremental_ids() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc 1".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc 2".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 3".to_string(), "Desc 3".to_string(), "None".to_string()).unwrap();

    assert_eq!(mgr.tasks().len(), 3);
    assert_eq!(mgr.tasks()[0].id, 0);
    assert_eq!(mgr.tasks()[1].id, 1);
    assert_eq!(mgr.tasks()[2].id, 2);
}

#[test]
fn test_add_task_after_delete_continues_id() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.delete(0).unwrap();
    mgr.add("Task 3".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    // ID продолжает инкрементиться, не перезаполняет удалённые
    assert_eq!(mgr.tasks().len(), 2);
    assert_eq!(mgr.tasks()[0].id, 1); // Task 2
    assert_eq!(mgr.tasks()[1].id, 2); // Task 3
}

// ===== search_id =====

#[test]
fn test_search_id() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc 1".to_string(), "None".to_string()).unwrap();

    let task = mgr.search_id(0).unwrap();
    assert_eq!(task.title, "Task 1");

    let err = mgr.search_id(999);
    assert_eq!(err, Err(TaskError::IDNotFound(999)));
}

#[test]
fn test_search_id_first_task() {
    let mut mgr = TaskManager::new();
    mgr.add("First".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    let task = mgr.search_id(0).unwrap();
    assert_eq!(task.id, 0);
    assert_eq!(task.title, "First");
}

#[test]
fn test_search_id_middle_task() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 3".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    let task = mgr.search_id(1).unwrap();
    assert_eq!(task.title, "Task 2");
}

#[test]
fn test_search_id_after_delete() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.delete(0).unwrap();

    let err = mgr.search_id(0);
    assert_eq!(err, Err(TaskError::IDNotFound(0)));

    let task = mgr.search_id(1).unwrap();
    assert_eq!(task.title, "Task 2");
}

#[test]
fn test_search_id_empty_manager() {
    let mgr = TaskManager::new();
    let err = mgr.search_id(0);
    assert_eq!(err, Err(TaskError::IDNotFound(0)));
}

// ===== complete =====

#[test]
fn test_complete_task() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc 1".to_string(), "None".to_string()).unwrap();

    assert!(mgr.complete(0).is_ok());
    assert!(mgr.tasks()[0].is_completed);

    // Повторное завершение возвращает ошибку AlreadyCompleted
    assert_eq!(mgr.complete(0), Err(TaskError::AlreadyCompleted));

    // Попытка завершить несуществующую задачу
    assert_eq!(mgr.complete(999), Err(TaskError::IDNotFound(999)));
}

#[test]
fn test_complete_first_task() {
    let mut mgr = TaskManager::new();
    mgr.add("First".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    let res = mgr.complete(0);
    assert!(res.is_ok());
    assert!(mgr.tasks()[0].is_completed);
}

#[test]
fn test_complete_middle_task() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 3".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    let res = mgr.complete(1);
    assert!(res.is_ok());
    assert!(mgr.tasks()[1].is_completed);
    assert!(!mgr.tasks()[0].is_completed);
    assert!(!mgr.tasks()[2].is_completed);
}

#[test]
fn test_complete_all_tasks() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    mgr.complete(0).unwrap();
    mgr.complete(1).unwrap();

    assert!(mgr.tasks()[0].is_completed);
    assert!(mgr.tasks()[1].is_completed);
}

#[test]
fn test_complete_empty_manager() {
    let mut mgr = TaskManager::new();
    let res = mgr.complete(0);
    assert_eq!(res, Err(TaskError::IDNotFound(0)));
}

// ===== delete =====

#[test]
fn test_delete_task() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc 1".to_string(), "None".to_string()).unwrap();

    assert!(mgr.delete(0).is_ok());
    assert!(mgr.tasks().is_empty());

    assert_eq!(mgr.delete(0), Err(TaskError::IDNotFound(0)));
}

#[test]
fn test_delete_first_task() {
    let mut mgr = TaskManager::new();
    mgr.add("First".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Second".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    mgr.delete(0).unwrap();
    assert_eq!(mgr.tasks().len(), 1);
    assert_eq!(mgr.tasks()[0].title, "Second");
}

#[test]
fn test_delete_middle_task() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 3".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    mgr.delete(1).unwrap();
    assert_eq!(mgr.tasks().len(), 2);
    assert_eq!(mgr.tasks()[0].title, "Task 1");
    assert_eq!(mgr.tasks()[1].title, "Task 3");
}

#[test]
fn test_delete_last_task() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    mgr.delete(1).unwrap();
    assert_eq!(mgr.tasks().len(), 1);
    assert_eq!(mgr.tasks()[0].title, "Task 1");
}

#[test]
fn test_delete_all_tasks() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    mgr.delete(0).unwrap();
    mgr.delete(1).unwrap();
    assert!(mgr.tasks().is_empty());
}

#[test]
fn test_delete_nonexistent_id() {
    let mut mgr = TaskManager::new();
    mgr.add("Task".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    let err = mgr.delete(999);
    assert_eq!(err, Err(TaskError::IDNotFound(999)));
    assert_eq!(mgr.tasks().len(), 1);
}

#[test]
fn test_delete_empty_manager() {
    let mut mgr = TaskManager::new();
    let err = mgr.delete(0);
    assert_eq!(err, Err(TaskError::IDNotFound(0)));
}

#[test]
fn test_delete_then_add_new_task() {
    let mut mgr = TaskManager::new();
    mgr.add("Task 1".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.delete(0).unwrap();
    mgr.add("Task 2".to_string(), "Desc".to_string(), "None".to_string()).unwrap();

    assert_eq!(mgr.tasks().len(), 1);
    assert_eq!(mgr.tasks()[0].id, 1); // ID продолжает расти
}

// ===== integration scenarios =====

#[test]
fn test_full_workflow() {
    let mut mgr = TaskManager::new();

    // Add
    mgr.add("Task 1".to_string(), "Desc 1".to_string(), "None".to_string()).unwrap();
    mgr.add("Task 2".to_string(), "Desc 2".to_string(), "None".to_string()).unwrap();
    assert_eq!(mgr.tasks().len(), 2);

    // Complete
    mgr.complete(0).unwrap();
    assert!(mgr.tasks()[0].is_completed);

    // Delete
    mgr.delete(1).unwrap();
    assert_eq!(mgr.tasks().len(), 1);

    // Search
    let task = mgr.search_id(0).unwrap();
    assert_eq!(task.title, "Task 1");
    assert!(task.is_completed);
}

#[test]
fn test_complete_then_delete() {
    let mut mgr = TaskManager::new();
    mgr.add("Task".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.complete(0).unwrap();
    mgr.delete(0).unwrap();

    assert!(mgr.tasks().is_empty());
}

#[test]
fn test_delete_then_complete_same_id_fails() {
    let mut mgr = TaskManager::new();
    mgr.add("Task".to_string(), "Desc".to_string(), "None".to_string()).unwrap();
    mgr.delete(0).unwrap();

    let res = mgr.complete(0);
    assert_eq!(res, Err(TaskError::IDNotFound(0)));
}

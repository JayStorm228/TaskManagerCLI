use crate::errors::TaskError;

#[test]
fn test_already_completed_display() {
    let err = TaskError::AlreadyCompleted;
    assert_eq!(err.to_string(), "Task is already completed");
}

#[test]
fn test_id_not_found_display() {
    let err = TaskError::IDNotFound(42);
    assert_eq!(err.to_string(), "Cannot find task with this ID: 42");
}

#[test]
fn test_short_title_display() {
    let err = TaskError::ShortTitle(3);
    assert_eq!(
        err.to_string(),
        "Title is too short! Must be at least 3 characters"
    );
}

#[test]
fn test_task_error_debug() {
    assert_eq!(
        format!("{:?}", TaskError::AlreadyCompleted),
        "AlreadyCompleted"
    );
    assert_eq!(format!("{:?}", TaskError::IDNotFound(7)), "IDNotFound(7)");
    assert_eq!(format!("{:?}", TaskError::ShortTitle(5)), "ShortTitle(5)");
}

#[test]
fn test_task_error_partial_eq() {
    assert_eq!(TaskError::AlreadyCompleted, TaskError::AlreadyCompleted);
    assert_ne!(TaskError::AlreadyCompleted, TaskError::IDNotFound(1));

    assert_eq!(TaskError::IDNotFound(10), TaskError::IDNotFound(10));
    assert_ne!(TaskError::IDNotFound(10), TaskError::IDNotFound(20));

    assert_eq!(TaskError::ShortTitle(4), TaskError::ShortTitle(4));
    assert_ne!(TaskError::ShortTitle(4), TaskError::ShortTitle(8));
}

#[test]
fn test_task_error_is_error() {
    let err: TaskError = TaskError::AlreadyCompleted;
    assert!(std::error::Error::source(&err).is_none());
}

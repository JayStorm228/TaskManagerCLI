use std::{fmt::Display, str::FromStr};

use super::errors::TaskError;

const SHORT_TITLE_LEN: u32 = 3;

/// Represents the priority assigned to a task.
///
/// The priority can be [`Low`](TaskPriority::Low),
/// [`Medium`](TaskPriority::Medium), [`High`](TaskPriority::High),
/// or [`None`](TaskPriority::None) when no priority is assigned.
#[derive(Debug, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    None,
}
/// Parses a task priority from a string.
///
/// The input is trimmed of leading and trailing whitespace and converted
/// to ASCII lowercase before parsing.
///
/// # Accepted Values
///
/// - `"low"`
/// - `"medium"`
/// - `"high"`
/// - `"none"`
///
/// # Errors
///
/// Returns [`TaskError::ParseTaskPriorityError`] if the input does not
/// match any accepted priority.
impl FromStr for TaskPriority {
    type Err = TaskError;
    /// Parses a string into a [`TaskPriority`].
    ///
    /// # Arguments
    ///
    /// * `s` - The string containing the priority name.
    ///
    /// # Returns
    ///
    /// Returns the parsed priority on success.
    ///
    /// # Errors
    ///
    /// Returns [`TaskError::ParseTaskPriorityError`] if the priority is unknown.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "low" => Ok(TaskPriority::Low),
            "medium" => Ok(TaskPriority::Medium),
            "high" => Ok(TaskPriority::High),
            "none" => Ok(TaskPriority::None),
            _ => Err(TaskError::ParseTaskPriorityError(s.to_string())),
        }
    }
}
impl Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TaskPriority::High => "high",
            TaskPriority::Medium => "medium",
            TaskPriority::Low => "low",
            TaskPriority::None => "none",
        };
        f.write_str(s)
    }
}
/// Represents a task managed by the application.
///
/// A task contains an identifier, a title, a description, a completion
/// status, and a priority.
///
/// Tasks are created using [`Task::new`] and can be marked as completed
/// using [`Task::mark_done`].
#[derive(PartialEq, Debug)]
pub struct Task {
    /// The unique identifier assigned to the task by the task manager.
    pub id: u32,

    /// The title of the task.
    pub title: String,

    /// The detailed description of the task.
    pub description: String,

    /// Indicates whether the task has been completed.
    pub is_completed: bool,

    /// The priority assigned to the task.
    pub priority: TaskPriority,
}

impl Task {
    /// Marks the task as completed.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the task was successfully marked as completed.
    ///
    /// # Errors
    ///
    /// Returns [`TaskError::AlreadyCompleted`] if the task is already completed.
    pub fn mark_done(&mut self) -> Result<(), TaskError> {
        if self.is_completed {
            Err(TaskError::AlreadyCompleted)
        } else {
            self.is_completed = true;
            Ok(())
        }
    }

    /// Creates a new task.
    ///
    /// The task is initialized with the provided identifier, title, description,
    /// and priority. Its completion status is set to `false`.
    ///
    /// If the description is `None`, it defaults to `"Не задано"`.
    /// The title must contain at least three Unicode characters.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier assigned to the task.
    /// * `title` - The task title.
    /// * `description` - An optional task description.
    /// * `priority` - The priority assigned to the task.
    ///
    /// # Returns
    ///
    /// Returns the newly created task if the title is valid.
    ///
    /// # Errors
    ///
    /// Returns [`TaskError::ShortTitle`] if the title contains fewer than
    /// three Unicode characters.
    pub fn new(
        id: u32,
        title: String,
        description: Option<String>,
        priority: TaskPriority,
    ) -> Result<Self, TaskError> {
        if title.chars().count() < (SHORT_TITLE_LEN as usize) {
            Err(TaskError::ShortTitle(SHORT_TITLE_LEN))
        } else {
            Ok(Self {
                id,
                title,
                description: description.unwrap_or_else(|| "Undefined".to_string()),
                is_completed: false,
                priority,
            })
        }
    }
}

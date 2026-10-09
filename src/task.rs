use std::str::FromStr;

use super::errors::TaskError;

const SHORT_TITLE_LEN: u32 = 3;
#[derive(Debug, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    None,
}
impl FromStr for TaskPriority {
    type Err = TaskError;
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
#[derive(PartialEq, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub priority: TaskPriority,
}

impl Task {
    pub fn mark_done(&mut self) -> Result<(), TaskError> {
        if self.is_completed {
            Err(TaskError::AlreadyCompleted)
        } else {
            self.is_completed = true;
            Ok(())
        }
    }

    pub fn new(
        id: u32,
        title: String,
        description: Option<String>,
        priority: TaskPriority
    ) -> Result<Self, TaskError> {
        if title.chars().count() < (SHORT_TITLE_LEN as usize) {
            Err(TaskError::ShortTitle(SHORT_TITLE_LEN))
        } else {
            Ok(Self {
                id,
                title,
                description: description.unwrap_or_else(|| "Не задано".to_string()),
                is_completed: false,
                priority,
            })
        }
    }
}

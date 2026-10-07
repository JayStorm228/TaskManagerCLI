use super::errors::TaskError;

const SHORT_TITLE_LEN: u32 = 3;

#[derive(PartialEq, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
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

    pub fn new(id: u32, title: String, description: Option<String>) -> Result<Self, TaskError> {
        if title.chars().count() < SHORT_TITLE_LEN as usize {
            Err(TaskError::ShortTitle(SHORT_TITLE_LEN))
        } else {
            Ok(Self {
                id,
                title,
                description: description.unwrap_or_else(|| "Не задано".to_string()),
                is_completed: false,
            })
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation_valid() {
        let task = Task::new(
            1,
            "Valid Title".to_string(),
            Some("Description".to_string()),
        )
        .unwrap();
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Valid Title");
        assert_eq!(task.description, "Description");
        assert!(!task.is_completed);
    }

    #[test]
    fn test_task_creation_default_description() {
        let task = Task::new(1, "Valid Title".to_string(), None).unwrap();
        assert_eq!(task.description, "Не задано");
    }

    #[test]
    fn test_task_creation_short_title() {
        let result = Task::new(1, "Hi".to_string(), None);
        assert_eq!(result, Err(TaskError::ShortTitle(3)));
    }

    #[test]
    fn test_mark_done_success() {
        let mut task = Task::new(1, "Test task".to_string(), None).unwrap();
        assert!(task.mark_done().is_ok());
        assert!(task.is_completed);
    }

    #[test]
    fn test_mark_done_already_completed() {
        let mut task = Task::new(1, "Test task".to_string(), None).unwrap();
        task.mark_done().unwrap();
        assert_eq!(task.mark_done(), Err(TaskError::AlreadyCompleted));
    }
}

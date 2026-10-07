use crate::errors::TaskError;

use super::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}
impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 0,
        }
    }
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
    pub fn add(&mut self, title: String, description: String) -> Result<(), TaskError> {
        let new_task = Task::new(self.next_id, title, Some(description))?;
        self.tasks.push(new_task);
        self.next_id += 1;
        Ok(())
    }

    pub fn delete(&mut self, id: u32) -> Result<(), TaskError> {
        if !self.tasks.iter().any(|t| t.id == id) {
            return Err(TaskError::IDNotFound(id));
        }
        self.tasks.retain(|t| t.id != id);
        Ok(())
    }
    pub fn complete(&mut self, id: u32) -> Result<(), TaskError> {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TaskError::IDNotFound(id))?
            .mark_done()
    }
    pub fn search_id(&self, id: u32) -> Result<&Task, TaskError> {
        self.tasks
            .iter()
            .find(|t| t.id == id)
            .ok_or(TaskError::IDNotFound(id))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task_success() {
        let mut mgr = TaskManager::new();
        let res = mgr.add("Buy milk".to_string(), "2 liters".to_string());
        assert!(res.is_ok());
        assert_eq!(mgr.tasks.len(), 1);
        assert_eq!(mgr.tasks[0].title, "Buy milk");
    }

    #[test]
    fn test_add_task_short_title_error() {
        let mut mgr = TaskManager::new();
        let res = mgr.add("No".to_string(), "Desc".to_string());
        assert_eq!(res, Err(TaskError::ShortTitle(3)));
        assert!(mgr.tasks.is_empty());
    }

    #[test]
    fn test_search_id() {
        let mut mgr = TaskManager::new();
        mgr.add("Task 1".to_string(), "Desc 1".to_string()).unwrap();

        let task = mgr.search_id(0).unwrap();
        assert_eq!(task.title, "Task 1");

        let err = mgr.search_id(999);
        assert_eq!(err, Err(TaskError::IDNotFound(999)));
    }

    #[test]
    fn test_complete_task() {
        let mut mgr = TaskManager::new();
        mgr.add("Task 1".to_string(), "Desc 1".to_string()).unwrap();

        assert!(mgr.complete(0).is_ok());
        assert!(mgr.tasks[0].is_completed);

        // Повторное завершение возвращает ошибку AlreadyCompleted
        assert_eq!(mgr.complete(0), Err(TaskError::AlreadyCompleted));

        // Попытка завершить несуществующую задачу
        assert_eq!(mgr.complete(999), Err(TaskError::IDNotFound(999)));
    }

    #[test]
    fn test_delete_task() {
        let mut mgr = TaskManager::new();
        mgr.add("Task 1".to_string(), "Desc 1".to_string()).unwrap();

        assert!(mgr.delete(0).is_ok());
        assert!(mgr.tasks.is_empty());

        assert_eq!(mgr.delete(0), Err(TaskError::IDNotFound(0)));
    }
}

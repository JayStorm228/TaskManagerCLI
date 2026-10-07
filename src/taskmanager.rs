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
    pub fn tasks(&self) -> &[Task] {
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

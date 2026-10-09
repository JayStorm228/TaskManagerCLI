use crate::errors::TaskError;

use crate::task::{Task, TaskPriority};

/// Manages the collection of tasks.
///
/// The task manager supports adding, deleting, searching for, and completing
/// tasks. It assigns identifiers to new tasks using an internal counter.
///
/// Task identifiers start at `0` and increase after each successful addition.
pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}
impl TaskManager {
    /// Creates an empty task manager.
    ///
    /// The internal task collection is initialized as empty, and the next task
    /// identifier is set to `0`.
    ///
    /// # Returns
    ///
    /// Returns a new [`TaskManager`] instance.
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 0,
        }
    }
    /// Returns an immutable slice of all managed tasks.
    ///
    /// The tasks are returned in their current storage order.
    ///
    /// # Returns
    ///
    /// A slice containing references to the manager's tasks.
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
    /// Adds a new task to the manager.
    ///
    /// The priority string is parsed into a [`TaskPriority`]. If both the
    /// priority and title are valid, the new task is added to the collection
    /// and the next identifier is incremented.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the new task.
    /// * `description` - The description of the new task.
    /// * `priority` - The priority name: `"low"`, `"medium"`, `"high"`, or `"none"`.
    ///
    /// # Errors
    ///
    /// Returns [`TaskError::ParseTaskPriorityError`] if the priority is invalid,
    /// or [`TaskError::ShortTitle`] if the title contains fewer than three
    /// Unicode characters.
    ///
    /// If an error occurs, the task is not added and the identifier counter
    /// is not incremented.
    pub fn add(
        &mut self,
        title: String,
        description: String,
        priority: String,
    ) -> Result<(), TaskError> {
        let priority: TaskPriority = priority.parse::<TaskPriority>()?;

        let new_task = Task::new(self.next_id, title, Some(description), priority)?;
        self.tasks.push(new_task);
        self.next_id += 1;
        Ok(())
    }
    /// Deletes a task by its identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the task to delete.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the task exists and is removed.
    ///
    /// # Errors
    ///
    /// Returns [`TaskError::IDNotFound`] if no task with the specified
    /// identifier exists.
    pub fn delete(&mut self, id: u32) -> Result<(), TaskError> {
        if !self.tasks.iter().any(|t| t.id == id) {
            return Err(TaskError::IDNotFound(id));
        }
        self.tasks.retain(|t| t.id != id);
        Ok(())
    }
    /// Marks a task as completed.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the task to complete.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the task is successfully marked as completed.
    ///
    /// # Errors
    ///
    /// Returns [`TaskError::IDNotFound`] if no task with the specified
    /// identifier exists, or [`TaskError::AlreadyCompleted`] if the task
    /// has already been completed.
    pub fn complete(&mut self, id: u32) -> Result<(), TaskError> {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TaskError::IDNotFound(id))?
            .mark_done()
    }
    /// Searches for a task by its identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier of the task to find.
    ///
    /// # Returns
    ///
    /// Returns an immutable reference to the matching task.
    ///
    /// # Errors
    ///
    /// Returns [`TaskError::IDNotFound`] if no task with the specified
    /// identifier exists.
    pub fn search_id(&self, id: u32) -> Result<&Task, TaskError> {
        self.tasks
            .iter()
            .find(|t| t.id == id)
            .ok_or(TaskError::IDNotFound(id))
    }
}

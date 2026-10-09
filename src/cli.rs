use std::process::exit;

use crate::{ command::Command, errors::TaskError, taskmanager::TaskManager };

/// Provides the command-line interface for the task manager.
///
/// The CLI receives parsed commands, delegates task operations to
/// [`TaskManager`], and displays results and task information to the user.
pub struct Cli {
    task_mgr: TaskManager,
}
impl Cli {
    /// Creates a new command-line interface.
    ///
    /// The interface is initialized with an empty [`TaskManager`].
    ///
    /// # Returns
    ///
    /// Returns a new [`Cli`] instance.
    pub fn new() -> Self {
        Self {
            task_mgr: TaskManager::new(),
        }
    }

    /// Returns an immutable reference to the underlying task manager.
    ///
    /// This method is available only in test builds and allows tests to inspect
    /// the current task state without modifying it.
    ///
    /// # Returns
    ///
    /// An immutable reference to the [`TaskManager`].
    #[cfg(test)]
    pub fn task_mgr(&self) -> &TaskManager {
        &self.task_mgr
    }

    /// Executes a parsed command.
    ///
    /// Dispatches the supplied command to the corresponding operation.
    /// Commands that operate on tasks may print information or report errors.
    /// The `Exit` command terminates the process.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the operation completes successfully.
    ///
    /// # Errors
    ///
    /// Returns a [`TaskError`] if a task operation fails, for example because
    /// the requested task does not exist or has already been completed.
    ///
    /// # Panics
    ///
    /// This method does not explicitly panic, but the `Exit` command terminates
    /// the process instead of returning.
    pub fn execute(&mut self, command: Command) -> Result<(), TaskError> {
        match command {
            Command::Add { title, description, priority } => {
                self.new_task(title, description, priority)?;
                Ok(())
            }
            Command::Complete { id } => {
                self.complete_task(id)?;
                Ok(())
            }
            Command::Delete { id } => {
                self.delete_task(id)?;
                Ok(())
            }
            Command::Exit => exit(0),
            Command::Help => {
                Cli::help();
                Ok(())
            }
            Command::List => {
                self.list_tasks();
                Ok(())
            }
            Command::Show { id } => {
                self.show_task(id)?;
                Ok(())
            }
        }
    }
    fn new_task(
        &mut self,
        title: String,
        description: String,
        priority: String
    ) -> Result<(), TaskError> {
        println!("Trying to add task");
        println!("Title: {title}, description:\n{description}");

        self.task_mgr.add(title, description, priority)?;

        println!("\u{2713} Success!");
        Ok(())
    }
    fn show_task(&self, id: u32) -> Result<(), TaskError> {
        let task = self.task_mgr.search_id(id)?;
        println!("\n====== Task {}:{} ======", task.id, task.title);
        println!("Priority: {}", task.priority);
        println!("{}", task.description);
        println!("--------------------------");
        if task.is_completed {
            println!("\u{2713} - Done!");
        } else {
            println!("\u{2717} - Waiting to be done!");
        }
        println!("==========================\n");
        Ok(())
    }
    fn list_tasks(&self) {
        if self.task_mgr.tasks().is_empty() {
            println!("No tasks!")
        } else {
            for task in self.task_mgr.tasks().iter() {
                if task.is_completed {
                    println!("[{}] - {} - \u{2713}", task.id, task.title);
                } else {
                    println!("[{}] - {} - \u{2717}", task.id, task.title);
                }
            }
            println!("For more information consider using \"taskmanager show <id>\"")
        }
    }
    fn delete_task(&mut self, id: u32) -> Result<(), TaskError> {
        let task = self.task_mgr.search_id(id)?;
        println!("Deleting task:\n{}--{}", task.id, task.title);
        self.task_mgr.delete(id)?;
        Ok(())
    }
    fn complete_task(&mut self, id: u32) -> Result<(), TaskError> {
        let task = self.task_mgr.search_id(id)?;
        println!("Task Complete:\n{}--{}", task.id, task.title);
        self.task_mgr.complete(id)?;
        Ok(())
    }
    fn help() {
        println!("{:<10} {:<24} Description", "Command", "Arguments");

        println!("{}", "-".repeat(80));

        println!(
            "{:<10} {:<24} Add a new task. The description is optional and defaults to .",
            "add",
            "<title> <priority> [description]"
        );

        println!("{:<10} {:<24} Display all tasks with their statuses.", "list", "—");

        println!("{:<10} {:<24} Mark a task as completed by its ID.", "complete", "<id>");

        println!("{:<10} {:<24} Delete a task by its ID", "delete", "<id>");

        println!("{:<10} {:<24} Display detailed information about a task.", "show", "<id>");

        println!("{:<10} {:<24} Display the list of available commands.", "help", "—");
        println!("{:<10} {:<24} Exit the program. Drops all stored information", "exit", "—")
    }
}

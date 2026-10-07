use crate::{errors::TaskError, taskmanager::TaskManager};

pub struct Cli {
    pub task_mgr: TaskManager,
}
impl Cli {
    pub fn new_task(&mut self, title: String, description: String) -> Result<(), TaskError> {
        println!("Trying to add task");
        println!("Title: {title}, description:\n{description}");

        self.task_mgr.add(title, description)?;

        println!("\u{2713} Success!");
        Ok(())
    }
    pub fn show_task(&self, id: u32) -> Result<(), TaskError> {
        let task = self.task_mgr.search_id(id)?;
        println!("\n====== Task {}:{} ======", task.id, task.title);
        println!("{}", task.description);
        println!("--------------------------");
        if task.is_completed {
            println!("\u{2713} - Done!")
        } else {
            println!("\u{2717} - Waiting to be done!")
        }
        println!("==========================\n");
        Ok(())
    }
    pub fn list_tasks(&self) {
        if self.task_mgr.tasks.is_empty() {
            println!("No tasks!")
        } else {
            for task in self.task_mgr.tasks.iter() {
                if task.is_completed {
                    println!("[{}] - {} - \u{2713}", task.id, task.title)
                } else {
                    println!("[{}] - {} - \u{2717}", task.id, task.title)
                }
            }
            println!("For more information consider using \"taskmanager show <id>\"")
        }
    }
    pub fn delete_task(&mut self, id: u32) -> Result<(), TaskError> {
        let task = self.task_mgr.search_id(id)?;
        println!("Deleting task:\n{}--{}", task.id, task.title);
        self.task_mgr.delete(id)?;
        Ok(())
    }
    pub fn complete_task(&mut self, id: u32) -> Result<(), TaskError> {
        let task = self.task_mgr.search_id(id)?;
        println!("Task Comlpete:\n{}--{}", task.id, task.title);
        self.task_mgr.complete(id)?;
        Ok(())
    }
    pub fn help() {
        println!("{:<10} {:<24} {}", "Command", "Arguments", "Description");

        println!("{}", "-".repeat(80));

        println!(
            "{:<10} {:<24} {}",
            "add",
            "<title> [description]",
            "Add a new task. The description is optional and defaults to an empty string."
        );

        println!(
            "{:<10} {:<24} {}",
            "list", "—", "Display all tasks with their statuses."
        );

        println!(
            "{:<10} {:<24} {}",
            "complete", "<id>", "Mark a task as completed by its ID."
        );

        println!(
            "{:<10} {:<24} {}",
            "delete", "<id>", "Delete a task by its ID."
        );

        println!(
            "{:<10} {:<24} {}",
            "show", "<id>", "Display detailed information about a task."
        );

        println!(
            "{:<10} {:<24} {}",
            "help", "—", "Display the list of available commands."
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn create_cli() -> Cli {
        Cli {
            task_mgr: TaskManager::new(),
        }
    }

    #[test]
    fn test_cli_new_task() {
        let mut cli = create_cli();
        let res = cli.new_task("Title".to_string(), "Description".to_string());
        assert!(res.is_ok());
        assert_eq!(cli.task_mgr.tasks.len(), 1);
    }

    #[test]
    fn test_cli_complete_task() {
        let mut cli = create_cli();
        cli.new_task("Title".to_string(), "Description".to_string())
            .unwrap();

        assert!(cli.complete_task(0).is_ok());
        assert!(cli.task_mgr.tasks[0].is_completed);
    }

    #[test]
    fn test_cli_delete_task() {
        let mut cli = create_cli();
        cli.new_task("Title".to_string(), "Description".to_string())
            .unwrap();

        assert!(cli.delete_task(0).is_ok());
        assert!(cli.task_mgr.tasks.is_empty());
    }

    #[test]
    fn test_cli_not_found_errors() {
        let mut cli = create_cli();
        assert_eq!(cli.show_task(42), Err(TaskError::IDNotFound(42)));
        assert_eq!(cli.complete_task(42), Err(TaskError::IDNotFound(42)));
        assert_eq!(cli.delete_task(42), Err(TaskError::IDNotFound(42)));
    }
}

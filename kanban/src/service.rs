use chrono::Utc;

use crate::domain;
use crate::{
    domain::{Status, Task},
    repository::TaskRepository,
};

pub struct TaskService<R: TaskRepository> {
    repo: R,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn add_task(&mut self, name: &str, desc: &str) -> Result<&Task, String> {
        // initialize a new task object
        let task = domain::Task::new(name.to_string(), desc.to_string());
        // perform validations
        task.before_add()?;

        Ok(self.repo.add_task(task)?)
    }

    pub fn move_to_doing(&mut self, id: u32) -> Result<(), String> {
        let task = self.repo.find_by_id(id);
        if task.is_none() {
            return Err("No task found".to_string());
        }

        let task = task.unwrap();
        task.before_move_to_doing()?;

        self.repo.move_to_doing(id)?;

        Ok(())
    }

    pub fn move_to_done(&mut self, id: u32) -> Result<(), String> {
        let task = self.repo.find_by_id(id);
        if task.is_none() {
            return Err("No task found".to_string());
        }

        let task = task.unwrap();
        task.before_move_to_done()?;

        self.repo.move_to_done(id)?;

        Ok(())
    }

    pub fn list_by_status(&mut self, status: Status) -> Vec<&Task> {
        self.repo.list_by_status(status)
    }
}

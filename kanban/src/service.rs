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

    pub fn find_by_id(&mut self, id: u32) -> Option<&mut Task> {
        match self.repo.find_by_id(id) {
            Some(task) => Some(task),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::InMemoryTaskRepository;
    use super::*;

    struct Setup {
        svc: TaskService<InMemoryTaskRepository>,
    }
    const TASK_NAME1: &str = "task1";
    const TASK_NAME2: &str = "task2";
    const TASK_NAME3: &str = "task3";
    const TASK_DESCRIPTION1: &str = "description1";
    const TASK_DESCRIPTION2: &str = "description2";
    const TASK_DESCRIPTION3: &str = "description3";

    // task ids
    // TODO: ensure ID is i32 and not u32(can accept negative integers)
    const TASK1_ID: u32 = 1;
    const TASK2_ID: u32 = 2;
    const TASK3_ID: u32 = 3;

    impl Setup {
        fn new() -> Self {
            let mut svc = TaskService::new(InMemoryTaskRepository::new());

            // seed tasks
            svc.add_task(TASK_NAME1, TASK_DESCRIPTION1).expect("task not created");
            svc.add_task(TASK_NAME2, TASK_DESCRIPTION2).expect("task not created");
            svc.add_task(TASK_NAME3, TASK_DESCRIPTION3).expect("task not created");

            // move task2 and task3 to Doing state
            let _ = svc.repo.move_to_doing(TASK2_ID);
            let _ = svc.repo.move_to_doing(TASK3_ID);
            // move task3 to Done state
            let _ = svc.repo.move_to_done(TASK3_ID);
            Setup{
                svc,
            }
        }

    }
    #[test]
    fn test_add_task_succeeds() {
        let mut setup = Setup::new();
        let new_task_name = "new task";
        let new_description = "new description";
        let new_task = setup.svc.add_task(new_task_name, "new task description");
        assert!(new_task.is_ok());

        let new_task = new_task.unwrap();
        assert_eq!(new_task.name, new_task_name);
    }

    #[test]
    fn test_add_task_fails_with_duplicate_task_name() {
        let mut setup = Setup::new();
        let res = setup.svc.add_task(TASK_NAME1, TASK_DESCRIPTION1);
        assert!(res.is_err());

        let err = res.expect_err("should return an error");
        assert_eq!(err.as_str(), format!("Task with name '{}' already exists", TASK_NAME1));
    }

    #[test]
    fn validation_errors_are_caught() {
        let mut setup = Setup::new();
        let task1 = Task::new("".to_string(), "description1".to_string());
        let res = setup.svc.add_task("", "description");
        assert!(res.is_err());

        let err = res.expect_err("should return an error");
        assert_eq!(err.as_str(), "Task name is required");
    }

    #[test]
    fn move_to_doing_succeeds() {
        let mut setup = Setup::new();
        let res = setup.svc.move_to_doing(TASK1_ID);
        assert!(res.is_ok());

        // find task
        let task1 = setup.svc.find_by_id(TASK1_ID).unwrap();
        // confirm it moved to Doing state
        assert_eq!(task1.status, Status::Doing);
    }

    #[test]
    fn move_to_doing_fails() {
        let mut setup = Setup::new();
        // task of id TASK3_ID(3) is already in the done state
        // cannot move to doing state
        let res = setup.svc.move_to_doing(TASK3_ID);
        assert!(res.is_err());

        let err = res.expect_err("should return an error");
        assert_eq!(err.as_str(), "Task must be in the Todo state before marking as in progress");
    }

    #[test]
    fn move_to_done_succeeds() {
        let mut setup = Setup::new();
        // create task
        let res = setup.svc.move_to_done(TASK2_ID);
        assert!(res.is_ok());

        // find task
        let task2 = setup.svc.find_by_id(TASK2_ID).unwrap();
        // confirm it moved to Done state
        assert_eq!(task2.status, Status::Done);
    }

    #[test]
    fn move_to_done_fails() {
        let mut setup = Setup::new();
        // task of id TASK3_ID(3) is already in the done state
        // ONLY tasks in progress(Doing state) can be marked as Done
        let res = setup.svc.move_to_done(TASK1_ID);
        assert!(res.is_err());

        let err = res.expect_err("should return an error");
        assert_eq!(err.as_str(), "Task must be in progress state before marking as Done");
    }
}

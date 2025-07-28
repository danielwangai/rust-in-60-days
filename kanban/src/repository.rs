use chrono::Utc;
use crate::domain::{Status, Task};

/// Trait defining the behavior of a Task repository.
pub trait TaskRepository {
    fn add_task(&mut self, task: Task) -> Result<&Task, String>;
    fn update(&mut self, task: Task);
    fn move_to_doing(&mut self, id: u32) -> Result<(), String>;
    fn move_to_done(&mut self, id: u32) -> Result<(), String>;
    fn list_by_status(&self, status: Status) -> Vec<&Task>;
    fn find_by_id(&mut self, id: u32) -> Option<&mut Task>;
    fn find_by_name(&mut self, name: &str) -> Option<&mut Task>;
}

/// In-memory implementation of a Task repository.
/// Stores tasks in a vector.
pub struct InMemoryTaskRepository {
    tasks: Vec<Task>,
}

impl InMemoryTaskRepository {
    /// Creates a new empty task repository.
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
}

impl TaskRepository for InMemoryTaskRepository {
    /// Adds a new task to the repository.
    ///
    /// # Arguments
    /// * `task` - The task to be added.
    ///
    /// # Returns
    /// * `Ok(&Task)` - A reference to the newly added task.
    /// * `Err(String)` - If a task with the same name already exists.
    fn add_task(&mut self, mut task: Task) -> Result<&Task, String> {
        // ensure task uniqueness
        let t = self.find_by_name(task.name.as_str());
        if let Some(t) = t {
            return Err(format!("Task with name '{}' already exists", t.name));
        }

        // Assign task ID and push to vector
        task.id = Some((self.tasks.len() + 1) as u32);
        self.tasks.push(task);

        Ok(self.tasks.last().unwrap())
    }

    /// Updates an existing task by replacing it in the vector.
    ///
    /// # Arguments
    /// * `task` - The updated task.
    fn update(&mut self, task: Task) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == task.id) {
            self.tasks[pos] = task;
        }
    }

    /// Transitions a task to `Doing`.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the task.
    ///
    /// # Returns
    /// * `Ok(())` - If the task was found and updated successfully.
    /// * `Err(String)` - If the task could not be found.
    fn move_to_doing(&mut self, id: u32) -> Result<(), String> {
        let task = self.find_by_id(id);
        if task.is_none() {
            return Err("Task not found".to_string());
        }

        let task = task.unwrap();
        if task.status != Status::Todo {
            return Err(String::from("task must be in Todo state to move to in progress"));
        }

        task.status = Status::Doing;
        task.updated_at = Some(Utc::now());

        Ok(())
    }

    /// Transitions a task to `Done`.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the task.
    ///
    /// # Returns
    /// * `Ok(())` - If the task was found and marked as done.
    /// * `Err(String)` - If the task could not be found.
    fn move_to_done(&mut self, id: u32) -> Result<(), String> {
        let task = self.find_by_id(id);
        if task.is_none() {
            return Err("Task not found".to_string());
        }

        let task = task.unwrap();
        if task.status != Status::Doing {
            return Err(String::from("task must be in progress to mark as complete"));
        }

        task.status = Status::Done;
        task.updated_at = Some(Utc::now());

        Ok(())
    }

    /// Lists tasks by their current status.
    ///
    /// # Arguments
    /// * `status` - The status to filter by.
    ///
    /// # Returns
    /// * `Vec<&Task>` - A list of tasks matching the given status.
    ///   If status is `Status::None`, all tasks are returned.
    fn list_by_status(&self, status: Status) -> Vec<&Task> {
        // if status is None list all tasks
        if status == Status::None {
            return self.tasks.iter().collect();
        }

        // otherwise list by status
        self.tasks.iter().filter(|t| t.status == status).collect()
    }

    /// Finds a task by its unique ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the task to retrieve.
    ///
    /// # Returns
    /// * `Some(&mut Task)` - A mutable reference to the task, if found.
    /// * `None` - If no task with that ID exists.
    fn find_by_id(&mut self, id: u32) -> Option<&mut Task> {
        // return the position of the task in storage vector
        // NOTE: positions start from 1 not 0 like index
        self.tasks.iter_mut().find(|t| t.id == Option::from(id))
    }

    /// Searches for a task by name (case-insensitive).
    ///
    /// # Arguments
    /// * `name` - The task name to search for.
    ///
    /// # Returns
    /// * `Some(&mut Task)` - A mutable reference to the task, if found.
    /// * `None` - If no task with that name exists.
    fn find_by_name(&mut self, name: &str) -> Option<&mut Task> {
        self.tasks
            .iter_mut()
            .find(|t| t.name.to_lowercase() == name.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TASK_NAME: &str = "task1";
    const TASK_DESCRIPTION: &str = "description1";
    struct Setup {
        repo: InMemoryTaskRepository,
    }

    impl Setup {
        fn new() -> Self {
            let mut repo = InMemoryTaskRepository::new();

            // add test data
            let task1 = Task::new(TASK_NAME.to_string(), TASK_DESCRIPTION.to_string());
            let task2 = Task::new("task 2".to_string(), "description task 2".to_string());
            let task3 = Task::new("task 3".to_string(), "description task 3".to_string());

            repo.add_task(task1);
            repo.add_task(task2);
            repo.add_task(task3);

            let task2 = &mut repo.tasks[1];
            task2.status = Status::Doing;

            let task3 = &mut repo.tasks[2];
            task3.status = Status::Done;

            Setup { repo }
        }
    }

    #[test]
    fn add_task_succeeds() {
        let mut setup = Setup::new();

        // insert new task
        let no_of_tasks_before = setup.repo.tasks.iter().clone().len();

        let task_name = "new task";
        let task_descr = "new task description";

        // position of created task in the tasks vector
        let new_task = Task::new(task_name.to_string(), task_descr.to_string());
        let new_task_copy = new_task.clone();
        setup.repo.add_task(new_task).expect("TODO: panic message");

        let no_of_tasks_after = setup.repo.tasks.len();

        // Assertions
        assert_eq!(setup.repo.tasks.last().unwrap().name, task_name.to_string());
        assert!(no_of_tasks_after > no_of_tasks_before);
        assert_eq!(new_task_copy.name, task_name.to_string());
    }
}

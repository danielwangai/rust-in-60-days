use chrono::{DateTime, Utc};
use std::cmp::PartialEq;

/// Represents the possible states of a task during it's lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// task has been created but not yet started
    Todo,
    /// task in progress
    Doing,
    /// task completed
    Done,
}

/// Represents the properties of a struct
#[derive(Debug)]
pub struct Task {
    /// unique identifier
    pub id: u32,
    /// task name/title
    pub name: String,
    /// detailed description of the task
    pub description: String, // optional field. Can be updated later
    /// current status of the task
    pub status: Status, // default is Todo set during creation of new task
    /// when the task was created
    pub created_at: DateTime<Utc>,
    /// when the task was last updated
    pub updated_at: Option<DateTime<Utc>>,
}

impl Task {
    /// Creates a new task with the given parameters.
    ///
    /// # Arguments
    /// * `id`: Unique identifier for the task
    /// * `name`: Name of the task
    /// * `desc`: Optional description of the task
    ///
    /// # Returns
    /// A new Task instance with:
    /// - Status set to Todo
    /// - Current UTC timestamp for creation
    /// - No update timestamp
    fn new(id: u32, name: String, description: String) -> Self {
        Task {
            id,
            name,
            description,
            status: Status::Todo,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}

/// application structure
pub struct App {
    /// collection of all tasks
    pub tasks: Vec<Task>,
}

impl App {
    /// Creates a new empty application instance.
    ///
    /// # Returns
    /// A new App instance with an empty task list
    pub fn new() -> Self {
        App { tasks: Vec::new() }
    }

    /// Adds a new task to the application's task list.
    ///
    /// # Arguments
    /// * `name`: The name of the task to add
    /// * `desc`: The description of the task
    ///
    /// # Returns
    /// * `Ok(&Task)`: A reference to the newly created task
    /// * `Err(String)`: An error message if a task with the same name already exists
    ///
    /// # Errors
    /// Returns an error if a task with the same name already exists in the system.
    pub fn add_task(&mut self, name: &str, desc: &str) -> Result<u32, String> {
        // Check for existing task
        if let Some(pos) = self.find_task_by_name(name) {
            if let Some(t) = self.tasks.get(pos) {
                return Err(format!("The task '{}' already exists", t.name));
            }
        }

        let id = (self.tasks.len() + 1) as u32;
        // Create a new task and store it in a variable
        let task = Task::new(id, name.to_string(), desc.to_string());

        // Push a reference to the task onto the vector
        self.tasks.push(task);

        // Return a reference to the task
        Ok(id)
    }

    /// Moves a task from its current state to the `Doing` state.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the task to move.
    ///
    /// # Returns
    /// * `Ok(&Task)` - A reference to the updated task if the operation succeeds.
    /// * `Err(String)` - An error message if:
    ///    - The task does not exist (invalid `id`).
    ///    - The task is already in the `Done` state (cannot move back to Doing).
    ///
    /// # Panics
    /// This function calls `unwrap()` on the result of `find_task_by_id`.
    /// If the task does not exist, this will panic.
    /// **Consider handling the `Option` gracefully instead of unwrapping in production code.**
    pub fn move_to_doing(&mut self, id: u32) -> Result<&Task, String> {
        // Find the position of the task with the given ID in the `tasks` vector.
        let pos = self.find_task_by_id(id).unwrap();

        // Get a mutable reference to the task at this position.
        let task = &mut self.tasks[pos];

        // If the task is already completed, we cannot move it back to Doing.
        if task.status == Status::Done {
            return Err(format!("The task of ID '{}' is already completed", id));
        }

        // Update the task status and the updated timestamp.
        task.status = Status::Doing;
        task.updated_at = Some(Utc::now());

        // Return a reference to the updated task.
        Ok(task)
    }

    fn move_to_done(&mut self, id: u32) -> Result<&Task, String> {
        let pos = self
            .find_task_by_id(id)
            .ok_or_else(|| format!("The task '{}' does not exist", id))?;

        let task = self.tasks.get_mut(pos).unwrap();
        if task.status != Status::Doing {
            return Err(format!("The task of ID '{}' not currently in progress", id));
        }

        task.status = Status::Done;
        task.updated_at = Some(Utc::now());

        Ok(task)
    }

    fn list_todo_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.status == Status::Todo)
            .collect()
    }

    fn list_doing_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.status == Status::Doing)
            .collect()
    }

    fn list_completed_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.status == Status::Done)
            .collect()
    }

    fn list_all_tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Searches for a task by its name in a case-insensitive manner.
    ///
    /// # Arguments
    /// * `name`: The name of the task to search for
    ///
    /// # Returns
    /// * `Some(&Task)`: A reference to the task if found
    /// * `None`: If no matching task is found
    ///
    /// # Note
    /// The search is case-insensitive, so "Task1" and "task1" are considered the same name.
    pub fn find_task_by_name(&self, name: &str) -> Option<usize> {
        self.tasks
            .iter()
            .position(|t| t.name.to_lowercase() == name.to_lowercase())
    }

    /// Searches for a task by its unique ID.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the task to search for.
    ///
    /// # Returns
    /// * `Some(usize)` - The position (index) of the task in the internal `tasks` vector.
    /// * `None` - If no task with the given ID exists.
    ///
    /// # Note
    /// The position returned here is a **0-based index**, which is how elements are stored in `Vec`.
    pub fn find_task_by_id(&self, id: u32) -> Option<usize> {
        // return the position of the task in storage vector
        // NOTE: positions start from 1 not 0 like index
        self.tasks.iter().position(|t| t.id == id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{App, Status};

    const TASK_NAME: &str = "task1";
    const TASK_DESCRIPTION: &str = "description1";
    struct Setup {
        app: App,
    }

    impl Setup {
        fn new() -> Self {
            let mut app = App::new();
            // add test data
            let _ = app.add_task(TASK_NAME, TASK_DESCRIPTION);
            let _ = app.add_task("task 2", "description task 2");
            let _ = app.add_task("task 3", "description task 3");

            let pos2 = app.find_task_by_id(2).unwrap();
            let task2 = &mut app.tasks[pos2];
            task2.status = Status::Doing;

            let pos3 = app.find_task_by_id(3).unwrap();
            let task3 = &mut app.tasks[pos3];
            task3.status = Status::Done;

            Setup { app }
        }
    }

    #[test]
    fn it_works() {
        // setup is initialized here with test variables
        let mut setup = Setup::new();

        // insert another task
        let no_of_tasks_before = setup.app.tasks.iter().clone().len();

        let task_name = "another one";
        let task_descr = "description2";

        // position of created task in the tasks vector
        let task_pos = setup.app.add_task(task_name, task_descr).unwrap() as usize;

        // minus one to convert to index because task_pos is the position which counts from 1 unlike
        // index which starts from zero
        let task = &setup.app.tasks[task_pos - 1];
        let no_of_tasks_after = setup.app.tasks.len();

        // Assertions
        assert_eq!(setup.app.tasks.last().unwrap().name, task_name.to_string());
        assert!(no_of_tasks_after > no_of_tasks_before);
        assert_eq!(task.name, task_name.to_string());
    }

    #[test]
    fn add_task_rejects_duplicate_name() {
        let mut setup = Setup::new();
        let res = setup.app.add_task(TASK_NAME, TASK_DESCRIPTION);

        assert_eq!(res.is_err(), true);
        let err = res.expect_err("Should have returned an error");
        assert_eq!(err.as_str(), "The task 'task1' already exists");
    }

    #[test]
    fn find_task_by_name_succeeds() {
        let mut setup = Setup::new();

        let pos1 = setup.app.find_task_by_name(TASK_NAME).unwrap();
        let task1 = &mut setup.app.tasks[pos1];
        assert_eq!(task1.name, TASK_NAME.to_string());
    }

    #[test]
    fn move_task_to_doing_state_succeeds() {
        // select task to be moved to Doing state
        let mut setup = Setup::new();

        // move to doing state
        let updated_task1 = setup.app.move_to_doing(1).unwrap();
        assert_eq!(updated_task1.status, Status::Doing)
    }

    #[test]
    fn move_task_to_doing_state_fails() {
        // select task to be moved to Doing state
        let mut setup = Setup::new();

        // move to doing state
        let res = setup.app.move_to_doing(3);
        assert!(res.is_err());
        let err = res.expect_err("Should have returned an error");
        assert_eq!(
            err.as_str(),
            format!("The task of ID '{}' is already completed", 3)
        );
    }

    #[test]
    fn move_task_to_done_state_succeeds() {
        // select task to be moved to Doing state
        let mut setup = Setup::new();

        // move to doing state
        let updated_task1 = setup.app.move_to_done(2).unwrap();
        assert_eq!(updated_task1.status, Status::Done)
    }

    #[test]
    fn move_task_to_done_state_fails() {
        // select task to be moved to Doing state
        let mut setup = Setup::new();

        // task of id 1 is in the todo state
        let res = setup.app.move_to_done(1);
        assert!(res.is_err());

        // fetch task of ID: 1 and confirm its still in the todo state
        let pos = setup.app.find_task_by_id(1).unwrap();
        let task = setup.app.tasks.get(pos).unwrap();
        assert_eq!(task.status, Status::Todo);
    }

    #[test]
    fn list_todo_tasks() {
        let setup = Setup::new();
        let tasks = setup.app.list_todo_tasks();
    }

    #[test]
    fn list_doing_tasks() {
        let setup = Setup::new();
        let tasks = setup.app.list_doing_tasks();
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn list_completed_tasks() {
        let setup = Setup::new();
        let tasks = setup.app.list_completed_tasks();
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn list_all_tasks() {
        let setup = Setup::new();
        let tasks = setup.app.list_all_tasks();
        assert_eq!(tasks.len(), 3);
    }
}

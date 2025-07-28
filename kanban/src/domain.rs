use chrono::{DateTime, Utc};

/// Represents the possible states of a task during it's lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// task has been created but not yet started
    Todo,
    /// task in progress
    Doing,
    /// task completed
    Done,
    /// invalid status
    None,
}

/// Represents the properties of a struct
#[derive(Debug, Clone)]
pub struct Task {
    /// unique identifier
    pub id: Option<u32>,
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
    pub fn new(name: String, description: String) -> Self {
        Task {
            id: None,
            name,
            description,
            status: Status::Todo,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    // validations for a new task
    pub fn before_add(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err(String::from("Task name is required"))
        }
        if self.status != Status::Todo {
            return Err(String::from("New task must be in todo state"))
        }

        Ok(())
    }

    pub fn before_move_to_doing(&self) -> Result<(), String> {
        if self.status != Status::Todo {
            return Err(String::from("New task must be in the Todo state"))
        }

        Ok(())
    }

    pub fn before_move_to_done(&self) -> Result<(), String> {
        if self.status != Status::Doing {
            return Err(String::from("New task must be in progress to mark as complete"))
        }

        Ok(())
    }

    fn move_to_doing(&mut self) -> Result<&mut Self, String> {
        match self.status {
            Status::Todo => {
                self.status = Status::Doing;
                Ok(self)
            }
            _ => Err(String::from("Task must be in Todo state to move to in progress")),
        }
    }

    fn move_to_done(&mut self) -> Result<&mut Self, String> {
        match self.status {
            Status::Doing => {
                self.status = Status::Done;
                Ok(self)
            }
            _ => Err(String::from("Task must be in progress state to mark as completed")),
        }
    }
}
use chrono::{DateTime, Utc};

/// Represents the possible states of a task during it's lifecycle
#[derive(Debug)]
enum Status {
    /// task has been created but not yet started
    Todo,
    /// task in progress
    Doing,
    /// task completed
    Done,
}

/// Represents the properties of a struct
#[derive(Debug)]
struct Task {
    /// unique identifier
    id: u32,
    /// task name/title
    name: String,
    /// detailed description of the task
    description: String, // optional field. Can be updated later
    /// current status of the task
    status: Status, // default is Todo set during creation of new task
    /// when the task was created
    created_at: DateTime<Utc>,
    /// when the task was last updated
    updated_at: Option<DateTime<Utc>>,
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
struct App {
    /// collection of all tasks
    tasks: Vec<Task>,
}

impl App {
    /// Creates a new empty application instance.
    ///
    /// # Returns
    /// A new App instance with an empty task list
    fn new() -> Self {
        App { tasks: Vec::new() }
    }
}

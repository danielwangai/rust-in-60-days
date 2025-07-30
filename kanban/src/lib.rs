pub mod domain;
pub mod inmemory_repository;
pub mod repository;
pub mod service;

use std::cmp::PartialEq;

pub use domain::{Status, Task};
pub use inmemory_repository::{InMemoryTaskRepo, InMemoryTaskRepository};
pub use service::TaskService;

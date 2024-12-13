use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            completed: false,
        }
    }

    pub fn mark_as_completed(&mut self) {
        self.completed = true;
    }
}

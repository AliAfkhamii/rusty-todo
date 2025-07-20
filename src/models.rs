use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::option::Option;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<Category>,
    pub created: DateTime<Utc>,
    #[serde(default = "default_false")]
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}

impl Task {
    pub fn print(&self) {
        println!(
            "{}: {} [{}]",
            self.id,
            self.name,
            self.category
                .as_ref()
                .map(|c| c.name.as_str())
                .unwrap_or("no category")
        );
    }
}

fn default_false() -> bool {
    false
}

pub fn get_task<'a>(tasks: &'a mut [Task], id: &Uuid) -> Option<&'a mut Task> {
    Some(tasks.iter_mut().find(|task| task.id == *id)?)
}

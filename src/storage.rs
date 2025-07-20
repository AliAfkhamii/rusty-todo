use std::fs;
use std::path::Path;

use crate::models::Task;

const FILE: &str = "tasks.json";

pub fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string(&tasks).expect("Failed to serialize tasks.json");
    fs::write(FILE, json).expect("Failed to write tasks.json");
}


pub fn load_tasks() -> Vec<Task> {
    if Path::new(FILE).exists() {
        let contents = fs::read_to_string(FILE).expect("Failed to read tasks.json");
        serde_json::from_str(&contents).expect("Failed to deserialize tasks.json")
    } else {
        Vec::new()
    }
}
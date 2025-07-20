mod command;
mod models;
mod storage;

use models::Task;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks: Vec<Task> = Vec::new();
}

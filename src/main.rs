mod command;
mod models;
mod storage;

use chrono::Utc;
use command::{Command, parse_args};
use models::{Task, get_task};
use std::env;
use storage::{load_tasks, save_tasks};
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = load_tasks();

    match parse_args(args) {
        Some(Command::Add {
                 name,
                 description,
                 category,
             }) => {
            let task = Task {
                id: Uuid::new_v4(),
                name,
                description,
                category,
                created: Utc::now(),
                completed: false,
            };
            println!("Adding task {}", task.name);
            tasks.push(task);
            save_tasks(&tasks);
        }
        Some(Command::List { category }) => {
            if let Some(cat) = category {
                for task in &tasks {
                    let match_cat: bool = match (&task.category, Some(&cat)) {
                        (Some(tasks_cat), Some(this_cat)) => tasks_cat.id == this_cat.id,
                        _ => false,
                    };
                    if match_cat {
                        task.print();
                    }
                }
            } else {
                for task in &tasks {
                    task.print()
                }
            }
        }
        Some(Command::Remove { id }) => {
            let before = tasks.len();
            tasks.retain(|task| task.id != id);
            let after = tasks.len();
            if before != after {
                println!("Removed task {}", id);
                save_tasks(&tasks);
            } else {
                println!("No task found with ID {}", id);
            }
        }
        Some(Command::Done { id }) => {
            let task = get_task(tasks.as_mut_slice(), &id).expect("no task found");
            task.completed = true;
            println!("Done: {}", task.name);
            save_tasks(&tasks);
        }
        Some(Command::Reset { id }) => {
            let task = get_task(tasks.as_mut_slice(), &id).expect("no task found");;
            task.completed = false;
            println!("Done: {}", task.name);
            save_tasks(&tasks);
        }
        Some(Command::Edit { id, name }) => {
            let task = get_task(tasks.as_mut_slice(), &id).expect("no task found");;
            task.name = name;
            save_tasks(&tasks);
        }
        _ => println!("Invalid command"),
    }
}

use std::{env, option::Option};

// mod task;
// mod category;
use chrono::{DateTime, Utc};

// fn main() {}

// mod task
struct Task {
    id: usize,
    name: String,
    description: Option<String>,
    category: Option<Category>,
    created: DateTime<Utc>,
}
struct Category {
    id: usize,
    name: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let command_and_args = &args[1..];

    match command_and_args {
        [_, command, rest @ ..] if command == "add" => {
            println!("Adding flags");

            let mut description: Option<&String> = None;
            let mut category: Option<&String> = None;
            let name: String = rest[0].to_string();

            let mut i = 1;
            while i < rest.len() {
                let arg = rest[i].as_str();
                match arg {
                    "-d" | "--description" => {
                        description = Some(&rest[i + 1]);
                    }
                    "-c" | "--category" => {
                        category = Some(&rest[i + 1]);
                    }
                    _ => eprintln!("Unknown flag: {}", arg),
                }
                i += 1;
            }

            let final_description = match description {
                Some(d) => d.to_string(),
                None => {
                    eprintln!("couldn't parse description");
                    std::process::exit(1);
                }
            };

            let final_category = category.map(|c| Category {
                id: 0, // todo!(auto increment or random hash)
                name: c.to_string(),
            });

            Task {
                id: 0,
                name,
                description: Some(final_description),
                created: Utc::now(),
                category: final_category,
            };
            todo!("persistent saving")
        }
        [_, command, rest @ ..] if command == "list" => {
            println!("Listing flags");
            let mut category: Option<&str> = None;
            if rest.len() > 2 {
                panic!("too much arguments for command \"list\"");
            }

            let i = 0;
            while i > rest.len() {
                let arg = rest[i].as_str();
                match arg {
                    "-c" | "--category" => category = Some(rest[i + 1].as_str()),
                    _ => println!("Unknown flag {} for command \"list\"", arg),
                }
            }
        }
        [_, command, flags] if command == "remove" => {
            println!("Removing flags");
        }
        [_, command, flags] if command == "edit" => {
            println!("Editing flags");
        }
        [_, command, flags] if command == "reset" => {
            println!("Resetting flags");
        }
        [_, command, flags] if command == "done" => {
            println!("Done");
        }
        _ => (),
    }
}

use crate::models::Category;
use uuid::Uuid;

#[derive(Debug)]
pub enum Command {
    Add {
        name: String,
        description: Option<String>,
        category: Option<Category>,
    },
    List {
        category: Option<Category>,
    },
    Remove {
        id: Uuid,
    },
    Done {
        id: Uuid,
    },
    Reset {
        id: Uuid,
    },
    Edit {
        id: Uuid,
        name: String,
    },
}

pub fn parse_args(args: Vec<String>) -> Option<Command> {
    if args.len() < 2 {
        return None;
    }

    if let Some(executable_name) = args.get(0).map(|s| s.as_str()) {
        if executable_name != "todo" {
            eprintln!("Executable name does not match");
            return None;
        }
    }

    let command = args[1].as_str();
    let rest = &args[2..];

    match command {
        "add" => {
            if rest.is_empty() {
                eprintln!("Usage: todo add <name> [-d <desc>] [-c <cat>]");
                return None;
            }
            let name: String = rest[0].to_string();
            let mut description = None;
            let mut category = None;

            let mut i = 1;
            while i < rest.len() {
                match rest[i].as_str() {
                    "-d" | "--description" => {
                        if i + 1 >= rest.len() {
                            description = Some(&rest[i + 1]).map(|d| d.to_string());
                        }
                        i += 2;
                    }
                    "-c" | "--category" => {
                        if i + 1 >= rest.len() {
                            category = Some(Category {
                                id: Uuid::new_v4(),
                                name: rest[i + 1].to_string(),
                            });
                            i += 2;
                        }
                    }
                    _ => {
                        eprintln!("Unknown flag: {}", rest[i]);
                        i += 1;
                    }
                }
            }

            Some(Command::Add {
                name,
                description,
                category,
            })
        }
        "list" => {
            if rest.len() == 0 {
                return Some(Command::List { category: None });
            }

            if rest.len() > 2 {
                eprintln!("too much arguments for command \"list\"");
                return None;
            }
            let mut category = None;
            let mut i = 0;

            while i < rest.len() {
                match rest[i].as_str() {
                    "-c" | "--category" => {
                        if i + 1 >= rest.len() {
                            category = Some(Category {
                                id: Uuid::new_v4(),
                                name: rest[i + 1].to_string(),
                            })
                        }
                        i += 2
                    }
                    _ => {
                        println!("Unknown flag {} for command \"list\"", rest[i]);
                        i += 1;
                    }
                }
            }
            Some(Command::List { category })
        }
        "edit" => {
            if rest.len() >= 2 {
                let id = Uuid::parse_str(&rest[0])
                    .ok()
                    .expect("Invalid UUID format for ID");
                let name = rest[1].clone();
                Some(Command::Edit { id, name })
            } else {
                eprintln!("Usage: todo edit <id> <new_name>");
                None
            }
        }
        "remove" => {
            let id_str = rest.get(0)?;
            let id = Uuid::parse_str(id_str)
                .ok()
                .expect("Invalid UUID format for ID");
            Some(Command::Remove { id })
        }
        "done" => {
            let id_str = rest.get(0)?;
            let id = Uuid::parse_str(id_str)
                .ok()
                .expect("Invalid UUID format for ID");
            Some(Command::Done { id })
        }
        "reset" => {
            let id_str = rest.get(0)?;
            let id = Uuid::parse_str(id_str)
                .ok()
                .expect("Invalid UUID format for ID");
            Some(Command::Reset { id })
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            None
        }
    }
}

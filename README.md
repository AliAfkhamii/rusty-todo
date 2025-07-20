# Rusty ToDo CLI

## 📝 A Simple, Fast, and Intuitive Command-Line Interface (CLI) for Managing Tasks

`rusty-todo` is a minimalist yet powerful command-line application for managing your daily tasks directly from your terminal. Built with Rust, it prioritizes performance, reliability, and a straightforward user experience. This project also serves as a practical exercise to reinforce core Rust programming concepts, particularly focusing on CLI argument parsing, data structures, and basic data persistence.

---

## ✨ Features

* **Add Tasks:** Quickly add new tasks with optional descriptions and categories.
    ```bash
    todo add "Activity Name" [-d "description"] [-c "category"]
    ```
* **List Tasks:** View all your tasks, with options to filter by category or order.
    ```bash
    todo list [-a "order by category"] [-c "retrieve that categories activities"]
    ```
* **Mark as Complete:** Mark a task as done using its ID.
    ```bash
    todo done <id>
    ```
* **Reset Task:** Revert a completed task to pending status.
    ```bash
    todo reset <id>
    ```
* **Remove Task:** Delete a task permanently using its ID.
    ```bash
    todo rm <id>
    ```
* **Edit Task:** Modify the name of an existing task.
    ```bash
    todo edit <id> -n "new name"
    ```

---


### 💡Usage

Here are some examples of how to use the todo CLI:

**Adding a Task**
```bash
todo add "Buy groceries"
todo add "Call mom" -d "Ask about dinner plans"
todo add "Submit report" -c "Work"
todo add "Read Rust book" -d "Chapter 11: Error Handling" -c "Learning"
```

**Listing Tasks**
```bash
todo list                         # List all tasks
todo list -c "Work"               # List tasks categorized as "Work"
todo list -a "category"           # List all tasks, ordered by category (example of potential future feature)
```

**Marking a Task as Complete**
```bash
todo done 123                     # Mark task with ID 123 as complete
```

**Resetting a Task**
```bash
todo reset 123                    # Mark task with ID 123 as pending
```

**Removing a Task**
```bash
todo rm 456                       # Remove task with ID 456
```

**Editing a Task**
```bash
todo edit 789 -n "Updated task name" # Change the name of task 789
```
---

### 🏗️ Project Structure

The core logic of the rusty-todo CLI is organized as follows:

-    src/main.rs: The main entry point for the application, handling command-line argument parsing and delegating to core logic.

-    src/models.rs (or similar): Defines the Task and Category data structures.

-    src/storage.rs (future): Will handle reading and writing tasks to a persistent storage (e.g., JSON file, SQLite).

-    src/commands.rs (future): Will contain functions for each command's logic (add, list, done, etc.).
---
🤝 Contributing

Contributions are welcome! If you have suggestions for improvements, bug reports, or want to add new features, please feel free to:

1.    Fork the repository.

2.    Create a new branch (git checkout -b feature/your-feature-name).

3.    Make your changes.

4.    Commit your changes (git commit -m 'feat: Add new feature X').

5.    Push to the branch (git push origin feature/your-feature-name).

6.    Open a Pull Request.

Please ensure your code adheres to Rust's best practices and includes appropriate tests.

---

### 🙏 Acknowledgements

-    Inspired by the "Rust Book" for practical application of Rust concepts.

-    Uses the chrono crate for date and time handling.

-    (Will use serde and serde_json for serialization/deserialization in future).

-    (Will use clap for robust CLI argument parsing in future).
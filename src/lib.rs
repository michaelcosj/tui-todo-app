use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct TodoList {
    filepath: String,
    pub todo: Vec<String>,
    pub done: Vec<String>,
}

impl TodoList {
    pub fn new(filepath: &str) -> TodoList {
        // Create a new todo list
        let mut list = TodoList {
            filepath: filepath.to_string(),
            todo: vec![],
            done: vec![],
        };

        // Get todo and done item from file if it exists and put them in the list
        if Path::new(filepath).exists() {
            let file = fs::File::open(filepath).expect("failed to open file");
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with("TODO") {
                        list.todo.push(line[6..].to_string());
                    } else if line.starts_with("DONE") {
                        list.done.push(line[6..].to_string());
                    }
                }
            }
        }

        list
    }

    pub fn add_todo(&mut self, title: &str) {
        // Add the todo to the todo list and save it to file
        self.todo.push(title.to_string());
        self.save_to_file();
    }

    pub fn tick_todo(&mut self, id: usize) {
        // Remove todo item from todo list and add it to the done list
        // Then save the list to file
        if self.todo.len() > id {
            let item = self.todo.remove(id);
            self.done.push(item);
            self.save_to_file();
        }
    }

    pub fn remove_todo(&mut self, id: usize) {
        // Remove item from todo list and save to file
        if self.todo.len() > id {
            self.todo.remove(id);
            self.save_to_file();
        }
    }

    pub fn clear_done(&mut self) {
        // Remove item from todo list and save to file
        self.done.clear();
        self.save_to_file();
    }

    fn save_to_file(&self) {
        // Create a new string with the capacity of the length of the two lists
        let mut list = String::with_capacity(self.todo.len() + self.done.len());

        // Push all items in the todo list to the string
        for todo in &self.todo {
            list.push_str(format!("TODO: {}\n", todo).as_str());
        }

        // Push all items in the done list to the string
        for done in &self.done {
            list.push_str(format!("DONE: {}\n", done).as_str());
        }

        // Write the string to file
        fs::write(&self.filepath, list).expect("failed to save todo list");
    }
}

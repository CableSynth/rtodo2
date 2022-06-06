use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, io};
use std::io::{Read, Write};
use std::{
    cell::Cell,
    fs::{self, DirBuilder},
    path,
};

const TODO_FILE: &str = "~/.rtodo2/todo_file";

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
enum Status {
    Open,
    Done,
    Overdue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum Lifespan {
    Day,
    Week,
    Month,
    Year,
    Life,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
enum LifeCycle {
    Once,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    title: String,
    description: String,
    status: Cell<Status>,
    lifespan: Lifespan,
    lifecycle: LifeCycle,
}

impl Todo {
    pub fn new(
        title: String,
        description: String,
        lifespan: Lifespan,
        lifecycle: LifeCycle,
    ) -> Self {
        Self {
            title,
            description,
            lifespan,
            lifecycle,
            status: Cell::new(Status::Open),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    fn new() -> Self {
        Self { todos: vec![] }
    }

    fn load(&mut self) {
        let file_path = shellexpand::full(TODO_FILE).unwrap();
        let path = path::Path::new(file_path.as_ref());
        let prefix = path.parent().unwrap();

        DirBuilder::new().recursive(true).create(prefix).unwrap();
        let mut todo_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path.as_ref())
            .expect("Unable to open file");
        let mut data = String::new();
        todo_file
            .read_to_string(&mut data)
            .expect("Unable to read file");
        if data.is_empty() {
            data = String::from("[]")
        }
        self.todos = serde_json::from_str(&data).unwrap();
    }

    fn get_all(&self) -> &Vec<Todo> {
        &self.todos
    }

    fn add(&mut self, todo: Todo) {
        self.todos.push(todo)
    }
    fn save (&mut self) -> Result<(), io::Error> {
        let encoded = serde_json::to_string(&self.todos)?;
        let file_path = shellexpand::full(TODO_FILE).unwrap();
        let mut file = fs::OpenOptions::new().write(true).open(file_path.as_ref())?;
        file.write_all(encoded.as_bytes())?;
        Ok(())
    }
}
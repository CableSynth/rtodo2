
use serde::{Deserialize, Serialize};

use std::{cell::Cell, collections::LinkedList};

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

#[derive(Debug, Serialize, Deserialize, Clone, )]
struct Todo {
    title: String,
    description: String,
    status: Cell<Status>,
    lifespan: Lifespan,
    lifecycle: LifeCycle, 
}

impl Todo {
    pub fn new(title: String, description: String, lifespan: Lifespan, lifecycle: LifeCycle) -> Self {
        Self {
            title,
            description,
            lifespan,
            lifecycle,
            status: Cell::new(Status::Open),
        }
    }
}
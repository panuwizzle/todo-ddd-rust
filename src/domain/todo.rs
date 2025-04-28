use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive( Deserialize, Debug)]
pub struct CreateTodo {
    pub title: String,
}

#[derive( Deserialize, Debug)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
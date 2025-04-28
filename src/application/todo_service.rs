use crate::domain::todo::{CreateTodo, Todo, UpdateTodo};
use std::sync::{Arc, Mutex};

pub struct TodoService {
    todos: Arc<Mutex<Vec<Todo>>>,
    next_id: Arc<Mutex<u64>>,
}

impl TodoService {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn get_all(&self) -> Vec<Todo> {
        let todos = self.todos.lock().unwrap();
        todos.clone()
    }

    pub fn get_by_id(&self, id: u64) -> Option<Todo> {
        let todos = self.todos.lock().unwrap();
        todos.iter().find(|todo| todo.id == id).cloned()
    }

    pub fn create(&self, create_todo: CreateTodo) -> Todo {
        let mut todos = self.todos.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        let new_todo = Todo {
            id: *next_id,
            title: create_todo.title,
            completed: false,
        };

        todos.push(new_todo.clone());
        *next_id += 1;

        new_todo
    }

    pub fn update(&self, id: u64, update_todo: UpdateTodo) -> Option<Todo> {
        let mut todos = self.todos.lock().unwrap();
        if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
            if let Some(title) = update_todo.title {
                todo.title = title;
            }
            if let Some(completed) = update_todo.completed {
                todo.completed = completed;
            }
            return Some(todo.clone());
        }
        None
    }

    pub fn delete(&self, id: u64) -> bool {
        let mut todos = self.todos.lock().unwrap();
        let initial_len = todos.len();
        todos.retain(|todo| todo.id != id);
        todos.len() < initial_len
    }
}
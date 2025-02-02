use ic_cdk::{query, update};

use crate::errors::AppError;
use crate::models::todo::Todo;
use crate::TODOS;

#[query]
fn get_todo(id: String) -> Option<Todo> {
    TODOS.with(|todos| todos.borrow().get(&id))
}

#[derive(candid::CandidType, serde::Serialize)]
pub struct PaginatedTodos {
    items: Vec<Todo>,
    next_cursor: Option<String>,
}

#[query]
fn get_todos(cursor: Option<String>, limit: u32) -> PaginatedTodos {
    TODOS.with(|todos| {
        let todos = todos.borrow();
        let iter = match cursor {
            Some(last_id) => todos.range(last_id..),
            None => todos.range(..),
        };

        let mut items: Vec<Todo> = iter.take(limit as usize + 1).map(|(_, t)| t).collect();

        let next_cursor = if items.len() > limit as usize {
            Some(items.pop().unwrap().id)
        } else {
            None
        };

        PaginatedTodos { items, next_cursor }
    })
}

#[update]
fn insert_todo(title: String) -> String {
    let todo = Todo::new(title);
    TODOS.with(|todos| todos.borrow_mut().insert(todo.id.clone(), todo.clone()));
    todo.id
}

#[update]
fn update_todo_title(id: String, title: String) -> Result<(), AppError> {
    TODOS.with(|todos| {
        let todos = todos.borrow_mut();
        if let Some(mut todo) = todos.get(&id) {
            todo.title = title;
            Ok(())
        } else {
            Err(AppError::NotFound("Todo".to_string()))
        }
    })
}

#[update]
fn delete_todo(id: String) -> Result<(), AppError> {
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        if todos.contains_key(&id) {
            todos.remove(&id);
            Ok(())
        } else {
            Err(AppError::NotFound("Todo".to_string()))
        }
    })
}

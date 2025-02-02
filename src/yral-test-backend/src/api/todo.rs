use ic_cdk::{query, update};

use crate::errors::AppError;
use crate::models::todo::Todo;
use crate::{NEXT_ID, TODOS};

#[query]
fn get_todo(id: u64) -> Option<Todo> {
    TODOS.with(|todos| todos.borrow().get(&id))
}

#[derive(candid::CandidType, serde::Serialize)]
pub struct PaginatedTodos {
    items: Vec<Todo>,
    next_cursor: Option<u64>,
}

#[query]
fn get_todos(cursor: Option<u64>, limit: u32) -> PaginatedTodos {
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
fn insert_todo(title: String) -> Result<u64, AppError> {
    if title.is_empty() {
        return Err(AppError::InvalidInput("Title cannot be empty".to_string()));
    }

    let id = NEXT_ID.with(|counter| {
        let next_id = *counter.borrow();
        *counter.borrow_mut() += 1;
        next_id
    });

    let todo = Todo::new(id, title);
    TODOS.with(|todos| todos.borrow_mut().insert(id, todo.clone()));
    Ok(id)
}

#[update]
fn update_todo_title(id: u64, title: String) -> Result<(), AppError> {
    if title.is_empty() {
        return Err(AppError::InvalidInput("Title cannot be empty".to_string()));
    }

    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        if let Some(mut todo) = todos.get(&id) {
            todo.title = title;
            todos.insert(id, todo);
            Ok(())
        } else {
            Err(AppError::NotFound("Todo".to_string()))
        }
    })
}

#[update]
fn delete_todo(id: u64) -> Result<(), AppError> {
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

mod api;
mod errors;
mod models;

use ic_cdk::export_candid;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use models::todo::Todo;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static NEXT_ID: RefCell<u64> = RefCell::new(0);

    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static TODOS: RefCell<StableBTreeMap<u64, Todo, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

use api::todo::*;
use errors::AppError;
export_candid!();

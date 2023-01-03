use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct AppState {
    todoList_entries: Mutex<Vec<TotolistEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TotolistEntry {
    id: i32,
    date: i64,
    title: String,
}
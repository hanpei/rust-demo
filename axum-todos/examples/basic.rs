use std::{
    net::SocketAddr,
    sync::atomic::{AtomicUsize, Ordering},
};

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

static TODO_ID: AtomicUsize = AtomicUsize::new(1);

fn gen_next_id() -> usize {
    TODO_ID.fetch_add(1, Ordering::Relaxed)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/todos", get(todos_handler).post(create_todo_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("litsening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn index_handler() -> Html<&'static str> {
    Html("hello world")
}
async fn todos_handler() -> impl IntoResponse {
    Json([
        Todo::new(1, "buy the cola"),
        Todo::new(2, "clean the house"),
        Todo::new(3, "write a blog"),
    ])
}
async fn create_todo_handler(Json(payload): Json<CreateTodo>) -> impl IntoResponse {
    let CreateTodo { title } = payload;
    tracing::info!("Create todo success, which title is: {title}");
    let new_todo = Todo::new(gen_next_id(), title);
    (StatusCode::CREATED, Json(new_todo))
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateTodo {
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

impl Todo {
    fn new(id: usize, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            completed: false,
        }
    }
}

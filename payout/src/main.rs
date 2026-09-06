use std::{
    collections::HashMap,
    io::{self, Write},
};

use axum::{routing::post, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
}

// 2. Define a shape for the JSON response (optional)
#[derive(Serialize)]
struct UserResponse {
    id: u64,
    username: String,
    status: String,
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<UserResponse> {
    println!("Creating user {} with email {}", payload.username, payload.email);

    // Return a JSON response back to the client
    Json(UserResponse {
        id: 1337,
        username: payload.username,
        status: "success".to_string(),
    })
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("🚀 Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

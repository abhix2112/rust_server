// Import necessary crates and modules
use axum::{
    routing::get,
    Router, Json,
};
use serde::{Deserialize, Serialize}; // Helps with JSON handling
use rand::{distributions::Alphanumeric, Rng}; // For generating random passwords
use std::net::SocketAddr;

// Main async function where the server runs
#[tokio::main]
async fn main() {
    // Define routes for the server
    let app = Router::new()
        .route("/", get(root))  // Home route
        .route("/password", get(generate_password)); // Password generator route

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for the root route
async fn root() -> &'static str {
    "Welcome to Rust Web Server!"
}

// Handler for password generation
async fn generate_password() -> Json<PasswordResponse> {
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric) // Generate alphanumeric characters
        .take(12) // Take 12 characters
        .map(char::from)
        .collect(); // Collect them into a String

    Json(PasswordResponse { password })
}

// Struct for the JSON response
#[derive(Serialize, Deserialize)]
struct PasswordResponse {
    password: String,
}
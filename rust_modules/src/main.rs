mod handlers;

use axum::{
    Router, routing::get
};

use handlers::{
    health::health,
    user::{get_user, get_users}
};

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().
    route("/health", get(health)).
    route("/get_user", get(get_user)).route("/get_users", get(get_users));

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("Server running on http://127.0.0.1:8080");

    axum::serve(listner, app).await.unwrap();
}

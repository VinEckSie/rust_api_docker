use std::env;
use axum::{response::Html, routing::get, Router};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.expect("DB connect failed");

    let row: (i32,) = sqlx::query_as("SELECT 42")
        .fetch_one(&pool)
        .await
        .expect("Query failed");

    println!("✅ DB Connected — query result: {}", row.0);
    
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World UPDATED!</h1>")
}






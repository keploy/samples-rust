use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, FromRow};
use std::env;
use dotenv::dotenv;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Clone)]
struct Item {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct CreateItemRequest {
    name: String,
    description: String,
}

struct AppState {
    db: Pool<Postgres>,
}

#[post("/items")]
async fn create_item(state: web::Data<AppState>, body: web::Json<CreateItemRequest>) -> impl Responder {
    let id = Uuid::new_v4();
    let query_result = sqlx::query_as::<_, Item>(
        "INSERT INTO items (id, name, description) VALUES ($1, $2, $3) RETURNING id, name, description"
    )
    .bind(id)
    .bind(&body.name)
    .bind(&body.description)
    .fetch_one(&state.db)
    .await;

    match query_result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => {
            eprintln!("Failed to create item: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to create item: {}", e))
        }
    }
}

#[get("/items/{id}")]
async fn get_item(state: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    let query_result = sqlx::query_as::<_, Item>("SELECT id, name, description FROM items WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await;

    match query_result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().body("Item not found"),
    }
}

#[get("/items")]
async fn get_items(state: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as::<_, Item>("SELECT id, name, description FROM items")
        .fetch_all(&state.db)
        .await;

    match query_result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
             eprintln!("Failed to get items: {}", e);
             HttpResponse::InternalServerError().body(format!("Failed to get items: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut pool = None;
    for _ in 0..10 {
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await {
                Ok(p) => {
                    pool = Some(p);
                    break;
                }
                Err(_) => {
                    println!("Waiting for database...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
    }
    
    let pool = pool.expect("Failed to connect to database");

    // Create table if not exists (simplification for quickstart)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id UUID PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    println!("Server running at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(create_item)
            .service(get_items)
            .service(get_item)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

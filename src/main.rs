mod modules;
mod states;
mod infrastructure;

use std::env;
use ntex::web::*;
use sqlx::postgres::PgPoolOptions;
use crate::modules::user::core::command::handler::UserRegisterCommandHandler;
use crate::modules::user::infrastructure::user_repository::UserRepository;
use crate::modules::user::interface::user_route::createUser;
use crate::states::{AppState, UserDeps};

#[ntex::main]
async fn main() -> std::io::Result<()>{
    dotenv::from_filename(".development.env").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    let pool_clone = pool.clone();

    let user_repository = UserRepository::new(pool.clone());
    let user_register_command_handler = UserRegisterCommandHandler::new(user_repository.clone());

    let user_deps = UserDeps {
        user_register_command_handler,
        user_repository,
    };

    println!("\nSERVER ADDRESS IS: {}", &server_addr);

    HttpServer::new(move || {
        App::new()
            .state(AppState {
                pool: pool_clone.clone(),
            })
            .state(user_deps.clone())
            .service(createUser)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

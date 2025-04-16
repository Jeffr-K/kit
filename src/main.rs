mod modules;
mod states;
mod infrastructure;

use std::env;
use dotenv::dotenv;
use ntex::web::*;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::modules::user::interface::user_route::createUser;
use crate::states::AppState;

#[ntex::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    HttpServer::new(|| {
        App::new()
            .state(AppState {
                app_name: String::from("Server"),
                pool: pool.clone()
            })
            .service(createUser)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

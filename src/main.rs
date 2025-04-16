mod modules;
mod states;
mod infrastructure;

use ntex::web::*;
use crate::modules::user::interface::user_route::createUser;
use crate::states::AppState;

#[ntex::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .state(AppState {
                app_name: String::from("Server"),
            })
            .service(createUser)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

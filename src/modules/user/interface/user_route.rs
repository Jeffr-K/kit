use ntex::web::*;
use ntex::web::types::State;
use crate::states::AppState;

#[post("/user")]
#[allow(non_snake_case)]
async fn createUser(data: State<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body("user created".to_string() + " " + app_name)
}
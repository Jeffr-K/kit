use fastrace::prelude::SpanContext;
use ntex::web::*;
use ntex::web::types::{Json, State};
use crate::modules::user::core::command::command::UserRegisterCommand;
use crate::states::{AppState, UserDeps};

#[post("/user")]
#[fastrace::trace]
#[allow(non_snake_case)]
async fn createUser(
    command: Json<UserRegisterCommand>,
    state: State<AppState>,
    deps: State<UserDeps>,
) -> Result<impl Responder, Error> {
    let result = match deps.user_register_command_handler
        .handle(
            command.into_inner(),
            &deps,
            &state,
        )
        .await {
        Ok(result) => result,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)));
        }
    };
    
    Ok(HttpResponse::Ok().json(&result))
}
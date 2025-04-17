use sqlx::PgPool;
use crate::modules::user::core::command::handler::UserRegisterCommandHandler;
use crate::modules::user::infrastructure::user_repository::UserRepository;

pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Clone)]
pub struct UserDeps {
    pub user_register_command_handler: UserRegisterCommandHandler,
    pub user_repository: UserRepository,
}
use async_nats::Client;
use sqlx::PgPool;
use crate::modules::user::core::command::handler::UserRegisterCommandHandler;
use crate::modules::user::infrastructure::user_repository::UserRepository;
use crate::modules::user::infrastructure::user_security_repository::UserSecurityRepository;

pub struct AppState {
    pub pool: PgPool,
    pub nats_client: Client,
}

#[derive(Debug, Clone)]
pub struct UserDeps {
    pub user_register_command_handler: UserRegisterCommandHandler,
    pub user_repository: UserRepository,
    pub user_security_repository: UserSecurityRepository,
}
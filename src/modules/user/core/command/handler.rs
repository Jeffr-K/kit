use ntex::web::types::State;
use serde_json::json;
use crate::modules::user::core::command::command::{UserRegisterCommand, UserRegisterCommandResult};
use crate::modules::user::infrastructure::user_repository::UserRepository;
use crate::states::{AppState, UserDeps};

#[derive(Debug, Clone)]
pub struct UserRegisterCommandHandler {
    pub user_repository: UserRepository,
}

impl UserRegisterCommandHandler {
    pub fn new(user_repository: UserRepository) -> Self {
        UserRegisterCommandHandler {
            user_repository,
        }
    }

    pub async fn handle(
        &self,
        command: UserRegisterCommand,
        deps: &State<UserDeps>,
        state: &State<AppState>,
    ) -> Result<UserRegisterCommandResult, String> {
        let user = deps
            .user_repository
                .insert(&command, state)
                .await
                .map_err(|e| {
                    format!("Error inserting user: {:?}", e)
                })?;

        let event = json!({
            "event": "user.registered",
            "user": {
                "id": user.id,
                "username": user.name,
                "email": user.email,
            }
        });

        state
            .nats_client
                .publish("user.registered", serde_json::to_vec(&event).unwrap().into())
                .await
                .map_err(|e| format!("Failed to publish NATS event: {}", e))?;

        Ok(UserRegisterCommandResult { id: user.id })
    }
}
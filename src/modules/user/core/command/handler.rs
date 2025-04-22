use futures::try_join;
use ntex::web::types::State;
use serde_json::json;
use crate::modules::user::core::command::command::{UserRegisterCommand, UserRegisterCommandResult};
use crate::modules::user::core::entity::password::PasswordEncrypter;
use crate::modules::user::infrastructure::user_repository::UserRepository;
use crate::modules::user::infrastructure::user_security_repository::UserSecurityRepository;
use crate::states::{AppState, UserDeps};

#[derive(Debug, Clone)]
pub struct UserRegisterCommandHandler {
    pub user_repository: UserRepository,
    pub user_security_repository: UserSecurityRepository,
}

impl UserRegisterCommandHandler {
    pub fn new(user_repository: UserRepository, user_security_repository: UserSecurityRepository) -> Self {
        Self {
            user_repository,
            user_security_repository,
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
            .map_err(|e| format!("Error inserting user: {:?}", e))?;
        
        let password = command.password.clone();
        let encrypter = PasswordEncrypter::new();
        let encrypted_password = encrypter.hash(&password)
            .map_err(|e| format!("Error encrypting password: {:?}", e))?;
        
        let security_repository = &deps.user_security_repository;
        
        try_join!(
            security_repository.insert_password(
                user.id, 
                encrypted_password.hash.clone(), 
                encrypted_password.salt.clone()
            ),
            security_repository.insert_security_history(
                user.id,
                "REGISTRATION".to_string(),
                None,
                None
            ),
            security_repository.insert_security_counter("USER_REGISTRATION".to_string())
        )
            .map_err(|e| format!("Failed to execute security operations: {:?}", e))?;
        
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

        Ok(UserRegisterCommandResult {
            id: user.id,
        })
    }
}
use ntex::web::types::State;
use crate::modules::user::core::command::command::{UserRegisterCommand, UserRegisterCommandResult};
use crate::modules::user::infrastructure::user_repository::UserRepository;
use crate::states::{AppState, UserDeps};

// class A { fun a() {} }
// struct {}, impl
//

// struct A, impl A { fn a() {} }
// struct A, func(a A) Add() {}



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

        Ok(UserRegisterCommandResult {
            id: user.id,
        })
    }
}
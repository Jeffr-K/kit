use sqlx::{PgPool, Error, query_as};
use ntex::web::types::State;
use crate::modules::user::core::command::command::UserRegisterCommand;
use crate::modules::user::core::entity::user::User;
use crate::states::AppState;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        UserRepository {
            pool
        }
    }

    pub async fn insert(&self, command: &UserRegisterCommand, _state: &State<AppState>) -> Result<User, Error> {
        let now = Utc::now();

        let user = query_as::<_, User>(
            r#"
            INSERT INTO users (name, email, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, email, created_at, updated_at, deleted_at
            "#
        )
            .bind(&command.name)
            .bind(&command.email)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
}
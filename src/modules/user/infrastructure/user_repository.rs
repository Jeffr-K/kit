use ntex::http::StatusCode;
use ntex::web::types::State;
use crate::modules::user::core::entity::user::User;
use crate::states::AppState;

pub struct UserRepository {

}

pub struct Data {
    pub name: String,
    pub email: String,
}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {

        }
    }

    pub async fn insert(&self, data: Data, state: State<AppState>) -> Result<User> {
        let rec = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        )
            .bind(&data.name)
            .bind(&data.email)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("데이터베이스 오류: {}", e),
                )
            })?;

        Ok((StatusCode::CREATED), Json(rec))
    }
}
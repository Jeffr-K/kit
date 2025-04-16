use sqlx::PgPool;

pub struct AppState {
    pub app_name: String,
    pub pool: PgPool,
}
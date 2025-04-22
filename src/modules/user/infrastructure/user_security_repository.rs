use sqlx::{PgPool, Error, query_as};
use crate::modules::user::core::entity::system_security_counter::SystemSecurityCounter;
use crate::modules::user::core::entity::user_security_history::UserSecurityHistory;
use crate::modules::user::core::entity::user_security_password::UserSecurityPassword;

#[derive(Debug, Clone)]
pub struct UserSecurityRepository {
    pool: PgPool,
}

impl UserSecurityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    // 비밀번호 보안 정보 저장
    pub async fn insert_password(&self, user_id: i32, password_hash: String, salt: String) -> Result<i32, Error> {
        let record = query_as::<_, UserSecurityPassword>(
            r#"
            INSERT INTO user_security_password (password_hash, salt, user_id)
            VALUES ($1, $2, $3)
            RETURNING id, password_hash, salt, user_id, last_password_change, failed_attempts, 
            account_locked, lock_time, created_at, updated_at, deleted_at
            "#
        )
            .bind(password_hash)
            .bind(salt)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(record.id)
    }

    // 보안 이력 저장
    pub async fn insert_security_history(
        &self,
        user_id: i32,
        action_type: String,
        ip_address: Option<String>,
        device_info: Option<String>
    ) -> Result<i64, Error> {
        let record = sqlx::query_as::<_, UserSecurityHistory>(
            r#"
        INSERT INTO user_security_history (user_id, action_type, ip_address, device_info)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, action_type, ip_address, device_info, timestamp
        "#
        )
            .bind(user_id)
            .bind(action_type)
            .bind(ip_address)
            .bind(device_info)
            .fetch_one(&self.pool)
            .await?;

        Ok(record.id)
    }

    // 시스템 보안 카운터 추가/증가
    pub async fn insert_security_counter(&self, counter_type: String) -> Result<i64, sqlx::Error> {
        let record = sqlx::query_as::<_, SystemSecurityCounter>(
            r#"
            INSERT INTO system_security_counter (counter_type, counter_value)
            VALUES ($1, 1)
            ON CONFLICT (counter_type) 
            DO UPDATE SET 
                counter_value = system_security_counter.counter_value + 1,
                last_updated = CURRENT_TIMESTAMP
            RETURNING counter_value
            "#
        )
            .bind(counter_type)
            .fetch_one(&self.pool)
            .await?;

        Ok(record.counter_value as i64)
    }
}
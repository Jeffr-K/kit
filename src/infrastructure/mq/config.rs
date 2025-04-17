use std::env;
use dotenv::{from_filename};

pub struct NatsConfig {
    pub url: String,
    pub connection_name: Option<String>,
    pub auth_token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub connect_timeout: Option<std::time::Duration>,
    pub reconnect_buffer_size: Option<usize>,
    pub max_reconnects: Option<usize>,
    pub reconnect_wait: Option<std::time::Duration>,
    pub jetstream_enabled: bool,
    pub domain: Option<String>,
}

impl Default for NatsConfig {
    fn default() -> Self {
        from_filename(".development.env").ok();

        let nats_url = env::var("NATS_URL").expect("DATABASE_URL must be set");

        Self {
            url: nats_url,
            connection_name: None,
            auth_token: None,
            username: None,
            password: None,
            connect_timeout: Some(std::time::Duration::from_secs(5)),
            reconnect_buffer_size: Some(8 * 1024 * 1024), // 8MB
            max_reconnects: Some(60),
            reconnect_wait: Some(std::time::Duration::from_secs(2)),
            jetstream_enabled: true,
            domain: None,
        }
    }
}

impl NatsConfig {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    pub fn with_connection_name(mut self, name: impl Into<String>) -> Self {
        self.connection_name = Some(name.into());
        self
    }

    pub fn with_auth_token(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    pub fn with_credentials(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    pub fn with_connect_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    pub fn with_reconnect_buffer_size(mut self, size: usize) -> Self {
        self.reconnect_buffer_size = Some(size);
        self
    }

    pub fn with_max_reconnects(mut self, max: usize) -> Self {
        self.max_reconnects = Some(max);
        self
    }

    pub fn with_reconnect_wait(mut self, wait: std::time::Duration) -> Self {
        self.reconnect_wait = Some(wait);
        self
    }

    pub fn with_jetstream(mut self, enabled: bool) -> Self {
        self.jetstream_enabled = enabled;
        self
    }

    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
}
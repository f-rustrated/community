use std::sync::OnceLock;

pub struct Config {
    pub(crate) jwt_secret: String,
    pub(crate) database_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or("JWT_SECRET".to_string()),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or("postgres://frustacean:abc123@localhost:5434/community".to_string()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        dotenv::dotenv().ok();
        Config::default()
    })
}

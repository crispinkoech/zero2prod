use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

pub fn get_config() -> Result<AppConfig, ConfigError> {
    let app_config = Config::builder()
        .add_source(File::new("config.yaml", FileFormat::Yaml))
        .build()?;
    app_config.try_deserialize()
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub app_host: String,
    pub app_port: u16,
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub database_name: String,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub username: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name,
        )
    }
}

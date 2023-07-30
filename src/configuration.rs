use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppSettings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            application_port: 8000,
            database: DatabaseSettings::default(),
        }
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            username: "postgres".into(),
            password: "password".into(),
            port: 5432,
            database_name: "newsletter".into(),
        }
    }
}

pub fn get_configuration() -> Result<AppSettings, config::ConfigError> {
    let configs = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build();
    match configs {
        Ok(config) => config.try_deserialize(),
        Err(_) => Ok(AppSettings::default()),
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port,
        )
    }
}

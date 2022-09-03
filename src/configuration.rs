use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

// Environment defines where our environment can run, production=docker and changes to host to 0.0.0.0
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    // define the string representation of an Enum
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "lroduction",
        }
    }
}

// implement the trait TryFrom which converts from a "String" to an enum of Environment
// e.g. "local" maps to "Environment::Local" with as_str returning "local"
impl TryFrom<String> for Environment {
    type Error = String;

    // we return Self which in this case is Environment, we define a specific error for this case
    fn try_from(s: String) -> Result<Self, Self::Error> {
        // if we are given e.g. "local" how do we match (again Self = enum Environment)
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`", other
            ))
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    settings.merge(config::File::from(configuration_directory.join(environment.as_str())).required(true))?;
    settings.try_into()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}/{}",
                            self.username,
                            self.password.expose_secret(),
                            self.host,
                            self.port,
                            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        // we need to make sure we don't specify a db so we can dynamically construct a random
        // db_name using Uuid v4.
        Secret::new(format!("postgres://{}:{}@{}:{}",
                            self.username,
                            self.password.expose_secret(),
                            self.host,
                            self.port,
        ))
    }
}
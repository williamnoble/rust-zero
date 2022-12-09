use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use crate::domain::SubscriberEmail;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub email_client: EmailClientSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
    pub authorization_token: Secret<String>,
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
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
            Environment::Production => "production",
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

    // Add base configuration
    let configuration_directory = base_path.join("configuration");
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // Add environment-specific configuration e.g. `local`
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    settings.merge(config::File::from(configuration_directory.join(environment.as_str())).required(true))?;

    // Add environment variables
    settings.merge(config::Environment::with_prefix("APP_NEWSLETTER").separator("__"))?;

    settings.try_into()
}
// TODO: For production change this to PgConnectionOptions (allow SSL) ref: 128
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
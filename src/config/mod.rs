mod dynamicarg;

use crate::error::Error;
pub use dynamicarg::DynamicArgument;
use serde::Deserialize;

/// The S3 credentials
#[derive(Debug, Deserialize)]
pub struct S3Creds {
    /// The S3 key ID
    pub id: DynamicArgument,
    /// The S3 secret
    pub secret: DynamicArgument,
}
/// Credentials
#[derive(Debug, Deserialize)]
pub struct Credentials {
    /// Restic encryption credentials
    pub restic: DynamicArgument,
    /// S3 credentials
    pub s3: Option<S3Creds>,
}

/// The raw restic flags to pass during invocation
#[derive(Debug, Deserialize, Default)]
pub struct Flags {
    /// The flags to pass during invocation of `restic backup ...`
    #[serde(default)]
    pub backup: Vec<String>,
}
/// The restic configuration
#[derive(Debug, Deserialize)]
pub struct Restic {
    /// The managed directories
    pub dirs: Vec<String>,
    /// The repository URL
    pub repo: String,
    /// Whether to perform a snapshot before restoring or not
    #[serde(default)]
    pub safe_restore: bool,
    /// The raw restic flags to pass during invocation
    #[serde(default)]
    pub flags: Flags,
}

/// Custom commands
#[derive(Debug, Deserialize, Default)]
pub struct Commands {
    /// Pre-exec commands
    pub preexec: Vec<String>,
    /// Post-exec commands
    pub postexec: Vec<String>,
}

/// The restic-ez configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    /// The restic config
    pub restic: Restic,
    /// Storage credentials
    pub credentials: Credentials,
    /// Custom commands
    #[serde(default)]
    pub commands: Commands,
}
impl Config {
    /// Loads the config
    pub fn from_str<T>(config: T) -> Result<Self, Error>
    where
        T: AsRef<str>,
    {
        let config = toml::from_str(config.as_ref()).map_err(|e| einval!("Invalid config: {e}"))?;
        Ok(config)
    }

    /// Generates the environment set from the config
    pub fn to_env(&self) -> Result<Vec<(&'static str, String)>, Error> {
        // The basic config
        let mut env =
            vec![("RESTIC_REPOSITORY", self.restic.repo.clone()), ("RESTIC_PASSWORD", self.credentials.restic.eval()?)];

        // Evaluate the credentials
        if let Some(s3) = self.credentials.s3.as_ref() {
            env.push(("AWS_ACCESS_KEY_ID", s3.id.eval()?));
            env.push(("AWS_SECRET_ACCESS_KEY", s3.secret.eval()?));
        }
        Ok(env)
    }
}

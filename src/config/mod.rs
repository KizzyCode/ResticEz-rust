mod dynamic_argument;
pub use dynamic_argument::DynamicArgument;

use crate::error::Result;
use serde::{ Serialize, Deserialize };
use std::fmt::{ self, Display, Formatter };


/// The S3 credentials
#[derive(Debug, Serialize, Deserialize)]
pub struct S3Creds {
    /// The S3 key ID
    pub id: DynamicArgument,
    /// The S3 secret
    pub secret: DynamicArgument
}
/// Credentials
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    /// Restic encryption credentials
    pub restic: DynamicArgument,
    /// S3 credentials
    pub s3: Option<S3Creds>
}


/// The raw restic flags to pass during invocation
#[derive(Debug, Serialize, Deserialize)]
pub struct Flags {
    /// The flags to pass during invocation of `restic backup ...`
    pub backup: Option<Vec<String>>
}
/// The restic configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Restic {
    /// The managed directory
    pub dir: String,
    /// The repository URL
    pub repo: String,
    /// Whether to perform a snapshot before restoring or not
    pub safe_restore: bool,
    /// The raw restic flags to pass during invocation
    pub flags: Option<Flags>,
}


/// The restic-ez configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// The restic config
    pub restic: Restic,
    /// Storage credentials
    pub credentials: Credentials
}
impl Config {
    /// Loads the config
    pub fn from_str<T>(config: T) -> Result<Self> where T: AsRef<str> {
        let config = toml::from_str(config.as_ref())
            .map_err(|e| einval!("Invalid config ({})", e))?;
        Ok(config)
    }

    /// Generates the environment set from the config
    pub fn to_env(&self) -> Result<Vec<(&'static str, String)>> {
        // The basic config
        let mut env = vec![
            ("DIRECTORY", self.restic.dir.clone()),
            ("RESTIC_REPOSITORY", self.restic.repo.clone()),
            ("RESTIC_PASSWORD", self.credentials.restic.eval()?)
        ];

        // Evaluate the credentials
        if let Some(s3) = self.credentials.s3.as_ref() {
            env.push(("AWS_ACCESS_KEY_ID", s3.id.eval()?));
            env.push(("AWS_SECRET_ACCESS_KEY", s3.secret.eval()?));
        }
        Ok(env)
    }
}
impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let json = toml::to_string_pretty(self)
            .expect("Failed to serialize config?!");
        write!(f, "{}", json)
    }
}

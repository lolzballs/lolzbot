use serde_json;

use std::fs::File;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> ::Result<Self> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }
}

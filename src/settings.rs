use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub ip_address: Option<Ipv4Addr>,
    pub port: Option<u16>,
    pub metrics_path: Option<String>,
    pub scrapinghub_api_key: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            ip_address: Some("0.0.0.0".parse().unwrap()),
            port: Some(3030),
            metrics_path: Some(String::from("metrics")),
            scrapinghub_api_key: String::new(),
        }
    }
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        debug!("Reading settings.");
        let mut s = Config::new();
        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("scrapinghub_exporter"))?;
        s.merge(Environment::with_prefix("scrapinghub_exporter"))?;
        let default: Settings = Settings::default();
        let parsed_settings: Settings = s.try_into()?;
        Ok(Settings {
            ip_address: parsed_settings.ip_address.or(default.ip_address),
            port: parsed_settings.port.or(default.port),
            metrics_path: parsed_settings.metrics_path.or(default.metrics_path),
            scrapinghub_api_key: parsed_settings.scrapinghub_api_key,
        })
    }
}

use crate::domain::config::Config;
use crate::ports::config_port::ConfigPort;
use std::fs;

pub struct FileConfigAdapter {
    file_path: String,
}

impl FileConfigAdapter {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl ConfigPort for FileConfigAdapter {
    fn load_config(&self) -> Config {
        let config_str = fs::read_to_string(&self.file_path).expect("Unable to read config file");
        serde_yaml::from_str(&config_str).expect("Unable to parse config file")
    }
}

use crate::domain::config::Config;

pub trait ConfigPort {
    fn load_config(&self) -> Config;
}

pub trait EnvConfigPort {
    fn load_proxy_address(&self) -> Option<String>;
}
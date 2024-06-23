use crate::ports::config_port::EnvConfigPort;

pub struct EnvConfigAdapter;

impl EnvConfigAdapter {
    pub fn new() -> Self {
        Self
    }
    fn get_env_var(key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
}

impl EnvConfigPort for EnvConfigAdapter {
    fn load_proxy_address(&self) -> Option<String> {
        Self::get_env_var("PORT").map(|port| {
            let mut address = "127.0.0.1:".to_string();
            address.push_str(&port);
            address
        })
    }
}

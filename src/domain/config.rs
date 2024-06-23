use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub proxy: ProxyConfig,
}

#[derive(Debug, Deserialize)]
pub struct ProxyConfig {
    pub address: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub match_criteria: MatchCriteria,
    pub forward_to: String,
}

#[derive(Debug, Deserialize)]
pub struct MatchCriteria {
    pub domain: Option<String>,
    pub base_path: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

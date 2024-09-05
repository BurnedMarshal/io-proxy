use crate::domain::config::Rule;
use hyper::Request;
use log::info;

pub fn find_forward_address(req: &Request<hyper::Body>, rules: &[Rule]) -> Option<String> {
    for rule in rules {
        info!("Request Host: {}", req.uri().host().unwrap_or_else(|| ""));
        info!("Request Path: {}", req.uri().path());
        if let Some(domain) = &rule.match_criteria.domain {
            if req.uri().host() == Some(domain.as_str()) {
                return Some(rule.forward_to.clone());
            }
        }
        if let Some(base_path) = &rule.match_criteria.base_path {
            info!("Rule Path checked: {}", base_path);
            if req.uri().path().starts_with(base_path) {
                return Some(rule.forward_to.clone());
            }
        }
        if let Some(headers) = &rule.match_criteria.headers {
            let mut headers_match = true;
            for (key, value) in headers {
                if req.headers().get(key).map(|v| v.to_str().unwrap()) != Some(value.as_str()) {
                    headers_match = false;
                    break;
                }
            }
            if headers_match {
                return Some(rule.forward_to.clone());
            }
        }
    }
    None
}

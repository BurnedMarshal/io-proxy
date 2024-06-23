use hyper::{Client, Request, Response, Body, Uri};
use std::sync::Arc;
use crate::domain::rules::find_forward_address;
use crate::domain::config::Rule;

pub async fn handle_request(
    req: Request<Body>,
    rules: Arc<Vec<Rule>>,
) -> Result<Response<Body>, hyper::Error> {
    let forward_address = find_forward_address(&req, &rules).unwrap_or_else(|| "http://default.example.com".to_string());
    let uri_string = format!("{}{}", forward_address, req.uri());
    let uri: Uri = uri_string.parse().unwrap();

    let (mut parts, body) = req.into_parts();
    parts.uri = uri;

    let new_req = Request::from_parts(parts, body);
    let client = Client::new();
    client.request(new_req).await
}

use hyper::{Client, Request, Response, Body, Uri};
use hyper_tls::HttpsConnector;
use std::sync::Arc;
use crate::domain::rules::find_forward_address;
use crate::domain::config::Rule;
use log::{info, error};

pub async fn handle_request(
    req: Request<Body>,
    rules: Arc<Vec<Rule>>,
) -> Result<Response<Body>, hyper::Error> {
    let forward_address = find_forward_address(&req, &rules).unwrap_or_else(|| "http://default.example.com".to_string());
    let uri_string = format!("{}{}", forward_address, req.uri());
    info!("Forwarding request to: {}", uri_string);  // Logging
    let uri: Uri = match uri_string.parse() {
        Ok(u) => u,
        Err(e) => {
            error!("Invalid URI: {}", e);
            return Ok(Response::builder()
                .status(500)
                .body(Body::from("Invalid URI"))
                .unwrap());
        }
    };

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let (mut parts, body) = req.into_parts();
    parts.uri = uri;
    parts.headers.clear();

    let new_req = Request::from_parts(parts, body);
    match client.request(new_req).await {
        Ok(res) => {
            info!("Received response with status: {}", res.status());  // Logging risposta
            Ok(res)
        }
        Err(e) => {
            error!("Error forwarding request: {}", e);  // Logging errore
            Ok(Response::builder()
                .status(500)
                .body(Body::from("Failed to forward request"))
                .unwrap())
        }
    }
}

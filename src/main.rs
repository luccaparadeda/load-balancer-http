use std::{ str::FromStr, sync::{atomic::{AtomicUsize, Ordering}, Arc}};

use axum::{body::Body, extract::{Request, State}, handler::{Handler, HandlerWithoutStateExt}, http::{uri::{Authority, Scheme}, StatusCode, Uri}, response::IntoResponse};
use hyper_util::{client::legacy::{connect::HttpConnector, Client}, rt::TokioExecutor};

#[derive(Clone)]
struct AppState {
    addrs: Vec<&'static str>,
    req_counter: Arc<AtomicUsize>,
    http_client: Client<HttpConnector, Body>,
}

#[tokio::main]
async fn main() { 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();
    
    let addrs = ["0.0.0.0:9998", "0.0.0.0:9997"];

    let client = Client::builder(TokioExecutor::new()).build_http::<Body>();

    let app_state = AppState {
        http_client: client.clone(),
        req_counter: Arc::new(AtomicUsize::new(0)),
        addrs: addrs.to_vec(),
    };
    
    let app = proxy.with_state(app_state);
    axum::serve(listener, app).await.unwrap();
    println!("Hello, world!");
}

async fn proxy(State(AppState {req_counter, addrs ,http_client}): State<AppState>, mut req: Request) -> impl IntoResponse {
    let count = req_counter.fetch_add(1, Ordering::Relaxed);
    *req.uri_mut() = {
        let uri = req.uri();
        let mut parts = uri.clone().into_parts();
        parts.authority = Authority::from_str(addrs[count % addrs.len()]).ok();
        parts.scheme = Some(Scheme::HTTP);
        Uri::from_parts(parts).unwrap()
    };

    match http_client.request(req).await {
        Ok(res) => Ok(res),
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}
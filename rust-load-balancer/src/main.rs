use std::{ borrow::Borrow, env, str::FromStr, sync::{atomic::{AtomicUsize, Ordering}, Arc}};

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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let first_url = env::var("FIRST_URL").unwrap_or_else(|_| "https://google.com".to_string());
    let second_url = env::var("SECOND_URL").unwrap_or_else(|_| "https://youtube.com".to_string());

    let urls = vec!["http://localhost:8081", "http://localhost:8082"];


    let client = Client::builder(TokioExecutor::new()).build_http::<Body>();

    let app_state = AppState {
        http_client: client.clone(),
        req_counter: Arc::new(AtomicUsize::new(0)),
        addrs: urls,
    };
    
    let app = proxy.with_state(app_state);
    axum::serve(listener, app).await.unwrap();
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
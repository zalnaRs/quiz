use askama::Template;
use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};

use crate::web::HtmlTemplate;
use crate::web::exit_ticket;

struct AppState {}

pub struct App {}
impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .route("/", get(self::get::root))
            .nest("/exit", exit_ticket::router());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:9090")
            .await
            .unwrap();
        tracing::debug!("listening on {}", listener.local_addr().unwrap());
        let _ = axum::serve(listener, app).await;

        Ok(())
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

mod get {
    use super::*;

    pub async fn root() -> impl IntoResponse {
        HtmlTemplate(IndexTemplate {})
    }
}

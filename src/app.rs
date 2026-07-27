use axum::{Json, Router, routing::get};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::models::Asset;

pub struct App;

pub struct AppState{
    assets: Vec<Asset>,
}

impl AppState {
    fn new() -> Self {
        Self{
            assets: Default::default(),
        }
    }
}

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        let router = Router::new()
            .route("/", get(list_assets))
            .with_state(AppState::new());

        info!("Starting service");

        axum::serve(listener, router).await?;
        Ok(())
    }
}

#[tracing::instrument]
async fn list_assets() -> Json<Vec<Asset>> {
    Json(Vec::new())
}

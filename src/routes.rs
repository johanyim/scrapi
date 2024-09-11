use std::{future::Future, pin::Pin};

use axum::response::IntoResponse;
use serde_yaml::Value;

async fn handler_with_params(param2: i32) -> String {
    format!("param2: {}", param2)
}

pub fn make_handler_with_params(
    param2: i32,
) -> impl FnOnce() -> Pin<Box<dyn Future<Output = String> + Send>> {
    move || {
        let fut = async move { handler_with_params(param2).await };
        Box::pin(fut)
    }
}

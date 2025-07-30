mod config;
mod plugins;
mod service;

use crate::service::embedding::EmbeddingServiceImpl;
use axum::{extract::Multipart, response::IntoResponse, Json};
use plugins::fastembed::EmbeddingPlugin;
use spring::App;
use spring_web::{
    axum,
    error::{KnownWebError, Result},
    extractor::Component,
    post, WebPlugin,
};

#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(EmbeddingPlugin)
        .add_plugin(WebPlugin)
        .run()
        .await
}

// #[axum::debug_handler]
#[post("/text_embedding")]
async fn text_embedding(
    Component(embedding): Component<EmbeddingServiceImpl>,
    text: String,
) -> Result<impl IntoResponse> {
    Ok(Json(embedding.text_embedding(&text).await?))
}

// #[axum::debug_handler]
#[post("/batch_text_embedding")]
async fn batch_text_embedding(
    Component(embedding): Component<EmbeddingServiceImpl>,
    Json(texts): Json<Vec<String>>,
) -> Result<impl IntoResponse> {
    Ok(Json(embedding.batch_text_embedding(texts).await?))
}

// #[axum::debug_handler]
#[post("/img_embedding")]
async fn img_embedding(
    Component(embedding): Component<EmbeddingServiceImpl>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "file" {
            let data = field.bytes().await.unwrap();

            return Ok(Json(embedding.img_embedding(&data).await?));
        }
    }
    Err(KnownWebError::bad_request("请求有误"))?
}

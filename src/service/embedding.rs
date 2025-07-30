use crate::plugins::fastembed::{ImgEmbedding, TxtEmbedding};
use anyhow::Context;
use spring::plugin::service::Service;

#[derive(Clone, Service)]
pub struct EmbeddingServiceImpl {
    #[inject(component)]
    text_embedding: TxtEmbedding,
    #[inject(component)]
    image_embedding: ImgEmbedding,
}

impl EmbeddingServiceImpl {
    pub async fn text_embedding(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        let mut embeddings = self
            .text_embedding
            .embed(vec![text], None)
            .context("txt embedding failed")?;
        let embedding = embeddings.remove(0);
        Ok(embedding)
    }

    pub async fn batch_text_embedding(&self, texts: Vec<String>) -> anyhow::Result<Vec<Vec<f32>>> {
        let embeddings = self
            .text_embedding
            .embed(texts, None)
            .context("txt embedding failed")?;
        Ok(embeddings)
    }

    pub async fn img_embedding(&self, img: &[u8]) -> anyhow::Result<Vec<f32>> {
        let mut embeddings = self
            .image_embedding
            .embed_bytes(&[img], None)
            .context("img embedding failed")?;
        let embedding = embeddings.remove(0);
        Ok(embedding)
    }
}

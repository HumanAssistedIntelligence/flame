use fastembed::{EmbeddingModel, FlagEmbedding, InitOptions};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref FAST_EMBEDDING: Mutex<Option<FlagEmbedding>> = Mutex::new(
        match FlagEmbedding::try_new(InitOptions {
            model_name: EmbeddingModel::AllMiniLML6V2,
            show_download_message: false,
            ..Default::default()
        }) {
            Ok(model) => Some(model),
            Err(e) => {
                eprintln!("Failed to initialize the FlagEmbedding model: {}", e);
                None
            }
        }
    );
}

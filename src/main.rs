use std::time::Duration;
use autosurgeon::{hydrate};
use samod::{BackoffConfig, DocumentId, Repo, Url};
use tokio::time::sleep;
use crate::types::Doc;
use anyhow::{Result};

mod types;
pub mod item_parser;

#[tokio::main]
pub async fn main() -> Result<()> {

    // Initialize logging based on verbose flag
    tracing_subscriber::fmt()
        .with_env_filter(
            if true {
                tracing_subscriber::EnvFilter::new("debug")
            } else {
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn"))
            },
        )
        .init();

    let repo: Repo = Repo::build_tokio()
        .with_storage(samod::storage::TokioFilesystemStorage::new("./data"))
        .load()
        .await;

    tracing::debug!("Initializing automerge-repo");

    let ws_url = Url::parse(&"wss://5bcaaf94-60ef-4757-b55c-5f2e443c480c.ka.bw-cloud-instance.org:443".to_string())
        .expect("valid WebSocket listener URL for samod");

    tracing::debug!("Connecting to sync server");

    let repo_dailer = repo.dial_websocket(ws_url, BackoffConfig::default());

    match repo_dailer {
        Ok(dailer_handle) => {
            tracing::debug!("WebSocket connected");
            let doc_id: DocumentId = "3J14FKnxJStfFycHKszg2xdv5hRy".parse().unwrap();

            tracing::debug!("Looking for document...");
            let mut doc_handle = repo.find(doc_id.clone()).await?;


            if doc_handle.is_none() {
                tracing::debug!("Document not immediately available, waiting for sync...");
                sleep(Duration::from_secs(2)).await;

                // Try again after sync
                doc_handle = repo.find(doc_id).await?;
            } else {
                tracing::debug!("Document found, waiting for full sync...");
                sleep(Duration::from_secs(2)).await;
            }

            match doc_handle {
                None => panic!("Document not found. Make sure:\n  1. The sync server is running\n  2. The document exists in the browser\n  3. The document ID is correct"),
                Some(doc_h) => {
                    let doc_data: Doc = doc_h.with_document(|doc| {
                        match hydrate(doc) {
                            Ok(data) => Ok(data),
                            Err(e) => {
                                tracing::error!("Failed to hydrate document: {:?}", e);
                                Err(anyhow::anyhow!("Failed to hydrate document for display: {:?}", e))
                            }
                        }
                    })?;

                    println!("{:?}", doc_data)
                }
            }

            Ok(())
        }
        Err(e) => {
            tracing::debug!("Failed to connect to WebSocket server");
            Err(anyhow::anyhow!("Failed to connect to WebSocket server: {:?}", e))
        }
    }
}
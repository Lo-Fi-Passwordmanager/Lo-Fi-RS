use crate::types::Doc;
use anyhow::Result;
use autosurgeon::hydrate;
use samod::{BackoffConfig, DialerHandle, DocHandle, DocumentId, Repo, Url};

pub mod security;
pub mod types;

pub async fn connect(websocket_url: Url) -> Result<(Repo, DialerHandle)> {
    tracing::debug!("Initializing automerge-repo");

    let repo: Repo = Repo::build_tokio()
        .with_storage(samod::storage::TokioFilesystemStorage::new("./data"))
        .load()
        .await;

    tracing::debug!("Connecting to sync server...");

    let repo_dialer = repo.dial_websocket(websocket_url, BackoffConfig::default());

    match repo_dialer {
        Ok(dailer_handle) => {
            tracing::debug!("WebSocket connected.");
            Ok((repo, dailer_handle))
        }
        Err(e) => {
            tracing::error!("Failed to connect to WebSocket server");
            Err(anyhow::anyhow!(
                "Failed to connect to WebSocket server: {:?}",
                e
            ))
        }
    }
}

pub async fn openDocument(
    repo: Repo,
    _dialer_handle: DialerHandle,
    doc_id: DocumentId,
) -> Result<DocHandle> {
    tracing::debug!("Looking for document...");
    let doc_handle = repo.find(doc_id.clone()).await?;

    match doc_handle {
        None => {
            tracing::error!("Document not found.");
            Err(anyhow::anyhow!(
                "Document not found. Make sure:\n  1. The sync server is running\n  2. The document exists in the browser\n  3. The document ID is correct"
            ))
        }
        Some(handle) => {
            tracing::debug!("Document found.");
            Ok(handle)
        }
    }
}

pub async fn getDocData(doc_handle: DocHandle) -> Result<Doc> {
    doc_handle.with_document(|doc| match hydrate(doc) {
        Ok(data) => Ok(data),
        Err(e) => {
            tracing::error!("Failed to hydrate document: {:?}", e);
            Err(anyhow::anyhow!("Failed to hydrate document: {:?}", e))
        }
    })
}

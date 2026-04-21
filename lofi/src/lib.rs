use crate::document::automerge_document::{AutomergeDoc, AutomergeItems, DocUpdate};
use anyhow::Result;
use automerge::{Automerge, PatchAction};
use automerge::{ROOT};
use automorph::crdt::Text;
use automorph::Automorph;
use futures_util::StreamExt;
use samod::{BackoffConfig, DialerHandle, DocHandle, DocumentId, Repo, Url};
use std::num::ParseIntError;
use thiserror::Error;
use tokio::task::JoinHandle;
use tracing::{debug, error};

pub mod document;
pub(crate) mod security;

pub(crate) type LofiResult<T> = Result<T, LoFiError>;

#[derive(Error, Debug)]
pub enum LoFiError {
    #[error("Invalid password for the doc you are trying to open.")]
    InvalidPassword,
    #[error("Something went wrong while decrypting an item: {0}")]
    CouldNotDecrypt(String),
    #[error("Something went wrong while encrypting an item: {0}")]
    CouldNotEncrypt(String),
    #[error("Something went wrong while parsing a hex string to bytes: {0}")]
    CouldNotParseHex(#[from] ParseIntError),
    #[error("Something went wrong while parsing the document: {0}")]
    CouldNotParseDocument(String),
    #[error("Hex strings can may only have an even number of characters.")]
    InvalidLength,
    #[error(
        "Invalid Scrypt parameter. This should not happen, these are hardcoded, wtf did you do."
    )]
    InvalidScryptParams,
}

pub async fn connect(websocket_url: Url) -> Result<(Repo, DialerHandle)> {
    debug!("Initializing document-repo");

    let repo: Repo = Repo::build_tokio()
        .with_storage(samod::storage::TokioFilesystemStorage::new("./data"))
        .load()
        .await;

    debug!("Connecting to sync server...");

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

pub async fn open_document(
    repo: Repo,
    doc_id: DocumentId,
    _change_handler: Option<&dyn Fn(Vec<DocUpdate>) -> ()>,
) -> Result<(DocHandle, JoinHandle<()>)> {
    debug!("Looking for document...");
    let doc_handle = repo.find(doc_id.clone()).await?;

    match doc_handle {
        None => {
            error!("Document not found.");
            Err(anyhow::anyhow!(
                "Document not found. Make sure:\n  1. The sync server is running\n  2. The document exists in the browser\n  3. The document ID is correct"
            ))
        }
        Some(handle) => {
            debug!("Document found.");

            let change_handle = handle.clone();

            let handle_changes = async move {
                let mut change_stream_rx = change_handle.changes();
                let mut current_heads = change_handle.with_document(|d| d.get_heads());
                while let Some(change) = change_stream_rx.next().await {
                    let patches = change_handle
                        .with_document(|d| d.diff(&*current_heads, &*change.new_heads));

                    let changes: Vec<DocUpdate> = Vec::new();

                    // Hier prob. nur noch einmal data.update aufrufen und die changes bekommen

                    for patch in patches {
                        match patch.action {
                            PatchAction::PutMap { .. } => debug!("putM"),
                            PatchAction::PutSeq { .. } => debug!("putS"),
                            PatchAction::Insert { .. } => debug!("insert"),
                            PatchAction::SpliceText { .. } => debug!("text"),
                            PatchAction::Increment { .. } => todo!(),
                            PatchAction::Conflict { .. } => todo!(),
                            PatchAction::DeleteMap { .. } => debug!("delM"),
                            PatchAction::DeleteSeq { .. } => debug!("delS"),
                            PatchAction::Mark { .. } => todo!(),
                        }
                    }

                    current_heads = change_handle.with_document(|d| d.get_heads());
                }
            };

            let am_changes_handle = tokio::spawn(handle_changes);

            Ok((handle, am_changes_handle))
        }
    }
}

pub async fn get_doc_data(doc_handle: DocHandle) -> Result<AutomergeDoc> {
    // doc_handle.with_document(|doc| match Doc::load(doc, &ROOT, 0) {
    //     Ok(data) => Ok(data),
    //     Err(e) => {
    //         tracing::error!("Failed to hydrate document: {:?}", e);
    //         Err(anyhow::anyhow!("Failed to hydrate document: {:?}", e))
    //     }
    // })

    let salt = doc_handle.with_document(|doc| Text::load(doc, &ROOT, "salt"))?;
    let validation = doc_handle.with_document(|doc| Text::load(doc, &ROOT, "validation"))?;
    let items = doc_handle
        .with_document(|doc: &mut Automerge| AutomergeItems::load(doc, &ROOT, "items"))?;

    Ok(AutomergeDoc {
        salt,
        validation,
        items,
    })
}

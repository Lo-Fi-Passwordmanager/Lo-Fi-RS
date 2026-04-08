use anyhow::Result;
use lofi::automerge_document::{Doc, Item};
use lofi::security::crypter::Crypter;
use lofi::{connect, getDocData, openDocument};
use samod::{DocumentId, Url};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Initialize logging based on verbose flag
    tracing_subscriber::fmt()
        .with_env_filter(if true {
            tracing_subscriber::EnvFilter::new("lofi=debug")
        } else {
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn"))
        })
        .init();

    let url = Url::parse(
        &"wss://5bcaaf94-60ef-4757-b55c-5f2e443c480c.ka.bw-cloud-instance.org:443".to_string(),
    )
    .expect("valid WebSocket listener URL for samod");
    let doc_id: DocumentId = "3J14FKnxJStfFycHKszg2xdv5hRy".parse()?;

    let (repo, dialer_handle) = connect(url).await?;

    let (doc_handle, am_changes_handle) = openDocument(repo, doc_id, None).await?;

    let doc_data = getDocData(doc_handle).await?;

    if doc_data.items.len() == 0 {
        println!("This document has no items.");
        return Ok(());
    }

    let crypter = Crypter::from_doc(doc_data, "1");

    for item in &doc_data.items {
        match item {
            Item::WEntry(entry) => {
                println!("Entry: {:?}", decrypt(entry.name.as_str(), key));
            }
            Item::WFolder(folder) => {
                println!("Folder: {:?}", decrypt(folder.name.as_str(), key));
            }
        }
    }

    am_changes_handle.await?;

    Ok(())
}

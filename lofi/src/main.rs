use anyhow::Result;
use lofi::security::crypter::decrypt;
use lofi::security::keygen::derive_key;
use lofi::types::{Doc, Item};
use lofi::{connect, getDocData, openDocument};
use samod::{DocumentId, Url};
use scrypt::password_hash::SaltString;

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

    let doc_handle = openDocument(repo, dialer_handle, doc_id).await?;

    let doc_data: Doc = getDocData(doc_handle).await?;

    if doc_data.items.len() == 0 {
        println!("This document has no items.");
        return Ok(());
    }

    let key = derive_key("1", SaltString::from_b64(doc_data.salt.as_str())?)?;

    let first_item = &doc_data.items[0];

    match first_item {
        Item::WEntry(entry) => {
            println!("{:?}", decrypt(entry.name.as_str(), key));
        }
        Item::WFolder(folder) => {
            println!("{:?}", decrypt(folder.name.as_str(), key));
        }
    }

    Ok(())
}

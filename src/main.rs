use samod::{DocumentId, Url};
use anyhow::{Result};
use mylib::{connect, getDocData, openDocument};

mod types;

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

    let url = Url::parse(&"wss://5bcaaf94-60ef-4757-b55c-5f2e443c480c.ka.bw-cloud-instance.org:443".to_string())
        .expect("valid WebSocket listener URL for samod");
    let doc_id: DocumentId = "3J14FKnxJStfFycHKszg2xdv5hRy".parse()?;

    let (repo, dialer_handle) = connect(url).await?;

    let doc_handle = openDocument(repo, dialer_handle, doc_id).await?;

    let doc_data = getDocData(doc_handle).await?;
    println!("{:?}", doc_data);

    Ok(())
}
// use std::str::FromStr;
// use autosurgeon::{hydrate, HydrateError};
// use samod::{BackoffConfig, DialerHandle, DocumentId, Repo, Stopped, Url};
// use crate::types::Doc;
// 
// mod types;
// pub mod item_parser;
// 
// pub async fn lib() {
//     let repo: Repo = Repo::builder(tokio::runtime::Handle::current())
//         .with_storage(samod::storage::TokioFilesystemStorage::new("./data"))
//         .with_announce_policy(|_doc_id, _peer_id| false)
//         .load()
//         .await;
// 
//     let ws_url = Url::parse(&"wss://5bcaaf94-60ef-4757-b55c-5f2e443c480c.ka.bw-cloud-instance.org/".to_string())
//         .expect("valid WebSocket listener URL for samod");
//     let repo_dailer = repo.dial_websocket(ws_url, BackoffConfig::default());
// 
//     match repo_dailer {
//         Ok(_) => {
//             let doc_id = DocumentId::from_str("3J14FKnxJStfFycHKszg2xdv5hRy").unwrap();
//             let handle = repo.find(doc_id).await.unwrap().unwrap();
//             handle.with_document(|automerge_doc| -> () {
//                 let doc: Doc = hydrate(automerge_doc).unwrap();
// 
//                 println!("{:?}", doc)
//             })
//         }
//         Err(_) => {
//             todo!()
//         }
//     }
// }

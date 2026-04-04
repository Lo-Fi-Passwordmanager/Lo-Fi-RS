use autosurgeon::{Hydrate, Reconcile};
use lofi_macros::lofi_untagged;

#[derive(Debug, Clone, Hydrate, Reconcile, PartialEq)]
pub struct Doc {
    salt: autosurgeon::Text,
    validation: autosurgeon::Text,
    items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
#[lofi_untagged(Hydrate, Reconcile)]
pub enum Item {
    WEntry(Entry),
    WFolder(Folder),
}

#[derive(Debug, Clone, Hydrate, Reconcile, PartialEq)]
pub struct Entry {
    #[autosurgeon(rename = "type")]
    entry_type: autosurgeon::Text,
    name: autosurgeon::Text,
    #[autosurgeon(rename = "createdAt")]
    created_at: f64,
    #[autosurgeon(rename = "editedAt")]
    edited_at: f64,
    #[autosurgeon(rename = "parentId")]
    parent_id: autosurgeon::Text,
    username: autosurgeon::Text,
    password: autosurgeon::Text,
    url: autosurgeon::Text,
    note: autosurgeon::Text,
}

#[derive(Debug, Clone, Hydrate, Reconcile, PartialEq)]
pub struct Folder {
    #[autosurgeon(rename = "type")]
    entry_type: autosurgeon::Text,
    name: autosurgeon::Text,
    #[autosurgeon(rename = "createdAt")]
    created_at: f64,
    #[autosurgeon(rename = "editedAt")]
    edited_at: f64,
    #[autosurgeon(rename = "parentId")]
    parent_id: autosurgeon::Text,
}

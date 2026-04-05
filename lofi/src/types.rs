use autosurgeon::{Hydrate, Reconcile};
use lofi_macros::lofi_autosurgeon_untagged;

#[derive(Debug, Clone, Hydrate, Reconcile, PartialEq)]
pub struct Doc {
    pub salt: autosurgeon::Text,
    pub validation: autosurgeon::Text,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
#[lofi_autosurgeon_untagged(Hydrate, Reconcile)]
pub enum Item {
    WEntry(Entry),
    WFolder(Folder),
}

#[derive(Debug, Clone, Hydrate, Reconcile, PartialEq)]
pub struct Entry {
    #[autosurgeon(rename = "type")]
    pub entry_type: autosurgeon::Text,
    pub name: autosurgeon::Text,
    #[autosurgeon(rename = "createdAt")]
    pub created_at: f64,
    #[autosurgeon(rename = "editedAt")]
    pub edited_at: f64,
    #[autosurgeon(rename = "parentId")]
    pub parent_id: autosurgeon::Text,
    pub username: autosurgeon::Text,
    pub password: autosurgeon::Text,
    pub url: autosurgeon::Text,
    pub note: autosurgeon::Text,
}

#[derive(Debug, Clone, Hydrate, Reconcile, PartialEq)]
pub struct Folder {
    #[autosurgeon(rename = "type")]
    pub entry_type: autosurgeon::Text,
    pub name: autosurgeon::Text,
    #[autosurgeon(rename = "createdAt")]
    pub created_at: f64,
    #[autosurgeon(rename = "editedAt")]
    pub edited_at: f64,
    #[autosurgeon(rename = "parentId")]
    pub parent_id: autosurgeon::Text,
}

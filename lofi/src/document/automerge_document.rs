use automorph::automerge::ObjId;
use automorph::crdt::Text;
use automorph::Automorph;

#[derive(Debug, Clone, Automorph, PartialEq)]
pub struct AutomergeDoc {
    pub salt: Text,
    pub validation: Text,
    pub items: AutomergeItems,
}

pub type AutomergeItems = Vec<AutomergeItem>;

#[derive(Debug, Clone, Automorph, PartialEq)]
#[automorph(untagged)]
pub enum AutomergeItem {
    WEntry(AutomergeEntry),
    WFolder(AutomergeFolder),
}

#[derive(Debug, Clone, Automorph, PartialEq)]
#[automorph(rename_all = "camelCase")]
pub struct AutomergeEntry {
    #[automorph(rename = "type")]
    pub item_type: Text,
    pub name: Text,
    pub created_at: f64,
    pub edited_at: f64,
    pub parent_id: Text,
    pub username: Text,
    pub password: Text,
    pub url: Text,
    pub note: Text,
}

#[derive(Debug, Clone, Automorph, PartialEq)]
#[automorph(rename_all = "camelCase")]
pub struct AutomergeFolder {
    #[automorph(rename = "type")]
    pub item_type: Text,
    pub name: Text,
    pub created_at: f64,
    pub edited_at: f64,
    pub parent_id: Text,
}

pub struct DocUpdate {
    obj_id: ObjId,
    update: Update,
}

pub enum Update {
    UpdateMapElement,
    DeleteMapElement,
    UpdateArrayElement,
    InsertArrayElement,
    DeleteArrayElement,
    UpdateTextElement,
}

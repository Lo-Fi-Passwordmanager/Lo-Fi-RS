use crate::hydrate::Unexpected::Text;
use std::collections::HashMap;
use std::iter::Map;
use automerge::iter::MapRangeItem;
use automerge::ObjType;
use autosurgeon::{Hydrate, HydrateError, Prop, ReadDoc, Reconcile, Reconciler};

#[derive(Debug, Clone, Hydrate, Reconcile, PartialEq)]
pub struct Doc {
    salt: autosurgeon::Text,
    validation: autosurgeon::Text,
    items: Vec<Item>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    WEntry(Entry),
    WFolder(Folder)
}

impl Reconcile for Item {
    type Key<'a> = ();

    fn reconcile<R: Reconciler>(&self, reconciler: R) -> Result<(), R::Error> {
        match self {
            Item::WEntry(entry) => {entry.reconcile(reconciler)}
            Item::WFolder(folder) => {folder.reconcile(reconciler)}
        }
    }
}

impl Hydrate for Item {
    fn hydrate_map<D: ReadDoc>(doc: &D, obj: &automerge::ObjId) -> Result<Self, HydrateError> {

        let Some(obj_type) = doc.object_type(obj) else {
            return Err(HydrateError::unexpected(
                "an item",
                "a scalar value".to_string(),
            ));
        };

        match obj_type {
            ObjType::Map | ObjType::Table => {
                let entries: HashMap<String, MapRangeItem> = doc
                .map_range(obj.clone(), ..).map(move |item| {
                    let key = item.key.as_ref();
                    (key.to_string(), item)
                }).collect();

                let t = entries.get("type").unwrap();

                let val = String::hydrate(doc, obj, t.key.as_ref().into())?;

                match val.as_str() {
                    "entry" => { Ok(Item::WEntry(Entry {
                        entry_type: autosurgeon::Text::with_value(val),
                        name: autosurgeon::Text::with_value(String::hydrate(doc, obj, entries.get("name").unwrap().key.as_ref().into())?),
                        created_at: u32::hydrate(doc, obj, entries.get("createdAt").unwrap().key.as_ref().into())?,
                        edited_at: u32::hydrate(doc, obj, entries.get("editedAt").unwrap().key.as_ref().into())?,
                        parent_id: autosurgeon::Text::with_value(String::hydrate(doc, obj, entries.get("parentId").unwrap().key.as_ref().into())?),
                        username: autosurgeon::Text::with_value(String::hydrate(doc, obj, entries.get("username").unwrap().key.as_ref().into())?),
                        password: autosurgeon::Text::with_value(String::hydrate(doc, obj, entries.get("password").unwrap().key.as_ref().into())?),
                        url: autosurgeon::Text::with_value(String::hydrate(doc, obj, entries.get("url").unwrap().key.as_ref().into())?),
                        note: autosurgeon::Text::with_value(String::hydrate(doc, obj, entries.get("note").unwrap().key.as_ref().into())?),
                    })) }
                    "folder" => { Ok(Item::WFolder(Folder {
                        entry_type: autosurgeon::Text::with_value(val),
                        name: autosurgeon::Text::with_value(String::hydrate(doc, obj, entries.get("name").unwrap().key.as_ref().into())?),
                        created_at: u32::hydrate(doc, obj, entries.get("createdAt").unwrap().key.as_ref().into())?,
                        edited_at: u32::hydrate(doc, obj, entries.get("editedAt").unwrap().key.as_ref().into())?,
                        parent_id: autosurgeon::Text::with_value(String::hydrate(doc, obj, entries.get("parentId").unwrap().key.as_ref().into())?),
                    })) }
                    _ => Err(HydrateError::unexpected(
                        "an item",
                        "a text object".to_string(),
                    ))
                }
            },
            ObjType::Text => Err(HydrateError::unexpected(
                "an item",
                "a text object".to_string(),
            )),
            ObjType::List => Err(HydrateError::unexpected(
                "an item",
                "a list object".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
pub struct Entry {
    #[autosurgeon(rename = "type")]
    entry_type: autosurgeon::Text,
    name: autosurgeon::Text,
    #[autosurgeon(rename = "createdAt")]
    created_at: u32,
    #[autosurgeon(rename = "editedAt")]
    edited_at: u32,
    #[autosurgeon(rename = "parentId")]
    parent_id: autosurgeon::Text,
    username: autosurgeon::Text,
    password: autosurgeon::Text,
    url: autosurgeon::Text,
    note: autosurgeon::Text,
}

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
pub struct Folder {
    #[autosurgeon(rename = "type")]
    entry_type: autosurgeon::Text,
    name: autosurgeon::Text,
    #[autosurgeon(rename = "createdAt")]
    created_at: u32,
    #[autosurgeon(rename = "editedAt")]
    edited_at: u32,
    #[autosurgeon(rename = "parentId")]
    parent_id: autosurgeon::Text,
}
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use tracing::debug;
use crate::document::automerge_document::{
    AutomergeDoc, AutomergeEntry, AutomergeFolder, AutomergeItem,
};
use crate::security::crypter::Crypter;
use crate::{LoFiError, LofiResult};
// TODO Impl from und Into für AutomergeDoc und items, ...

trait FromAutomerge<From, Into> {
    fn from_automerge(value: &From, crypter: &Crypter) -> LofiResult<Into>;
}

trait ItemAttr {
    fn title(&self) -> String;
    fn id(&self) -> String;
    fn created_at(&self) -> f64;
    fn edited_at(&self) -> f64;
}

trait FolderFunc {
    fn insert(&mut self, item: Rc<RefCell<Item>>) -> ();
    fn items(&self) -> &[Rc<RefCell<Item>>];
    fn get_item_by_id(&self, id: &str) -> Option<Rc<RefCell<Item>>>;
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct LofiDocument(Folder);

impl FolderFunc for LofiDocument {
    fn insert(&mut self, item: Rc<RefCell<Item>>) -> () {
        self.0.items.push(item);
    }

    fn items(&self) -> &[Rc<RefCell<Item>>] {
        &self.0.items
    }

    fn get_item_by_id(&self, id: &str) -> Option<Rc<RefCell<Item>>> {
        self.0.items.iter().find(|item| {
            item.borrow().id() == id
        }).cloned()
    }
}

impl LofiDocument {
    fn new() -> Self {
        LofiDocument(Folder{
            title: "".to_string(),
            id: "".to_string(),
            created_at: 0.0,
            edited_at: 0.0,
            items: Vec::new(),
        })
    }
    
    fn folder(&self) -> Folder {
        self.0
    }
}

impl FromAutomerge<AutomergeDoc, Self> for LofiDocument {
    fn from_automerge(value: &AutomergeDoc, crypter: &Crypter) -> LofiResult<Self> {

        let mut base: LofiDocument = LofiDocument::new();

        let mut automerge_items_by_id: HashMap<String, AutomergeItem> = HashMap::new();

        for item in &value.items {
            automerge_items_by_id.insert(item.id().to_string(), (*item).clone());
        }

        let mut path_length_by_item: Vec<(&AutomergeItem, u32)> = Vec::new();

        for item in &value.items {
            path_length_by_item.push((item, path_length(item, &automerge_items_by_id)));
        }

        // Nach pfadlänge sortieren, damit auf jeden fall immer die eltern zuerst eingesetzt werden
        path_length_by_item.sort_by(|a, b| {
            a.1.cmp(&b.1)
        });

        let mut items_by_id: HashMap<String, Rc<RefCell<Item>>> = HashMap::new();

        for (item, path) in path_length_by_item {
            if item.parent_id() == "" {
                items_by_id.insert(item.id().to_string(), Rc::from(RefCell::from(Item::from_automerge(item, crypter)?)));
            } else {
                let parent = items_by_id.get_mut(&item.parent_id());
                let i = Item::from_automerge(item, crypter)?;
                match parent {
                    None => Err(LoFiError::CouldNotParseDocument(format!("Child with ID {} does not exist on Element with ID {}.", &item.parent_id(), item.id())))?,
                    Some(p) => match *p.borrow_mut() {
                        Item::Folder(mut folder) => folder.insert(Rc::from(RefCell::from(i))),
                        Item::Entry(_) => Err(LoFiError::CouldNotParseDocument("Cannot insert value into Entry.".to_string()))?
                    },
                }
                items_by_id.insert((&i).id().to_string(), Rc::from(RefCell::from(i));)
            }
        }

        for (_, value) in items_by_id {
            base.insert(value);
        }

        Ok(base)
    }
}

fn find_and_insert(root: &Rc<RefCell<Item>>, target_id: &str, new_node: Rc<RefCell<Item>>) -> bool {
    let mut node = root.borrow_mut();

    // If this is the node we are looking for, push the child
    if &node.id() == target_id {
        match *node {
            Item::Entry(_) => {},
            Item::Folder(mut n) => {
                n.items.push(new_node);
            }
        }

        return true;
    }

    match *node {
        Item::Entry(_) => {},
        Item::Folder(mut n) => {
            for child in &n.items {
                if find_and_insert(child, target_id, new_node.clone()) {
                    return true;
                }
            }
        }
    }

    // Otherwise, check all children recursively


    false
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Item {
    Folder(Folder),
    Entry(Entry),
}

impl Item {
    pub fn id(&self) -> String {
        match self {
            Item::Folder(entry) => entry.id.clone(),
            Item::Entry(folder) => folder.id.clone()
        }
    }
}

impl FromAutomerge<AutomergeItem, Self> for Item {
    fn from_automerge(value: &AutomergeItem, crypter: &Crypter) -> LofiResult<Self> {
        match value {
            AutomergeItem::WEntry(inner) => Ok(Item::Entry(Entry::from_automerge(inner, crypter)?)),
            AutomergeItem::WFolder(inner) => {
                Ok(Item::Folder(Folder::from_automerge(inner, crypter)?))
            }
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Folder {
    title: String,
    id: String,
    created_at: f64,
    edited_at: f64,
    items: Vec<Rc<RefCell<Item>>>,
}

impl ItemAttr for Folder {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn created_at(&self) -> f64 {
        self.created_at
    }

    fn edited_at(&self) -> f64 {
        self.edited_at
    }
}

impl FolderFunc for Folder {
    fn insert(&mut self, item: Rc<RefCell<Item>>) -> () {
        self.items.push(item);
    }

    fn items(&self) -> &[Rc<RefCell<Item>>] {
        &self.items
    }

    fn get_item_by_id(&self, id: &str) -> Option<Rc<RefCell<Item>>> {
        self.items.iter().find(|item| {
            item.borrow().id() == id
        }).cloned()
    }
}

impl FromAutomerge<AutomergeFolder, Self> for Folder {
    fn from_automerge(value: &AutomergeFolder, crypter: &Crypter) -> LofiResult<Self> {
        todo!()
    }
}

impl Folder {
    fn items(&self) -> Vec<Item> {
        self.items.clone()
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Entry {
    title: String,
    id: String,
    created_at: f64,
    edited_at: f64,
    username: String,
    password: String,
    url: String,
    note: String,
}

impl ItemAttr for Entry {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn created_at(&self) -> f64 {
        self.created_at
    }

    fn edited_at(&self) -> f64 {
        self.edited_at
    }
}

impl FromAutomerge<AutomergeEntry, Self> for Entry {
    fn from_automerge(value: &AutomergeEntry, crypter: &Crypter) -> LofiResult<Self> {
        todo!()
    }
}

impl Entry {
    fn username(&self) -> String {
        self.username.clone()
    }
    fn password(&self) -> String {
        self.password.clone()
    }
    fn url(&self) -> String {
        self.url.clone()
    }
    fn note(&self) -> String {
        self.note.clone()
    }
}

fn path_length(item: &AutomergeItem, items_by_id: &HashMap<String, AutomergeItem>) -> u32 {
    if item.parent_id().is_empty() {
        return 0;
    }

    let parent = items_by_id.get(&item.parent_id());

    match parent {
        None => {
            debug!("Found item with invalid parent id, treating as empty parent id.");
            0
        }
        Some(parent) => {
            path_length(parent, items_by_id) + 1
        }
    }
}
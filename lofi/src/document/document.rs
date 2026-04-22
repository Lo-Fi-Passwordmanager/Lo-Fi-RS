use std::cell::{RefCell};
use std::collections::HashMap;
use std::ops::{DerefMut};
use std::rc::Rc;
use tracing::debug;
use crate::document::automerge_document::{
    AutomergeDoc, AutomergeEntry, AutomergeFolder, AutomergeItem,
};
use crate::security::crypter::Crypter;
use crate::{LofiResult};
// TODO Impl from und Into für AutomergeDoc und items, ...

pub(crate) trait FromAutomerge<From, Into> {
    fn from_automerge(value: &From, crypter: &Crypter) -> LofiResult<Into>;
}

pub trait ItemAttr {
    fn title(&self) -> String;
    fn id(&self) -> String;
    fn created_at(&self) -> f64;
    fn edited_at(&self) -> f64;
}

// TODO Split trait in public (items, getbyid) and non public (insert)
pub trait FolderFunc {
    fn insert(&mut self, item: Rc<RefCell<Item>>) -> ();
    fn items(&self) -> Vec<Item>;
    fn get_item_by_id(&self, id: &str) -> Option<Item>;
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct LofiDocument(Rc<RefCell<Folder>>);

impl FolderFunc for LofiDocument {
    fn insert(&mut self, item: Rc<RefCell<Item>>) -> () {
        self.0.borrow_mut().items.push(item);
    }

    fn items(&self) -> Vec<Item> {
        let items: Vec<Item> = self.0.borrow().items().iter()
            .map(|rc| {
                // 1. .borrow() gives a Ref<Item>
                // 2. .clone() creates a new Item instance
                rc.borrow().clone()
            })
            .collect();
        items
    }

    fn get_item_by_id(&self, id: &str) -> Option<Item> {
        self.0.borrow().items.iter().find(|item| {
            item.borrow().id() == id
        }).cloned().map(|rc| {
            rc.borrow().clone()
        })
    }
}

impl LofiDocument {
    fn new() -> Self {
        LofiDocument(Rc::from(RefCell::from(Folder{
            title: "".to_string(),
            id: "".to_string(),
            created_at: 0.0,
            edited_at: 0.0,
            items: Vec::new(),
        })))
    }

    fn folder(&self) -> Rc<RefCell<Folder>> {
        self.0.clone()
    }
}

impl FromAutomerge<AutomergeDoc, Self> for LofiDocument {
    fn from_automerge(value: &AutomergeDoc, crypter: &Crypter) -> LofiResult<Self> {

        let mut base: LofiDocument = LofiDocument::new();

        let mut automerge_items_by_id: HashMap<String, AutomergeItem> = HashMap::new();

        for item in &value.items {
            automerge_items_by_id.insert(item.id().to_string(), (*item).clone());
        }

        let mut path_by_item: Vec<(&AutomergeItem, u32, Vec<String>)> = Vec::new();

        for item in &value.items {
            let (len, path) = path(item, &automerge_items_by_id);
            path_by_item.push((item, len, path));
        }

        // Nach pfadlänge sortieren, damit auf jeden fall immer die eltern zuerst eingesetzt werden
        path_by_item.sort_by(|a, b| {
            a.1.cmp(&b.1)
        });

        for (item, len, path) in path_by_item {
            if len == 0 {
                base.insert(Rc::from(RefCell::from(Item::from_automerge(item, crypter)?)))
            } else {
                let i = Item::from_automerge(item, crypter)?;
                find_and_insert(&Rc::from(RefCell::from(Item::Folder(base.folder()))), &path, Rc::new(RefCell::new(i)));
            }
        }


        Ok(base)
    }
}

fn find_and_insert(root: &Rc<RefCell<Item>>, path: &[String], new_node: Rc<RefCell<Item>>) -> bool {
    let mut node = root.borrow_mut();

    // If this is the node we are looking for, push the child
    if path.len() == 1 && node.id() == path[0] {
        match node.deref_mut() {
            Item::Entry(_) => {},
            Item::Folder(n) => {
                n.clone().borrow_mut().items.push(new_node);
            }
        }

        return true;
    }

    match node.deref_mut() {
        Item::Entry(_) => {},
        Item::Folder(n) => {
            for child in &n.clone().borrow_mut().items {
                if find_and_insert(child, &path[1..], new_node.clone()) {
                    return true;
                }
            }
        }
    }
    false
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Item {
    Folder(Rc<RefCell<Folder>>),
    Entry(Rc<RefCell<Entry>>),
}

impl Item {
    pub fn id(&self) -> String {
        match self {
            Item::Folder(entry) => entry.borrow().id.clone(),
            Item::Entry(folder) => folder.borrow().id.clone()
        }
    }
}

impl FromAutomerge<AutomergeItem, Self> for Item {
    fn from_automerge(value: &AutomergeItem, crypter: &Crypter) -> LofiResult<Self> {
        match value {
            AutomergeItem::WEntry(inner) => Ok(Item::Entry(Rc::from(RefCell::from(Entry::from_automerge(inner, crypter)?)))),
            AutomergeItem::WFolder(inner) => {
                Ok(Item::Folder(Rc::from(RefCell::from(Folder::from_automerge(inner, crypter)?))))
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

    fn items(&self) -> Vec<Item> {
        let items: Vec<Item> = self.items.iter()
            .map(|rc| {
                rc.borrow().clone()
            })
            .collect();
        items
    }

    fn get_item_by_id(&self, id: &str) -> Option<Item> {
        self.items.iter().find(|item| {
            item.borrow().id() == id
        }).cloned().map(|rc| {
            rc.borrow().clone()
        })
    }
}

impl FromAutomerge<AutomergeFolder, Self> for Folder {
    fn from_automerge(value: &AutomergeFolder, crypter: &Crypter) -> LofiResult<Self> {
        Ok(Self {
            title: crypter.decrypt(&value.name)?,
            id: value.id.to_string(),
            created_at: value.created_at,
            edited_at: value.edited_at,
            items: Vec::new()
        })
    }
}

impl Folder {
    fn items(&self) -> Vec<Rc<RefCell<Item>>> {
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
        Ok(Self {
            title: crypter.decrypt(&value.name)?,
            id: value.id.to_string(),
            created_at: value.created_at,
            edited_at: value.edited_at,
            username: crypter.decrypt(&value.username)?,
            password: crypter.decrypt(&value.password)?,
            url: crypter.decrypt(&value.url)?,
            note: crypter.decrypt(&value.note)?,
        })
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

fn path(item: &AutomergeItem, items_by_id: &HashMap<String, AutomergeItem>) -> (u32, Vec<String>) {
    if item.parent_id().is_empty() {
        return (0, Vec::new());
    }

    let parent = items_by_id.get(&item.parent_id());

    match parent {
        None => {
            debug!("Found item with invalid parent id, treating as empty parent id.");
            (0, Vec::new())
        }
        Some(parent) => {
            let (len, mut p) = path(parent, items_by_id);
            p.push(item.parent_id());
            (len + 1, p)
        }
    }
}
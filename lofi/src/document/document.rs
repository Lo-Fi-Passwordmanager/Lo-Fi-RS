use crate::document::automerge_document::{
    AutomergeDoc, AutomergeEntry, AutomergeFolder, AutomergeItem,
};
use crate::security::crypter::Crypter;
use crate::LofiResult;

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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct LofiDocument {
    items: Vec<Item>,
}

impl FromAutomerge<AutomergeDoc, Self> for LofiDocument {
    fn from_automerge(value: &AutomergeDoc, crypter: &Crypter) -> LofiResult<Self> {
        let items: LofiResult<Vec<Item>> = value
            .items
            .iter()
            .map(|item| Item::from_automerge(item, crypter))
            .collect();

        Ok(LofiDocument { items: items? })
    }
}

//     const root = new DatabaseRoot(automergeDoc.salt);
//
//     const itemsById = new Map<string, AutomergeItem>();
//     // Die items in eine map packen, wo sie schnell nach id erreichbar sind
//     for (const item of automergeDoc.items) {
//         const id = getObjectId(item)!;
//         itemsById.set(id, item);
//     }
//
//     const pathByItem = new Map<AutomergeItem, Array<string>>();
//     // Eine map aufbauen, in der zu jedem item der pfad steht (pfad so wie vorher, also array an ids)
//     for (const item of automergeDoc.items) {
//         const path = buildPath(item, itemsById);
//         pathByItem.set(item, path);
//     }
//
//     // Nach pfadlänge sortieren, damit auf jeden fall immer die eltern zuerst eingesetzt werden
//     const sortedByPathLength = new Map([...pathByItem.entries()].sort((a, b) => a[1].length - b[1].length));
//
//     // Der pfadlänge nach in den passwordmanagerroot einsetzen
//     for (const [item, path] of sortedByPathLength) {
//         insertNestedValue(root, path, databaseItemFromAutomergeItem(item, securityProvider)); // gleiche fkt wie früher
//     }
//
//     return [root, itemsById];
// }
//
//function insertNestedValue(databaseRoot: DatabaseRoot, path: string[], insert: Item) {
//     const value = findNestedValue(databaseRoot, path);
//     if (!value.isFolder()) {
//         throw Error("Cannot insert value into Entry.");
//     }
//     (value as Folder).addItem(insert);
// }
//
//function findNestedValue(databaseRoot: DatabaseRoot, path: string[]): Item {
//     if (path.length === 0) {
//         return databaseRoot.rootFolder;
//     }
//
//     let currentValue: Item | null = databaseRoot.rootFolder.getChildById(path[0]);
//
//     if (currentValue === null) {
//         throw Error(`Child with ID ${path[0]} does not exist on DatabaseRoot.`);
//     }
//
//     path.slice(1).forEach((id) => {
//         if (currentValue!.isFolder()) {
//             const nestedValue = (currentValue! as Folder).getChildById(id);
//
//             if (nestedValue === null) {
//                 throw Error(`Child with ID ${id} does not exist on Element with ID ${currentValue!.id}.`);
//             }
//
//             currentValue = nestedValue;
//         } else {
//             throw Error("Cannot index into Entry because it has no children");
//         }
//     });
//
//     return currentValue;
// }

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Item {
    Folder(Folder),
    Entry(Entry),
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
    items: Vec<Item>,
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

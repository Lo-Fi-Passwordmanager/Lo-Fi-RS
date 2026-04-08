#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct LofiDocument {
    items: Vec<Item>,
}

// TODO Impl from und Into für AutomergeDoc und items, ...

trait ItemAttr {
    fn title(&self) -> String;
    fn id(&self) -> String;
    fn created_at(&self) -> f64;
    fn edited_at(&self) -> f64;
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Item {
    Folder(Folder),
    Entry(Entry),
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

#![feature(prelude_import)]
extern crate std;
use automorph::automerge::ObjId;
#[prelude_import]
use automorph::Automorph;
pub struct Doc {
    pub salt: String,
    pub validation: String,
    pub items: Vec<Item>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Doc {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Doc",
            "salt",
            &self.salt,
            "validation",
            &self.validation,
            "items",
            &&self.items,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Doc {
    #[inline]
    fn clone(&self) -> Doc {
        Doc {
            salt: ::core::clone::Clone::clone(&self.salt),
            validation: ::core::clone::Clone::clone(&self.validation),
            items: ::core::clone::Clone::clone(&self.items),
        }
    }
}
#[doc = r" Change report for #ident."]
#[doc = r" Each field contains `None` if unchanged, or `Some(nested_changes)` if changed."]
pub struct DocChanges {
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub salt: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub validation: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub items: Option<<Vec<Item> as automorph::Automorph>::Changes>,
}
#[automatically_derived]
impl ::core::fmt::Debug for DocChanges {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "DocChanges",
            "salt",
            &self.salt,
            "validation",
            &self.validation,
            "items",
            &&self.items,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for DocChanges {
    #[inline]
    fn clone(&self) -> DocChanges {
        DocChanges {
            salt: ::core::clone::Clone::clone(&self.salt),
            validation: ::core::clone::Clone::clone(&self.validation),
            items: ::core::clone::Clone::clone(&self.items),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for DocChanges {
    #[inline]
    fn default() -> DocChanges {
        DocChanges {
            salt: ::core::default::Default::default(),
            validation: ::core::default::Default::default(),
            items: ::core::default::Default::default(),
        }
    }
}
impl automorph::ChangeReport for DocChanges {
    fn any(&self) -> bool {
        false || self.salt.is_some() || self.validation.is_some() || self.items.is_some()
    }
    fn paths(&self) -> Vec<Vec<std::borrow::Cow<'static, str>>> {
        let mut paths = Vec::new();
        if let Some(ref field_changes) = self.salt {
            paths.push(vec![std::borrow::Cow::Borrowed("salt")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("salt")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.validation {
            paths.push(vec![std::borrow::Cow::Borrowed("validation")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("validation")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.items {
            paths.push(vec![std::borrow::Cow::Borrowed("items")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("items")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        paths
    }
    fn leaf_paths(&self) -> Vec<Vec<std::borrow::Cow<'static, str>>> {
        let mut paths = Vec::new();
        if let Some(ref field_changes) = self.salt {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("salt")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("salt")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.validation {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("validation")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("validation")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.items {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("items")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("items")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        paths
    }
}
#[doc = r" Cursor for tracking #ident changes efficiently."]
pub struct DocCursor {
    #[doc = r" Cursor for this field."]
    pub salt: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub validation: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub items: <Vec<Item> as automorph::Automorph>::Cursor,
}
#[automatically_derived]
impl ::core::fmt::Debug for DocCursor {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "DocCursor",
            "salt",
            &self.salt,
            "validation",
            &self.validation,
            "items",
            &&self.items,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for DocCursor {
    #[inline]
    fn clone(&self) -> DocCursor {
        DocCursor {
            salt: ::core::clone::Clone::clone(&self.salt),
            validation: ::core::clone::Clone::clone(&self.validation),
            items: ::core::clone::Clone::clone(&self.items),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for DocCursor {
    #[inline]
    fn default() -> DocCursor {
        DocCursor {
            salt: ::core::default::Default::default(),
            validation: ::core::default::Default::default(),
            items: ::core::default::Default::default(),
        }
    }
}
impl automorph::FieldCursor for DocCursor {
    type Changes = DocChanges;
    fn diff<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: &automerge::ObjId,
    ) -> automorph::Result<Self::Changes> {
        let mut changes = DocChanges::default();
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "salt")? {
                let field_changes = automorph::FieldCursor::diff(&self.salt, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.salt = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "validation")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.validation, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.validation = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "items")? {
                let field_changes = automorph::FieldCursor::diff(&self.items, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.items = Some(field_changes);
                }
            }
        }
        Ok(changes)
    }
    fn refresh<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: &automerge::ObjId,
    ) -> automorph::Result<()> {
        if let Some((_, field_obj_id)) = doc.get(obj, "salt")? {
            automorph::FieldCursor::refresh(&mut self.salt, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "validation")? {
            automorph::FieldCursor::refresh(&mut self.validation, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "items")? {
            automorph::FieldCursor::refresh(&mut self.items, doc, &field_obj_id)?;
        }
        Ok(())
    }
}
impl automorph::Automorph for Doc {
    type Changes = DocChanges;
    type Cursor = DocCursor;
    fn save<D: automerge::transaction::Transactable + automerge::ReadDoc>(
        &self,
        doc: &mut D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<()> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        let cur = doc.get(obj, prop.clone())?;
        let map_id = match cur {
            Some((automerge::Value::Object(automerge::ObjType::Map), id)) => id,
            _ => doc.put_object(obj, prop, automerge::ObjType::Map)?,
        };
        automorph::Automorph::save(&self.salt, doc, &map_id, "salt")?;
        automorph::Automorph::save(&self.validation, doc, &map_id, "validation")?;
        automorph::Automorph::save(&self.items, doc, &map_id, "items")?;
        Ok(())
    }
    fn load<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get(obj, prop)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => Ok(Self {
                salt: automorph::Automorph::load(doc, &map_id, "salt")
                    .map_err(|e| e.with_field("salt"))?,
                validation: automorph::Automorph::load(doc, &map_id, "validation")
                    .map_err(|e| e.with_field("validation"))?,
                items: automorph::Automorph::load(doc, &map_id, "items")
                    .map_err(|e| e.with_field("items"))?,
                au
            }),
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Doc",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn load_at<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get_at(obj, prop, heads)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => Ok(Self {
                salt: automorph::Automorph::load_at(doc, &map_id, "salt", heads)
                    .map_err(|e| e.with_field("salt"))?,
                validation: automorph::Automorph::load_at(doc, &map_id, "validation", heads)
                    .map_err(|e| e.with_field("validation"))?,
                items: automorph::Automorph::load_at(doc, &map_id, "items", heads)
                    .map_err(|e| e.with_field("items"))?,
            }),
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Doc",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn diff<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get(obj, prop)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = DocChanges::default();
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.salt, doc, &map_id, "salt")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.salt = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.validation, doc, &map_id, "validation")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.validation = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.items, doc, &map_id, "items")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.items = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Doc",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn diff_at<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get_at(obj, prop, heads)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = DocChanges::default();
                {
                    let field_changes =
                        automorph::Automorph::diff_at(&self.salt, doc, &map_id, "salt", heads)?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.salt = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.validation,
                        doc,
                        &map_id,
                        "validation",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.validation = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff_at(&self.items, doc, &map_id, "items", heads)?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.items = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Doc",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn update<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get(obj, prop)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = DocChanges::default();
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.salt, doc, &map_id, "salt")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.salt = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update(
                        &mut self.validation,
                        doc,
                        &map_id,
                        "validation",
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.validation = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.items, doc, &map_id, "items")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.items = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Doc",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn update_at<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get_at(obj, prop, heads)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = DocChanges::default();
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.salt,
                        doc,
                        &map_id,
                        "salt",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.salt = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.validation,
                        doc,
                        &map_id,
                        "validation",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.validation = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.items,
                        doc,
                        &map_id,
                        "items",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.items = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Doc",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn save_flat<D: automerge::transaction::Transactable + automerge::ReadDoc>(
        &self,
        doc: &mut D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<()> {
        let map_id = obj.as_ref();
        automorph::Automorph::save(&self.salt, doc, &map_id, "salt")?;
        automorph::Automorph::save(&self.validation, doc, &map_id, "validation")?;
        automorph::Automorph::save(&self.items, doc, &map_id, "items")?;
        Ok(())
    }
    fn load_flat<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<Self> {
        let map_id = obj.as_ref();
        Ok(Self {
            salt: automorph::Automorph::load(doc, &map_id, "salt")
                .map_err(|e| e.with_field("salt"))?,
            validation: automorph::Automorph::load(doc, &map_id, "validation")
                .map_err(|e| e.with_field("validation"))?,
            items: automorph::Automorph::load(doc, &map_id, "items")
                .map_err(|e| e.with_field("items"))?,
        })
    }
    fn load_flat_at<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self> {
        let map_id = obj.as_ref();
        Ok(Self {
            salt: automorph::Automorph::load_at(doc, &map_id, "salt", heads)
                .map_err(|e| e.with_field("salt"))?,
            validation: automorph::Automorph::load_at(doc, &map_id, "validation", heads)
                .map_err(|e| e.with_field("validation"))?,
            items: automorph::Automorph::load_at(doc, &map_id, "items", heads)
                .map_err(|e| e.with_field("items"))?,
        })
    }
    fn diff_flat<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = DocChanges::default();
        {
            let field_changes = automorph::Automorph::diff(&self.salt, doc, &map_id, "salt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.salt = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff(&self.validation, doc, &map_id, "validation")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.validation = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::diff(&self.items, doc, &map_id, "items")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.items = Some(field_changes);
            }
        }
        Ok(changes)
    }
    fn diff_flat_at<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = DocChanges::default();
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.salt, doc, &map_id, "salt", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.salt = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.validation, doc, &map_id, "validation", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.validation = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.items, doc, &map_id, "items", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.items = Some(field_changes);
            }
        }
        Ok(changes)
    }
    fn update_flat<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = DocChanges::default();
        {
            let field_changes = automorph::Automorph::update(&mut self.salt, doc, &map_id, "salt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.salt = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.validation, doc, &map_id, "validation")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.validation = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.items, doc, &map_id, "items")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.items = Some(field_changes);
            }
        }
        Ok(changes)
    }
    fn update_flat_at<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = DocChanges::default();
        {
            let field_changes =
                automorph::Automorph::update_at(&mut self.salt, doc, &map_id, "salt", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.salt = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update_at(
                &mut self.validation,
                doc,
                &map_id,
                "validation",
                heads,
            )?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.validation = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update_at(&mut self.items, doc, &map_id, "items", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.items = Some(field_changes);
            }
        }
        Ok(changes)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Doc {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Doc {
    #[inline]
    fn eq(&self, other: &Doc) -> bool {
        self.salt == other.salt && self.validation == other.validation && self.items == other.items
    }
}
#[automorph(untagged)]
pub enum Item {
    WEntry(Entry),
    WFolder(Folder),
}
#[automatically_derived]
impl ::core::fmt::Debug for Item {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Item::WEntry(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "WEntry", &__self_0)
            }
            Item::WFolder(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "WFolder", &__self_0)
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Item {
    #[inline]
    fn clone(&self) -> Item {
        match self {
            Item::WEntry(__self_0) => Item::WEntry(::core::clone::Clone::clone(__self_0)),
            Item::WFolder(__self_0) => Item::WFolder(::core::clone::Clone::clone(__self_0)),
        }
    }
}
impl automorph::Automorph for Item
where
    Self: PartialEq,
{
    type Changes = automorph::PrimitiveChanged;
    type Cursor = automorph::ScalarCursor;
    fn save<D: automerge::transaction::Transactable + automerge::ReadDoc>(
        &self,
        doc: &mut D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<()> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match self {
            Self::WEntry(inner) => {
                inner.save(doc, obj, prop)?;
            }
            Self::WFolder(inner) => {
                inner.save(doc, obj, prop)?;
            }
        }
        Ok(())
    }
    fn load<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self> {
        let prop: automerge::Prop = prop.into();
        if let Ok(inner) = <Entry>::load(doc, obj.as_ref(), prop.clone()) {
            return Ok(Self::WEntry(inner));
        }
        if let Ok(inner) = <Folder>::load(doc, obj.as_ref(), prop.clone()) {
            return Ok(Self::WFolder(inner));
        }
        Err(automorph::Error::invalid_value(
            "no variant matched for untagged enum",
        ))
    }
    fn load_at<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self> {
        let prop: automerge::Prop = prop.into();
        if let Ok(inner) = <Entry>::load_at(doc, obj.as_ref(), prop.clone(), heads) {
            return Ok(Self::WEntry(inner));
        }
        if let Ok(inner) = <Folder>::load_at(doc, obj.as_ref(), prop.clone(), heads) {
            return Ok(Self::WFolder(inner));
        }
        Err(automorph::Error::invalid_value(
            "no variant matched for untagged enum",
        ))
    }
    fn diff<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self::Changes> {
        let loaded = Self::load(doc, obj, prop)?;
        Ok(automorph::PrimitiveChanged::new(self != &loaded))
    }
    fn diff_at<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let loaded = Self::load_at(doc, obj, prop, heads)?;
        Ok(automorph::PrimitiveChanged::new(self != &loaded))
    }
    fn update<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self::Changes> {
        let loaded = Self::load(doc, obj, prop)?;
        let changed = self != &loaded;
        *self = loaded;
        Ok(automorph::PrimitiveChanged::new(changed))
    }
    fn update_at<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let loaded = Self::load_at(doc, obj, prop, heads)?;
        let changed = self != &loaded;
        *self = loaded;
        Ok(automorph::PrimitiveChanged::new(changed))
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Item {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Item {
    #[inline]
    fn eq(&self, other: &Item) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (Item::WEntry(__self_0), Item::WEntry(__arg1_0)) => __self_0 == __arg1_0,
                (Item::WFolder(__self_0), Item::WFolder(__arg1_0)) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() },
            }
    }
}
#[automorph(rename_all = "camelCase")]
pub struct Entry {
    #[automorph(rename = "type")]
    pub item_type: String,
    pub name: String,
    pub created_at: f64,
    pub edited_at: f64,
    pub parent_id: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub note: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for Entry {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "item_type",
            "name",
            "created_at",
            "edited_at",
            "parent_id",
            "username",
            "password",
            "url",
            "note",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.item_type,
            &self.name,
            &self.created_at,
            &self.edited_at,
            &self.parent_id,
            &self.username,
            &self.password,
            &self.url,
            &&self.note,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Entry", names, values)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Entry {
    #[inline]
    fn clone(&self) -> Entry {
        Entry {
            item_type: ::core::clone::Clone::clone(&self.item_type),
            name: ::core::clone::Clone::clone(&self.name),
            created_at: ::core::clone::Clone::clone(&self.created_at),
            edited_at: ::core::clone::Clone::clone(&self.edited_at),
            parent_id: ::core::clone::Clone::clone(&self.parent_id),
            username: ::core::clone::Clone::clone(&self.username),
            password: ::core::clone::Clone::clone(&self.password),
            url: ::core::clone::Clone::clone(&self.url),
            note: ::core::clone::Clone::clone(&self.note),
        }
    }
}
#[doc = r" Change report for #ident."]
#[doc = r" Each field contains `None` if unchanged, or `Some(nested_changes)` if changed."]
pub struct EntryChanges {
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub item_type: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub name: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub created_at: Option<<f64 as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub edited_at: Option<<f64 as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub parent_id: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub username: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub password: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub url: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub note: Option<<String as automorph::Automorph>::Changes>,
}
#[automatically_derived]
impl ::core::fmt::Debug for EntryChanges {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "item_type",
            "name",
            "created_at",
            "edited_at",
            "parent_id",
            "username",
            "password",
            "url",
            "note",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.item_type,
            &self.name,
            &self.created_at,
            &self.edited_at,
            &self.parent_id,
            &self.username,
            &self.password,
            &self.url,
            &&self.note,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "EntryChanges", names, values)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for EntryChanges {
    #[inline]
    fn clone(&self) -> EntryChanges {
        EntryChanges {
            item_type: ::core::clone::Clone::clone(&self.item_type),
            name: ::core::clone::Clone::clone(&self.name),
            created_at: ::core::clone::Clone::clone(&self.created_at),
            edited_at: ::core::clone::Clone::clone(&self.edited_at),
            parent_id: ::core::clone::Clone::clone(&self.parent_id),
            username: ::core::clone::Clone::clone(&self.username),
            password: ::core::clone::Clone::clone(&self.password),
            url: ::core::clone::Clone::clone(&self.url),
            note: ::core::clone::Clone::clone(&self.note),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for EntryChanges {
    #[inline]
    fn default() -> EntryChanges {
        EntryChanges {
            item_type: ::core::default::Default::default(),
            name: ::core::default::Default::default(),
            created_at: ::core::default::Default::default(),
            edited_at: ::core::default::Default::default(),
            parent_id: ::core::default::Default::default(),
            username: ::core::default::Default::default(),
            password: ::core::default::Default::default(),
            url: ::core::default::Default::default(),
            note: ::core::default::Default::default(),
        }
    }
}
impl automorph::ChangeReport for EntryChanges {
    fn any(&self) -> bool {
        false
            || self.item_type.is_some()
            || self.name.is_some()
            || self.created_at.is_some()
            || self.edited_at.is_some()
            || self.parent_id.is_some()
            || self.username.is_some()
            || self.password.is_some()
            || self.url.is_some()
            || self.note.is_some()
    }
    fn paths(&self) -> Vec<Vec<std::borrow::Cow<'static, str>>> {
        let mut paths = Vec::new();
        if let Some(ref field_changes) = self.item_type {
            paths.push(vec![std::borrow::Cow::Borrowed("type")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("type")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.name {
            paths.push(vec![std::borrow::Cow::Borrowed("name")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("name")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.created_at {
            paths.push(vec![std::borrow::Cow::Borrowed("createdAt")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("createdAt")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.edited_at {
            paths.push(vec![std::borrow::Cow::Borrowed("editedAt")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("editedAt")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.parent_id {
            paths.push(vec![std::borrow::Cow::Borrowed("parentId")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("parentId")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.username {
            paths.push(vec![std::borrow::Cow::Borrowed("username")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("username")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.password {
            paths.push(vec![std::borrow::Cow::Borrowed("password")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("password")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.url {
            paths.push(vec![std::borrow::Cow::Borrowed("url")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("url")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.note {
            paths.push(vec![std::borrow::Cow::Borrowed("note")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("note")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        paths
    }
    fn leaf_paths(&self) -> Vec<Vec<std::borrow::Cow<'static, str>>> {
        let mut paths = Vec::new();
        if let Some(ref field_changes) = self.item_type {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("type")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("type")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.name {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("name")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("name")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.created_at {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("createdAt")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("createdAt")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.edited_at {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("editedAt")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("editedAt")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.parent_id {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("parentId")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("parentId")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.username {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("username")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("username")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.password {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("password")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("password")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.url {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("url")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("url")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.note {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("note")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("note")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        paths
    }
}
#[doc = r" Cursor for tracking #ident changes efficiently."]
pub struct EntryCursor {
    #[doc = r" Cursor for this field."]
    pub item_type: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub name: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub created_at: <f64 as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub edited_at: <f64 as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub parent_id: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub username: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub password: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub url: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub note: <String as automorph::Automorph>::Cursor,
}
#[automatically_derived]
impl ::core::fmt::Debug for EntryCursor {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "item_type",
            "name",
            "created_at",
            "edited_at",
            "parent_id",
            "username",
            "password",
            "url",
            "note",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.item_type,
            &self.name,
            &self.created_at,
            &self.edited_at,
            &self.parent_id,
            &self.username,
            &self.password,
            &self.url,
            &&self.note,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "EntryCursor", names, values)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for EntryCursor {
    #[inline]
    fn clone(&self) -> EntryCursor {
        EntryCursor {
            item_type: ::core::clone::Clone::clone(&self.item_type),
            name: ::core::clone::Clone::clone(&self.name),
            created_at: ::core::clone::Clone::clone(&self.created_at),
            edited_at: ::core::clone::Clone::clone(&self.edited_at),
            parent_id: ::core::clone::Clone::clone(&self.parent_id),
            username: ::core::clone::Clone::clone(&self.username),
            password: ::core::clone::Clone::clone(&self.password),
            url: ::core::clone::Clone::clone(&self.url),
            note: ::core::clone::Clone::clone(&self.note),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for EntryCursor {
    #[inline]
    fn default() -> EntryCursor {
        EntryCursor {
            item_type: ::core::default::Default::default(),
            name: ::core::default::Default::default(),
            created_at: ::core::default::Default::default(),
            edited_at: ::core::default::Default::default(),
            parent_id: ::core::default::Default::default(),
            username: ::core::default::Default::default(),
            password: ::core::default::Default::default(),
            url: ::core::default::Default::default(),
            note: ::core::default::Default::default(),
        }
    }
}
impl automorph::FieldCursor for EntryCursor {
    type Changes = EntryChanges;
    fn diff<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: &automerge::ObjId,
    ) -> automorph::Result<Self::Changes> {
        let mut changes = EntryChanges::default();
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "type")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.item_type, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.item_type = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "name")? {
                let field_changes = automorph::FieldCursor::diff(&self.name, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.name = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "createdAt")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.created_at, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.created_at = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "editedAt")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.edited_at, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.edited_at = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "parentId")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.parent_id, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.parent_id = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "username")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.username, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.username = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "password")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.password, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.password = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "url")? {
                let field_changes = automorph::FieldCursor::diff(&self.url, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.url = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "note")? {
                let field_changes = automorph::FieldCursor::diff(&self.note, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.note = Some(field_changes);
                }
            }
        }
        Ok(changes)
    }
    fn refresh<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: &automerge::ObjId,
    ) -> automorph::Result<()> {
        if let Some((_, field_obj_id)) = doc.get(obj, "type")? {
            automorph::FieldCursor::refresh(&mut self.item_type, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "name")? {
            automorph::FieldCursor::refresh(&mut self.name, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "createdAt")? {
            automorph::FieldCursor::refresh(&mut self.created_at, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "editedAt")? {
            automorph::FieldCursor::refresh(&mut self.edited_at, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "parentId")? {
            automorph::FieldCursor::refresh(&mut self.parent_id, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "username")? {
            automorph::FieldCursor::refresh(&mut self.username, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "password")? {
            automorph::FieldCursor::refresh(&mut self.password, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "url")? {
            automorph::FieldCursor::refresh(&mut self.url, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "note")? {
            automorph::FieldCursor::refresh(&mut self.note, doc, &field_obj_id)?;
        }
        Ok(())
    }
}
impl automorph::Automorph for Entry {
    type Changes = EntryChanges;
    type Cursor = EntryCursor;
    fn save<D: automerge::transaction::Transactable + automerge::ReadDoc>(
        &self,
        doc: &mut D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<()> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        let cur = doc.get(obj, prop.clone())?;
        let map_id = match cur {
            Some((automerge::Value::Object(automerge::ObjType::Map), id)) => id,
            _ => doc.put_object(obj, prop, automerge::ObjType::Map)?,
        };
        automorph::Automorph::save(&self.item_type, doc, &map_id, "type")?;
        automorph::Automorph::save(&self.name, doc, &map_id, "name")?;
        automorph::Automorph::save(&self.created_at, doc, &map_id, "createdAt")?;
        automorph::Automorph::save(&self.edited_at, doc, &map_id, "editedAt")?;
        automorph::Automorph::save(&self.parent_id, doc, &map_id, "parentId")?;
        automorph::Automorph::save(&self.username, doc, &map_id, "username")?;
        automorph::Automorph::save(&self.password, doc, &map_id, "password")?;
        automorph::Automorph::save(&self.url, doc, &map_id, "url")?;
        automorph::Automorph::save(&self.note, doc, &map_id, "note")?;
        Ok(())
    }
    fn load<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get(obj, prop)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => Ok(Self {
                item_type: automorph::Automorph::load(doc, &map_id, "type")
                    .map_err(|e| e.with_field("type"))?,
                name: automorph::Automorph::load(doc, &map_id, "name")
                    .map_err(|e| e.with_field("name"))?,
                created_at: automorph::Automorph::load(doc, &map_id, "createdAt")
                    .map_err(|e| e.with_field("createdAt"))?,
                edited_at: automorph::Automorph::load(doc, &map_id, "editedAt")
                    .map_err(|e| e.with_field("editedAt"))?,
                parent_id: automorph::Automorph::load(doc, &map_id, "parentId")
                    .map_err(|e| e.with_field("parentId"))?,
                username: automorph::Automorph::load(doc, &map_id, "username")
                    .map_err(|e| e.with_field("username"))?,
                password: automorph::Automorph::load(doc, &map_id, "password")
                    .map_err(|e| e.with_field("password"))?,
                url: automorph::Automorph::load(doc, &map_id, "url")
                    .map_err(|e| e.with_field("url"))?,
                note: automorph::Automorph::load(doc, &map_id, "note")
                    .map_err(|e| e.with_field("note"))?,
            }),
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Entry",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn load_at<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get_at(obj, prop, heads)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => Ok(Self {
                item_type: automorph::Automorph::load_at(doc, &map_id, "type", heads)
                    .map_err(|e| e.with_field("type"))?,
                name: automorph::Automorph::load_at(doc, &map_id, "name", heads)
                    .map_err(|e| e.with_field("name"))?,
                created_at: automorph::Automorph::load_at(doc, &map_id, "createdAt", heads)
                    .map_err(|e| e.with_field("createdAt"))?,
                edited_at: automorph::Automorph::load_at(doc, &map_id, "editedAt", heads)
                    .map_err(|e| e.with_field("editedAt"))?,
                parent_id: automorph::Automorph::load_at(doc, &map_id, "parentId", heads)
                    .map_err(|e| e.with_field("parentId"))?,
                username: automorph::Automorph::load_at(doc, &map_id, "username", heads)
                    .map_err(|e| e.with_field("username"))?,
                password: automorph::Automorph::load_at(doc, &map_id, "password", heads)
                    .map_err(|e| e.with_field("password"))?,
                url: automorph::Automorph::load_at(doc, &map_id, "url", heads)
                    .map_err(|e| e.with_field("url"))?,
                note: automorph::Automorph::load_at(doc, &map_id, "note", heads)
                    .map_err(|e| e.with_field("note"))?,
            }),
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Entry",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn diff<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get(obj, prop)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = EntryChanges::default();
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.item_type, doc, &map_id, "type")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.item_type = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.name, doc, &map_id, "name")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.name = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.created_at, doc, &map_id, "createdAt")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.created_at = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.edited_at, doc, &map_id, "editedAt")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.edited_at = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.parent_id, doc, &map_id, "parentId")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.parent_id = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.username, doc, &map_id, "username")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.username = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.password, doc, &map_id, "password")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.password = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff(&self.url, doc, &map_id, "url")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.url = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.note, doc, &map_id, "note")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.note = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Entry",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn diff_at<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get_at(obj, prop, heads)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = EntryChanges::default();
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.item_type,
                        doc,
                        &map_id,
                        "type",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.item_type = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff_at(&self.name, doc, &map_id, "name", heads)?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.name = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.created_at,
                        doc,
                        &map_id,
                        "createdAt",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.created_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.edited_at,
                        doc,
                        &map_id,
                        "editedAt",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.edited_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.parent_id,
                        doc,
                        &map_id,
                        "parentId",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.parent_id = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.username,
                        doc,
                        &map_id,
                        "username",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.username = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.password,
                        doc,
                        &map_id,
                        "password",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.password = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff_at(&self.url, doc, &map_id, "url", heads)?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.url = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff_at(&self.note, doc, &map_id, "note", heads)?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.note = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Entry",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn update<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get(obj, prop)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = EntryChanges::default();
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.item_type, doc, &map_id, "type")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.item_type = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.name, doc, &map_id, "name")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.name = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update(
                        &mut self.created_at,
                        doc,
                        &map_id,
                        "createdAt",
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.created_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update(
                        &mut self.edited_at,
                        doc,
                        &map_id,
                        "editedAt",
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.edited_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update(
                        &mut self.parent_id,
                        doc,
                        &map_id,
                        "parentId",
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.parent_id = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.username, doc, &map_id, "username")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.username = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.password, doc, &map_id, "password")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.password = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.url, doc, &map_id, "url")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.url = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.note, doc, &map_id, "note")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.note = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Entry",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn update_at<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get_at(obj, prop, heads)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = EntryChanges::default();
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.item_type,
                        doc,
                        &map_id,
                        "type",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.item_type = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.name,
                        doc,
                        &map_id,
                        "name",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.name = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.created_at,
                        doc,
                        &map_id,
                        "createdAt",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.created_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.edited_at,
                        doc,
                        &map_id,
                        "editedAt",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.edited_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.parent_id,
                        doc,
                        &map_id,
                        "parentId",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.parent_id = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.username,
                        doc,
                        &map_id,
                        "username",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.username = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.password,
                        doc,
                        &map_id,
                        "password",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.password = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::update_at(&mut self.url, doc, &map_id, "url", heads)?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.url = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.note,
                        doc,
                        &map_id,
                        "note",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.note = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Entry",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn save_flat<D: automerge::transaction::Transactable + automerge::ReadDoc>(
        &self,
        doc: &mut D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<()> {
        let map_id = obj.as_ref();
        automorph::Automorph::save(&self.item_type, doc, &map_id, "type")?;
        automorph::Automorph::save(&self.name, doc, &map_id, "name")?;
        automorph::Automorph::save(&self.created_at, doc, &map_id, "createdAt")?;
        automorph::Automorph::save(&self.edited_at, doc, &map_id, "editedAt")?;
        automorph::Automorph::save(&self.parent_id, doc, &map_id, "parentId")?;
        automorph::Automorph::save(&self.username, doc, &map_id, "username")?;
        automorph::Automorph::save(&self.password, doc, &map_id, "password")?;
        automorph::Automorph::save(&self.url, doc, &map_id, "url")?;
        automorph::Automorph::save(&self.note, doc, &map_id, "note")?;
        Ok(())
    }
    fn load_flat<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<Self> {
        let map_id = obj.as_ref();
        Ok(Self {
            item_type: automorph::Automorph::load(doc, &map_id, "type")
                .map_err(|e| e.with_field("type"))?,
            name: automorph::Automorph::load(doc, &map_id, "name")
                .map_err(|e| e.with_field("name"))?,
            created_at: automorph::Automorph::load(doc, &map_id, "createdAt")
                .map_err(|e| e.with_field("createdAt"))?,
            edited_at: automorph::Automorph::load(doc, &map_id, "editedAt")
                .map_err(|e| e.with_field("editedAt"))?,
            parent_id: automorph::Automorph::load(doc, &map_id, "parentId")
                .map_err(|e| e.with_field("parentId"))?,
            username: automorph::Automorph::load(doc, &map_id, "username")
                .map_err(|e| e.with_field("username"))?,
            password: automorph::Automorph::load(doc, &map_id, "password")
                .map_err(|e| e.with_field("password"))?,
            url: automorph::Automorph::load(doc, &map_id, "url")
                .map_err(|e| e.with_field("url"))?,
            note: automorph::Automorph::load(doc, &map_id, "note")
                .map_err(|e| e.with_field("note"))?,
        })
    }
    fn load_flat_at<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self> {
        let map_id = obj.as_ref();
        Ok(Self {
            item_type: automorph::Automorph::load_at(doc, &map_id, "type", heads)
                .map_err(|e| e.with_field("type"))?,
            name: automorph::Automorph::load_at(doc, &map_id, "name", heads)
                .map_err(|e| e.with_field("name"))?,
            created_at: automorph::Automorph::load_at(doc, &map_id, "createdAt", heads)
                .map_err(|e| e.with_field("createdAt"))?,
            edited_at: automorph::Automorph::load_at(doc, &map_id, "editedAt", heads)
                .map_err(|e| e.with_field("editedAt"))?,
            parent_id: automorph::Automorph::load_at(doc, &map_id, "parentId", heads)
                .map_err(|e| e.with_field("parentId"))?,
            username: automorph::Automorph::load_at(doc, &map_id, "username", heads)
                .map_err(|e| e.with_field("username"))?,
            password: automorph::Automorph::load_at(doc, &map_id, "password", heads)
                .map_err(|e| e.with_field("password"))?,
            url: automorph::Automorph::load_at(doc, &map_id, "url", heads)
                .map_err(|e| e.with_field("url"))?,
            note: automorph::Automorph::load_at(doc, &map_id, "note", heads)
                .map_err(|e| e.with_field("note"))?,
        })
    }
    fn diff_flat<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = EntryChanges::default();
        {
            let field_changes = automorph::Automorph::diff(&self.item_type, doc, &map_id, "type")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.item_type = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::diff(&self.name, doc, &map_id, "name")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.name = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff(&self.created_at, doc, &map_id, "createdAt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.created_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff(&self.edited_at, doc, &map_id, "editedAt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.edited_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff(&self.parent_id, doc, &map_id, "parentId")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.parent_id = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff(&self.username, doc, &map_id, "username")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.username = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff(&self.password, doc, &map_id, "password")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.password = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::diff(&self.url, doc, &map_id, "url")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.url = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::diff(&self.note, doc, &map_id, "note")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.note = Some(field_changes);
            }
        }
        Ok(changes)
    }
    fn diff_flat_at<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = EntryChanges::default();
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.item_type, doc, &map_id, "type", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.item_type = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.name, doc, &map_id, "name", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.name = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.created_at, doc, &map_id, "createdAt", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.created_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.edited_at, doc, &map_id, "editedAt", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.edited_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.parent_id, doc, &map_id, "parentId", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.parent_id = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.username, doc, &map_id, "username", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.username = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.password, doc, &map_id, "password", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.password = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.url, doc, &map_id, "url", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.url = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.note, doc, &map_id, "note", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.note = Some(field_changes);
            }
        }
        Ok(changes)
    }
    fn update_flat<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = EntryChanges::default();
        {
            let field_changes =
                automorph::Automorph::update(&mut self.item_type, doc, &map_id, "type")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.item_type = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update(&mut self.name, doc, &map_id, "name")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.name = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.created_at, doc, &map_id, "createdAt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.created_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.edited_at, doc, &map_id, "editedAt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.edited_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.parent_id, doc, &map_id, "parentId")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.parent_id = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.username, doc, &map_id, "username")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.username = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.password, doc, &map_id, "password")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.password = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update(&mut self.url, doc, &map_id, "url")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.url = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update(&mut self.note, doc, &map_id, "note")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.note = Some(field_changes);
            }
        }
        Ok(changes)
    }
    fn update_flat_at<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = EntryChanges::default();
        {
            let field_changes =
                automorph::Automorph::update_at(&mut self.item_type, doc, &map_id, "type", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.item_type = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update_at(&mut self.name, doc, &map_id, "name", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.name = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update_at(
                &mut self.created_at,
                doc,
                &map_id,
                "createdAt",
                heads,
            )?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.created_at = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update_at(
                &mut self.edited_at,
                doc,
                &map_id,
                "editedAt",
                heads,
            )?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.edited_at = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update_at(
                &mut self.parent_id,
                doc,
                &map_id,
                "parentId",
                heads,
            )?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.parent_id = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update_at(
                &mut self.username,
                doc,
                &map_id,
                "username",
                heads,
            )?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.username = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update_at(
                &mut self.password,
                doc,
                &map_id,
                "password",
                heads,
            )?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.password = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update_at(&mut self.url, doc, &map_id, "url", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.url = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update_at(&mut self.note, doc, &map_id, "note", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.note = Some(field_changes);
            }
        }
        Ok(changes)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Entry {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Entry {
    #[inline]
    fn eq(&self, other: &Entry) -> bool {
        self.created_at == other.created_at
            && self.edited_at == other.edited_at
            && self.item_type == other.item_type
            && self.name == other.name
            && self.parent_id == other.parent_id
            && self.username == other.username
            && self.password == other.password
            && self.url == other.url
            && self.note == other.note
    }
}
#[automorph(rename_all = "camelCase")]
pub struct Folder {
    #[automorph(rename = "type")]
    pub item_type: String,
    pub name: String,
    pub created_at: f64,
    pub edited_at: f64,
    pub parent_id: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for Folder {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "Folder",
            "item_type",
            &self.item_type,
            "name",
            &self.name,
            "created_at",
            &self.created_at,
            "edited_at",
            &self.edited_at,
            "parent_id",
            &&self.parent_id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Folder {
    #[inline]
    fn clone(&self) -> Folder {
        Folder {
            item_type: ::core::clone::Clone::clone(&self.item_type),
            name: ::core::clone::Clone::clone(&self.name),
            created_at: ::core::clone::Clone::clone(&self.created_at),
            edited_at: ::core::clone::Clone::clone(&self.edited_at),
            parent_id: ::core::clone::Clone::clone(&self.parent_id),
        }
    }
}
#[doc = r" Change report for #ident."]
#[doc = r" Each field contains `None` if unchanged, or `Some(nested_changes)` if changed."]
pub struct FolderChanges {
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub item_type: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub name: Option<<String as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub created_at: Option<<f64 as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub edited_at: Option<<f64 as automorph::Automorph>::Changes>,
    #[doc = r" Changes for this field. `None` means unchanged, `Some` contains nested changes."]
    pub parent_id: Option<<String as automorph::Automorph>::Changes>,
}
#[automatically_derived]
impl ::core::fmt::Debug for FolderChanges {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "FolderChanges",
            "item_type",
            &self.item_type,
            "name",
            &self.name,
            "created_at",
            &self.created_at,
            "edited_at",
            &self.edited_at,
            "parent_id",
            &&self.parent_id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FolderChanges {
    #[inline]
    fn clone(&self) -> FolderChanges {
        FolderChanges {
            item_type: ::core::clone::Clone::clone(&self.item_type),
            name: ::core::clone::Clone::clone(&self.name),
            created_at: ::core::clone::Clone::clone(&self.created_at),
            edited_at: ::core::clone::Clone::clone(&self.edited_at),
            parent_id: ::core::clone::Clone::clone(&self.parent_id),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for FolderChanges {
    #[inline]
    fn default() -> FolderChanges {
        FolderChanges {
            item_type: ::core::default::Default::default(),
            name: ::core::default::Default::default(),
            created_at: ::core::default::Default::default(),
            edited_at: ::core::default::Default::default(),
            parent_id: ::core::default::Default::default(),
        }
    }
}
impl automorph::ChangeReport for FolderChanges {
    fn any(&self) -> bool {
        false
            || self.item_type.is_some()
            || self.name.is_some()
            || self.created_at.is_some()
            || self.edited_at.is_some()
            || self.parent_id.is_some()
    }
    fn paths(&self) -> Vec<Vec<std::borrow::Cow<'static, str>>> {
        let mut paths = Vec::new();
        if let Some(ref field_changes) = self.item_type {
            paths.push(vec![std::borrow::Cow::Borrowed("type")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("type")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.name {
            paths.push(vec![std::borrow::Cow::Borrowed("name")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("name")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.created_at {
            paths.push(vec![std::borrow::Cow::Borrowed("createdAt")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("createdAt")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.edited_at {
            paths.push(vec![std::borrow::Cow::Borrowed("editedAt")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("editedAt")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        if let Some(ref field_changes) = self.parent_id {
            paths.push(vec![std::borrow::Cow::Borrowed("parentId")]);
            for sub_path in automorph::ChangeReport::paths(field_changes) {
                let mut path: Vec<std::borrow::Cow<'static, str>> =
                    vec![std::borrow::Cow::Borrowed("parentId")];
                path.extend(sub_path);
                paths.push(path);
            }
        }
        paths
    }
    fn leaf_paths(&self) -> Vec<Vec<std::borrow::Cow<'static, str>>> {
        let mut paths = Vec::new();
        if let Some(ref field_changes) = self.item_type {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("type")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("type")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.name {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("name")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("name")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.created_at {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("createdAt")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("createdAt")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.edited_at {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("editedAt")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("editedAt")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        if let Some(ref field_changes) = self.parent_id {
            let sub_paths = automorph::ChangeReport::leaf_paths(field_changes);
            if sub_paths.is_empty() {
                paths.push(vec![std::borrow::Cow::Borrowed("parentId")]);
            } else {
                for sub_path in sub_paths {
                    let mut path: Vec<std::borrow::Cow<'static, str>> =
                        vec![std::borrow::Cow::Borrowed("parentId")];
                    path.extend(sub_path);
                    paths.push(path);
                }
            }
        }
        paths
    }
}
#[doc = r" Cursor for tracking #ident changes efficiently."]
pub struct FolderCursor {
    #[doc = r" Cursor for this field."]
    pub item_type: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub name: <String as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub created_at: <f64 as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub edited_at: <f64 as automorph::Automorph>::Cursor,
    #[doc = r" Cursor for this field."]
    pub parent_id: <String as automorph::Automorph>::Cursor,
}
#[automatically_derived]
impl ::core::fmt::Debug for FolderCursor {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "FolderCursor",
            "item_type",
            &self.item_type,
            "name",
            &self.name,
            "created_at",
            &self.created_at,
            "edited_at",
            &self.edited_at,
            "parent_id",
            &&self.parent_id,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for FolderCursor {
    #[inline]
    fn clone(&self) -> FolderCursor {
        FolderCursor {
            item_type: ::core::clone::Clone::clone(&self.item_type),
            name: ::core::clone::Clone::clone(&self.name),
            created_at: ::core::clone::Clone::clone(&self.created_at),
            edited_at: ::core::clone::Clone::clone(&self.edited_at),
            parent_id: ::core::clone::Clone::clone(&self.parent_id),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for FolderCursor {
    #[inline]
    fn default() -> FolderCursor {
        FolderCursor {
            item_type: ::core::default::Default::default(),
            name: ::core::default::Default::default(),
            created_at: ::core::default::Default::default(),
            edited_at: ::core::default::Default::default(),
            parent_id: ::core::default::Default::default(),
        }
    }
}
impl automorph::FieldCursor for FolderCursor {
    type Changes = FolderChanges;
    fn diff<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: &automerge::ObjId,
    ) -> automorph::Result<Self::Changes> {
        let mut changes = FolderChanges::default();
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "type")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.item_type, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.item_type = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "name")? {
                let field_changes = automorph::FieldCursor::diff(&self.name, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.name = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "createdAt")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.created_at, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.created_at = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "editedAt")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.edited_at, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.edited_at = Some(field_changes);
                }
            }
        }
        {
            if let Some((_, field_obj_id)) = doc.get(obj, "parentId")? {
                let field_changes =
                    automorph::FieldCursor::diff(&self.parent_id, doc, &field_obj_id)?;
                if automorph::ChangeReport::any(&field_changes) {
                    changes.parent_id = Some(field_changes);
                }
            }
        }
        Ok(changes)
    }
    fn refresh<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: &automerge::ObjId,
    ) -> automorph::Result<()> {
        if let Some((_, field_obj_id)) = doc.get(obj, "type")? {
            automorph::FieldCursor::refresh(&mut self.item_type, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "name")? {
            automorph::FieldCursor::refresh(&mut self.name, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "createdAt")? {
            automorph::FieldCursor::refresh(&mut self.created_at, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "editedAt")? {
            automorph::FieldCursor::refresh(&mut self.edited_at, doc, &field_obj_id)?;
        }
        if let Some((_, field_obj_id)) = doc.get(obj, "parentId")? {
            automorph::FieldCursor::refresh(&mut self.parent_id, doc, &field_obj_id)?;
        }
        Ok(())
    }
}
impl automorph::Automorph for Folder {
    type Changes = FolderChanges;
    type Cursor = FolderCursor;
    fn save<D: automerge::transaction::Transactable + automerge::ReadDoc>(
        &self,
        doc: &mut D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<()> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        let cur = doc.get(obj, prop.clone())?;
        let map_id = match cur {
            Some((automerge::Value::Object(automerge::ObjType::Map), id)) => id,
            _ => doc.put_object(obj, prop, automerge::ObjType::Map)?,
        };
        automorph::Automorph::save(&self.item_type, doc, &map_id, "type")?;
        automorph::Automorph::save(&self.name, doc, &map_id, "name")?;
        automorph::Automorph::save(&self.created_at, doc, &map_id, "createdAt")?;
        automorph::Automorph::save(&self.edited_at, doc, &map_id, "editedAt")?;
        automorph::Automorph::save(&self.parent_id, doc, &map_id, "parentId")?;
        Ok(())
    }
    fn load<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get(obj, prop)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => Ok(Self {
                item_type: automorph::Automorph::load(doc, &map_id, "type")
                    .map_err(|e| e.with_field("type"))?,
                name: automorph::Automorph::load(doc, &map_id, "name")
                    .map_err(|e| e.with_field("name"))?,
                created_at: automorph::Automorph::load(doc, &map_id, "createdAt")
                    .map_err(|e| e.with_field("createdAt"))?,
                edited_at: automorph::Automorph::load(doc, &map_id, "editedAt")
                    .map_err(|e| e.with_field("editedAt"))?,
                parent_id: automorph::Automorph::load(doc, &map_id, "parentId")
                    .map_err(|e| e.with_field("parentId"))?,
            }),
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Folder",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn load_at<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get_at(obj, prop, heads)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => Ok(Self {
                item_type: automorph::Automorph::load_at(doc, &map_id, "type", heads)
                    .map_err(|e| e.with_field("type"))?,
                name: automorph::Automorph::load_at(doc, &map_id, "name", heads)
                    .map_err(|e| e.with_field("name"))?,
                created_at: automorph::Automorph::load_at(doc, &map_id, "createdAt", heads)
                    .map_err(|e| e.with_field("createdAt"))?,
                edited_at: automorph::Automorph::load_at(doc, &map_id, "editedAt", heads)
                    .map_err(|e| e.with_field("editedAt"))?,
                parent_id: automorph::Automorph::load_at(doc, &map_id, "parentId", heads)
                    .map_err(|e| e.with_field("parentId"))?,
            }),
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Folder",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn diff<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get(obj, prop)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = FolderChanges::default();
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.item_type, doc, &map_id, "type")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.item_type = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.name, doc, &map_id, "name")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.name = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.created_at, doc, &map_id, "createdAt")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.created_at = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.edited_at, doc, &map_id, "editedAt")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.edited_at = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff(&self.parent_id, doc, &map_id, "parentId")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.parent_id = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Folder",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn diff_at<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get_at(obj, prop, heads)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = FolderChanges::default();
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.item_type,
                        doc,
                        &map_id,
                        "type",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.item_type = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::diff_at(&self.name, doc, &map_id, "name", heads)?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.name = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.created_at,
                        doc,
                        &map_id,
                        "createdAt",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.created_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.edited_at,
                        doc,
                        &map_id,
                        "editedAt",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.edited_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::diff_at(
                        &self.parent_id,
                        doc,
                        &map_id,
                        "parentId",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.parent_id = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Folder",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn update<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get(obj, prop)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = FolderChanges::default();
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.item_type, doc, &map_id, "type")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.item_type = Some(field_changes);
                    }
                }
                {
                    let field_changes =
                        automorph::Automorph::update(&mut self.name, doc, &map_id, "name")?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.name = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update(
                        &mut self.created_at,
                        doc,
                        &map_id,
                        "createdAt",
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.created_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update(
                        &mut self.edited_at,
                        doc,
                        &map_id,
                        "editedAt",
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.edited_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update(
                        &mut self.parent_id,
                        doc,
                        &map_id,
                        "parentId",
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.parent_id = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Folder",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn update_at<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        prop: impl Into<automerge::Prop>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let prop: automerge::Prop = prop.into();
        let obj = obj.as_ref();
        match doc.get_at(obj, prop, heads)? {
            Some((automerge::Value::Object(automerge::ObjType::Map), map_id)) => {
                let mut changes = FolderChanges::default();
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.item_type,
                        doc,
                        &map_id,
                        "type",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.item_type = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.name,
                        doc,
                        &map_id,
                        "name",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.name = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.created_at,
                        doc,
                        &map_id,
                        "createdAt",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.created_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.edited_at,
                        doc,
                        &map_id,
                        "editedAt",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.edited_at = Some(field_changes);
                    }
                }
                {
                    let field_changes = automorph::Automorph::update_at(
                        &mut self.parent_id,
                        doc,
                        &map_id,
                        "parentId",
                        heads,
                    )?;
                    if automorph::ChangeReport::any(&field_changes) {
                        changes.parent_id = Some(field_changes);
                    }
                }
                Ok(changes)
            }
            Some((v, _)) => Err(automorph::Error::type_mismatch(
                "Folder",
                Some(::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0:?}", v))
                })),
            )),
            None => Err(automorph::Error::missing_value()),
        }
    }
    fn save_flat<D: automerge::transaction::Transactable + automerge::ReadDoc>(
        &self,
        doc: &mut D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<()> {
        let map_id = obj.as_ref();
        automorph::Automorph::save(&self.item_type, doc, &map_id, "type")?;
        automorph::Automorph::save(&self.name, doc, &map_id, "name")?;
        automorph::Automorph::save(&self.created_at, doc, &map_id, "createdAt")?;
        automorph::Automorph::save(&self.edited_at, doc, &map_id, "editedAt")?;
        automorph::Automorph::save(&self.parent_id, doc, &map_id, "parentId")?;
        Ok(())
    }
    fn load_flat<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<Self> {
        let map_id = obj.as_ref();
        Ok(Self {
            item_type: automorph::Automorph::load(doc, &map_id, "type")
                .map_err(|e| e.with_field("type"))?,
            name: automorph::Automorph::load(doc, &map_id, "name")
                .map_err(|e| e.with_field("name"))?,
            created_at: automorph::Automorph::load(doc, &map_id, "createdAt")
                .map_err(|e| e.with_field("createdAt"))?,
            edited_at: automorph::Automorph::load(doc, &map_id, "editedAt")
                .map_err(|e| e.with_field("editedAt"))?,
            parent_id: automorph::Automorph::load(doc, &map_id, "parentId")
                .map_err(|e| e.with_field("parentId"))?,
        })
    }
    fn load_flat_at<D: automerge::ReadDoc>(
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self> {
        let map_id = obj.as_ref();
        Ok(Self {
            item_type: automorph::Automorph::load_at(doc, &map_id, "type", heads)
                .map_err(|e| e.with_field("type"))?,
            name: automorph::Automorph::load_at(doc, &map_id, "name", heads)
                .map_err(|e| e.with_field("name"))?,
            created_at: automorph::Automorph::load_at(doc, &map_id, "createdAt", heads)
                .map_err(|e| e.with_field("createdAt"))?,
            edited_at: automorph::Automorph::load_at(doc, &map_id, "editedAt", heads)
                .map_err(|e| e.with_field("editedAt"))?,
            parent_id: automorph::Automorph::load_at(doc, &map_id, "parentId", heads)
                .map_err(|e| e.with_field("parentId"))?,
        })
    }
    fn diff_flat<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = FolderChanges::default();
        {
            let field_changes = automorph::Automorph::diff(&self.item_type, doc, &map_id, "type")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.item_type = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::diff(&self.name, doc, &map_id, "name")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.name = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff(&self.created_at, doc, &map_id, "createdAt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.created_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff(&self.edited_at, doc, &map_id, "editedAt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.edited_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff(&self.parent_id, doc, &map_id, "parentId")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.parent_id = Some(field_changes);
            }
        }
        Ok(changes)
    }
    fn diff_flat_at<D: automerge::ReadDoc>(
        &self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = FolderChanges::default();
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.item_type, doc, &map_id, "type", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.item_type = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.name, doc, &map_id, "name", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.name = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.created_at, doc, &map_id, "createdAt", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.created_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.edited_at, doc, &map_id, "editedAt", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.edited_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::diff_at(&self.parent_id, doc, &map_id, "parentId", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.parent_id = Some(field_changes);
            }
        }
        Ok(changes)
    }
    fn update_flat<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = FolderChanges::default();
        {
            let field_changes =
                automorph::Automorph::update(&mut self.item_type, doc, &map_id, "type")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.item_type = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update(&mut self.name, doc, &map_id, "name")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.name = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.created_at, doc, &map_id, "createdAt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.created_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.edited_at, doc, &map_id, "editedAt")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.edited_at = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update(&mut self.parent_id, doc, &map_id, "parentId")?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.parent_id = Some(field_changes);
            }
        }
        Ok(changes)
    }
    fn update_flat_at<D: automerge::ReadDoc>(
        &mut self,
        doc: &D,
        obj: impl AsRef<automerge::ObjId>,
        heads: &[automerge::ChangeHash],
    ) -> automorph::Result<Self::Changes> {
        let map_id = obj.as_ref();
        let mut changes = FolderChanges::default();
        {
            let field_changes =
                automorph::Automorph::update_at(&mut self.item_type, doc, &map_id, "type", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.item_type = Some(field_changes);
            }
        }
        {
            let field_changes =
                automorph::Automorph::update_at(&mut self.name, doc, &map_id, "name", heads)?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.name = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update_at(
                &mut self.created_at,
                doc,
                &map_id,
                "createdAt",
                heads,
            )?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.created_at = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update_at(
                &mut self.edited_at,
                doc,
                &map_id,
                "editedAt",
                heads,
            )?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.edited_at = Some(field_changes);
            }
        }
        {
            let field_changes = automorph::Automorph::update_at(
                &mut self.parent_id,
                doc,
                &map_id,
                "parentId",
                heads,
            )?;
            if automorph::ChangeReport::any(&field_changes) {
                changes.parent_id = Some(field_changes);
            }
        }
        Ok(changes)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Folder {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Folder {
    #[inline]
    fn eq(&self, other: &Folder) -> bool {
        self.created_at == other.created_at
            && self.edited_at == other.edited_at
            && self.item_type == other.item_type
            && self.name == other.name
            && self.parent_id == other.parent_id
    }
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

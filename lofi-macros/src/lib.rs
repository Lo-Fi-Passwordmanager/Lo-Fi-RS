use proc_macro::TokenStream;
use syn::__private::quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, ItemEnum, Type, Variant};

struct UntaggedAttrs {
    hydrate: bool,
    reconcile: bool,
}

impl Parse for UntaggedAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut hydrate = false;
        let mut reconcile = false;

        while !input.is_empty() {
            let id: syn::Ident = input.parse()?;
            match id.to_string().as_str() {
                "Hydrate" => hydrate = true,
                "Reconcile" => reconcile = true,
                _ => return Err(syn::Error::new(id.span(), "Expected Hydrate or Reconcile")),
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(UntaggedAttrs { hydrate, reconcile })
    }
}

#[proc_macro_attribute]
pub fn samod_untagged(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let cloned_item = item.clone();

    let ItemEnum {
        attrs,
        vis,
        enum_token,
        ident,
        generics,
        brace_token,
        variants,
    } = parse_macro_input!(cloned_item as ItemEnum);

    let UntaggedAttrs { hydrate, reconcile } = parse_macro_input!(attr as UntaggedAttrs);

    let str_ident = ident.to_string();
    let vars: Vec<&Variant> = variants.iter().collect();

    let hydrate_vars: Vec<String> = vars
        .iter()
        .map(|v| {
            let variant = v.ident.to_string();
            let f = v.fields.iter().next().unwrap();
            let inner_variant = match &f.ty {
                Type::Path(path) => path.path.segments[0].ident.to_string(),
                _ => "".to_string(),
            };
            let lower_inner_variant = inner_variant.to_lowercase();
            format!("\"{lower_inner_variant}\" => Ok({str_ident}::{variant}({inner_variant}::hydrate_map(doc, obj)?)),")
        })
        .collect();

    let reconcile_vars: Vec<String> = vars
        .iter()
        .map(|v| {
            let variant = v.ident.to_string();
            format!("{str_ident}::{variant}(item) => item.reconcile(reconciler),")
        })
        .collect();

    if reconcile {
        <proc_macro::TokenStream as Extend<proc_macro::TokenTree>>::extend::<proc_macro::TokenStream>(&mut item, quote! {
            impl autosurgeon::Reconcile for #str_ident {
                type Key<'a> = ();

                fn reconcile<R: autosurgeon::Reconciler>(&self, reconciler: R) -> Result<(), R::Error> {
                    match self {
                        #(#reconcile_vars)*
                    }
                }
            }
        }.into());
    }

    if hydrate {
        <proc_macro::TokenStream as Extend<proc_macro::TokenTree>>::extend::<proc_macro::TokenStream>(&mut item, quote! {
            impl autosurgeon::Hydrate for #str_ident {
                fn hydrate_map<D: autosurgeon::ReadDoc>(doc: &D, obj: &automerge::ObjId) -> Result<Self, autosurgeon::HydrateError> {
                    let Some(obj_type) = doc.object_type(obj) else {
                        return Err(autosurgeon::HydrateError::unexpected(
                            "an item",
                            "a scalar value".to_string(),
                        ));
                    };

                    match obj_type {
                        automerge::ObjType::Map | automerge::ObjType::Table => {
                            let entries: std::collections::HashMap<String, automerge::iter::MapRangeItem> = doc
                                .map_range(obj.clone(), ..)
                                .map(move |item| {
                                    let key = item.key.as_ref();
                                    (key.to_string(), item)
                                })
                                .collect();

                            let t = entries.get("type").unwrap();

                            let val = autosurgeon::Text::hydrate(doc, obj, t.key.as_ref().into())?;

                            match val.as_str() {
                                #(#hydrate_vars)*
                                _ => Err(autosurgeon::HydrateError::unexpected(
                                    "an item",
                                    format!("a/an {}", val.as_str()).to_string(),
                                )),
                            }
                        }
                        automerge::ObjType::Text => Err(HydrateError::unexpected(
                            "an item",
                            "a text object".to_string(),
                        )),
                        automerge::ObjType::List => Err(HydrateError::unexpected(
                            "an item",
                            "a list object".to_string(),
                        )),
                    }
                }
            }
        }.into());
    }

    item
}

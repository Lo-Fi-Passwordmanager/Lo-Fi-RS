use proc_macro::TokenStream;
use syn::__private::quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, ItemEnum, Variant};

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
pub fn lofi_autosurgeon_untagged(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let cloned_item = item.clone();

    let ItemEnum {
        attrs: _,
        vis: _,
        enum_token: _,
        ident,
        generics: _,
        brace_token: _,
        variants,
    } = parse_macro_input!(cloned_item as ItemEnum);

    let UntaggedAttrs { hydrate, reconcile } = parse_macro_input!(attr as UntaggedAttrs);
    let vars: Vec<&Variant> = variants.iter().collect();

    let hydrate_vars: Vec<proc_macro2::TokenStream> = vars
        .iter()
        .map(|v| {
            let variant_ident = &v.ident;

            let f = v
                .fields
                .iter()
                .next()
                .expect("Variant must have at least one field");

            let inner_type = match &f.ty {
                syn::Type::Path(path) => &path.path.segments[0].ident,
                _ => panic!("Expected a type path for variant {}", variant_ident),
            };

            let pattern_str = inner_type.to_string().to_lowercase();

            quote! {
                #pattern_str => Ok(#ident::#variant_ident(#inner_type::hydrate_map(doc, obj)?)),
            }
        })
        .collect();

    let reconcile_vars: Vec<proc_macro2::TokenStream> = vars
        .iter()
        .map(|v| {
            let variant = &v.ident;
            quote! {
                #ident::#variant(item) => item.reconcile(reconciler),
            }
        })
        .collect();

    if reconcile {
        <proc_macro::TokenStream as Extend<proc_macro::TokenTree>>::extend::<proc_macro::TokenStream>(&mut item, quote! {
            impl autosurgeon::Reconcile for #ident {
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
            impl autosurgeon::Hydrate for #ident {
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
                        automerge::ObjType::Text => Err(autosurgeon::HydrateError::unexpected(
                            "an item",
                            "a text object".to_string(),
                        )),
                        automerge::ObjType::List => Err(autosurgeon::HydrateError::unexpected(
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

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
    let identifier_item = item.clone();
    let identifier_item2 = item.clone();

    if syn::parse::<ItemEnum>(identifier_item).is_err() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(identifier_item2),
            "Error: Attribute lofi_autosurgeon_untagged is only works with enums",
        )
        .to_compile_error()
        .into();
    }

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
            let variant = &v.ident;

            let f = v
                .fields
                .iter()
                .next()
                .expect("Variant must have at least one field");

            let inner_type = match &f.ty {
                syn::Type::Path(path) => &path.path.segments[0].ident,
                _ => panic!("Expected a type path for variant {}", variant),
            };

            quote! {
                let result = #inner_type::hydrate_map(doc, obj);
                if result.is_ok() {
                    return Ok(#ident::#variant(result?))
                }
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
            #[automatically_derived]
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
            #[automatically_derived]
            impl autosurgeon::Hydrate for #ident {
                fn hydrate_map<D: autosurgeon::ReadDoc>(doc: &D, obj: &automerge::ObjId) -> Result<Self, autosurgeon::HydrateError> {
                    let Some(obj_type) = doc.object_type(obj) else {
                        return Err(autosurgeon::HydrateError::unexpected(
                            "a value matching to any of the members of #ident",
                            "a scalar value".to_string(),
                        ));
                    };

                    match obj_type {
                        automerge::ObjType::Map | automerge::ObjType::Table => {
                            let entries: Vec<String> = doc
                                .map_range(obj.clone(), ..)
                                .map(|item| {
                                    item.key.to_string()
                                })
                                .collect();

                            #(#hydrate_vars)*

                            Err(autosurgeon::HydrateError::unexpected(
                                "a value matching to any of the members of #ident",
                                format!(
                                    "a structure with the following fields: {:?}",
                                    entries
                                ),
                            ))
                        }
                        automerge::ObjType::Text => Err(autosurgeon::HydrateError::unexpected(
                            "a value matching to any of the members of #ident",
                            "a text object".to_string(),
                        )),
                        automerge::ObjType::List => Err(autosurgeon::HydrateError::unexpected(
                            "a value matching to any of the members of #ident",
                            "a list object".to_string(),
                        )),
                    }
                }
            }
        }.into());
    }

    item
}

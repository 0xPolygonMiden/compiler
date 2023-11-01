use inflector::cases::kebabcase::to_kebab_case;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(PassInfo)]
pub fn derive_pass_info(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(item as DeriveInput);
    let derive_span = derive_input.span();
    let id = derive_input.ident.clone();
    let generics = derive_input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name = derive_input.ident.to_string();
    let pass_name = to_kebab_case(&name);
    let pass_name_lit = syn::Lit::Str(syn::LitStr::new(&pass_name, id.span()));

    let docs_ident = syn::Ident::new("docs", derive_span);
    let docs = derive_input
        .attrs
        .iter()
        .filter_map(|attr| match attr.meta {
            syn::Meta::NameValue(ref nv) => {
                if nv.path.get_ident()? == &docs_ident {
                    match nv.value {
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(ref lit),
                            ..
                        }) => Some(lit.value()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            syn::Meta::Path(_) | syn::Meta::List(_) => None,
        })
        .collect::<String>();
    let pass_help = syn::Lit::Str(syn::LitStr::new(&docs, derive_span));

    let quoted = quote! {
        impl #impl_generics PassInfo for #id #ty_generics #where_clause {
            const FLAG: &'static str = #pass_name_lit;
            const HELP: &'static str = #pass_help;
        }
    };

    proc_macro::TokenStream::from(quoted)
}

#[proc_macro_derive(AnalysisKey, attributes(analysis_key))]
pub fn derive_analysis_key(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(item as DeriveInput);
    let derive_span = derive_input.span();
    let id = derive_input.ident.clone();
    let generics = derive_input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let found = match &derive_input.data {
        syn::Data::Struct(ref data) => match &data.fields {
            syn::Fields::Named(ref fields) => {
                let mut found = None;
                for field in fields.named.iter() {
                    if field.attrs.iter().any(is_analysis_key_attr) {
                        if found.is_some() {
                            return syn::Error::new(
                                field.span(),
                                "duplicate #[analysis_key] field",
                            )
                            .into_compile_error()
                            .into();
                        }
                        found = Some((field.ident.as_ref().cloned().unwrap(), field.ty.clone()));
                    }
                }
                found
            }
            syn::Fields::Unnamed(ref fields) => {
                let mut found = None;
                for (i, field) in fields.unnamed.iter().enumerate() {
                    if field.attrs.iter().any(is_analysis_key_attr) {
                        if found.is_some() {
                            return syn::Error::new(
                                field.span(),
                                "duplicate #[analysis_key] field",
                            )
                            .into_compile_error()
                            .into();
                        }
                        found = Some((Ident::new(&i.to_string(), field.span()), field.ty.clone()));
                    }
                }
                found
            }
            syn::Fields::Unit => {
                return syn::Error::new(
                    derive_span,
                    "structs with unit fields cannot derive AnalysisKey",
                )
                .into_compile_error()
                .into()
            }
        },
        syn::Data::Enum(_) => {
            return syn::Error::new(derive_span, "enums cannot derive AnalysisKey")
                .into_compile_error()
                .into()
        }
        syn::Data::Union(_) => {
            return syn::Error::new(derive_span, "unions cannot derive AnalysisKey")
                .into_compile_error()
                .into()
        }
    };

    let (field_id, field_ty) = match found {
        Some(found) => found,
        None => {
            return syn::Error::new(derive_span, "missing #[analysis_key] attribute")
                .into_compile_error()
                .into()
        }
    };

    let quoted = quote! {
        impl #impl_generics AnalysisKey for #id #ty_generics #where_clause {
            type Key = #field_ty;

            fn key(&self) -> Self::Key { self.#field_id }
        }
    };

    proc_macro::TokenStream::from(quoted)
}

#[proc_macro_derive(RewritePassRegistration)]
pub fn derive_rewrite_pass_registration(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(item as DeriveInput);
    let derive_span = derive_input.span();
    let id = derive_input.ident.clone();
    let ctor_name = syn::Ident::new(&format!("__{id}_rewrite_pass_ctor"), derive_span);

    let quoted = quote! {
        inventory::submit!(RewritePassRegistration::new(
            <#id as PassInfo>::FLAG,
            <#id as PassInfo>::HELP,
            #ctor_name,
        ));

        fn #ctor_name() -> Box<dyn RewritePass<Entity = <#id as RewritePass>::Entity>> {
            Box::new(#id::default())
        }
    };

    proc_macro::TokenStream::from(quoted)
}

fn is_analysis_key_attr(attr: &syn::Attribute) -> bool {
    if let syn::Meta::Path(ref path) = attr.meta {
        path.is_ident("analysis_key")
    } else {
        false
    }
}

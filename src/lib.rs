use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Field, Fields, Lit, Meta, Type};

static CB_PATH: &str = "cb";
static CB_ABBR: &str = "abbr";

#[proc_macro_derive(ColonBuilder, attributes(cb))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let struct_def = parse_macro_input!(input);
    impl_derive(&struct_def).into()
}

fn impl_derive(struct_def: &syn::DeriveInput) -> TokenStream {
    let t = match struct_def.data {
        syn::Data::Struct(ref s) => handle_data_struct(s, struct_def),
        _ => panic!("#[derive(PrintFields)] is only defined for structs"),
    };
    t.into()
}

fn handle_data_struct(s: &syn::DataStruct, struct_def: &syn::DeriveInput) -> TokenStream {
    let (fields, methods, method_names) = collect_fields_methods(&s.fields);
    let name = &struct_def.ident;
    let builder_ident = format_ident!("{}Builder", name);

    quote! {
        pub struct #builder_ident;
        impl #builder_ident {
            #(#methods)*
            fn build(s: &str) -> #name {
                #name {
                    #(
                        #fields: #builder_ident::#method_names(s),
                    )*
                }
            }
        }
        impl #name {
            pub fn from_str(s: &str) -> #name {
                #builder_ident::build(s)
            }
        }
    }
}

fn collect_fields_methods(
    fields: &Fields,
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let mut gen_fields = Vec::<TokenStream>::new();
    let mut methods = Vec::<TokenStream>::new();
    let mut method_names = Vec::<TokenStream>::new();

    match fields {
        Fields::Named(fs) => {
            handle_named_field(fs, &mut gen_fields, &mut methods, &mut method_names);
        }
        _ => panic!("Builder pattern only supports structs with named fields."),
    }
    (gen_fields, methods, method_names)
}

fn handle_named_field(fs: &syn::FieldsNamed, gen_fields: &mut Vec<TokenStream>, methods: &mut Vec<TokenStream>, method_names: &mut Vec<TokenStream>) {
    fs.named.iter().for_each(|f| {
        let ident = f.ident.as_ref().unwrap();
        let abbr = get_abbr(f);
        let match_prefix = abbr.unwrap_or(format!("{}", ident));
        let regexp = get_regexp(match_prefix);
        let method_name = format_ident!("set_{}", ident);

        let method = generate_set_method(f, regexp, &method_name);
        gen_fields.push(quote!(#ident));
        methods.push(method);
        method_names.push(quote!(#method_name));
    })
}
fn generate_set_method(f: &Field, regexp: String, method_name: &Ident) -> TokenStream {
    let vec_string_type = syn::parse_quote!(Vec<String>);
    let string_type = syn::parse_quote!(String);
    let option_vec_string_type: Type = syn::parse_quote!(Option<Vec<String>>);
    let ty = &f.ty;
    let get_match = quote!{
        {
            let pat = reg!(#regexp);
            pat.captures(s).and_then(|m| m.name("v").map(|s| s.as_str().trim().to_owned()))
        }
    };
    if ty == &vec_string_type {
        quote! {
            #[allow(non_snake_case)]
            pub fn #method_name(s: &str) -> Vec<String> {
                let m = #get_match;
                m.map(|s| {
                    s.split(',')
                    .into_iter()
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<String>>()
                }).unwrap_or(vec![])
            }
        }
    } else if ty == &string_type {
        quote! {
            #[allow(non_snake_case)]
            pub fn #method_name(s: &str) -> String {
                let m = #get_match;
                m.unwrap_or("".to_string())      
            }
        }
    } else if ty == &option_vec_string_type {
        quote! {
            #[allow(non_snake_case)]
            pub fn #method_name(s: &str) -> Option<Vec<String>> {
                let m = #get_match;
                m.and_then(|m| {
                    let tc: Vec<String> = m
                    .split(',')
                    .into_iter()
                    .map(|s| s.trim().to_owned())
                    .collect();
                    Some(tc)
                })
            }
        }
    } else {
        quote! {
            #[allow(non_snake_case)]
            pub fn #method_name(s: &str) -> Option<String> {
                #get_match
            }
        }
    }
}

fn get_abbr(f: &syn::Field) -> Option<String> {
    f.attrs
        .iter()
        .find(|attr| attr.path.is_ident(CB_PATH))
        .and_then(|attr|
            {
                match attr.parse_meta() {
                    Ok(ref data) => process_meta(data),
                    Err(_) => None,
            }
        })
}

fn process_meta(data: &Meta) -> Option<String> {
    match data {
        Meta::NameValue(name_value) => {
            if name_value.path.is_ident(CB_ABBR) {
                if let Lit::Str(lit_str) = &name_value.lit {
                    Some(lit_str.value())
                } else {
                    None
                }
            } else {
                None
            }
        }
        syn::Meta::Path(_) => None,
        syn::Meta::List(p) => process_meta_list(p),
    }
}

fn process_meta_list(p: &syn::MetaList) -> Option<String> {
    p.nested.iter().find_map(|att| match att {
        syn::NestedMeta::Meta(meta) => process_meta(meta),
        _ => todo!(),
    })
}

fn get_regexp(field: String) -> String {
    format!(r"{}:\s*(?P<v>.+?)((\r?\n)|$)", field)
}

//! Parse the associated attribute `#[holder(...)]` with `#[derive(Holder)]`
//!
//! There are three options:
//!
//! - `#[holder(table = {path::to::table::struct})]`
//! - `#[holder(field = {field_ident})]`
//! - `#[holder(use_place_holder)]`
//! - `#[holder(generate_deserialize)]`
//!

#[derive(Debug, Clone, PartialEq)]
pub struct HolderAttr {
    pub table: Option<syn::Path>,
    pub field: Option<syn::Ident>,
    pub place_holder: bool,
    pub generate_deserialize: bool,
}

impl HolderAttr {
    pub fn parse(attrs: &[syn::Attribute]) -> Self {
        let mut table = None;
        let mut field = None;
        let mut place_holder = false;
        let mut generate_deserialize = false;

        for attr in attrs {
            match attr.parse_args().unwrap() {
                Attr::Table(path) => {
                    table = Some(path);
                }
                Attr::Field(ident) => {
                    field = Some(ident);
                }
                Attr::PlaceHolder => {
                    place_holder = true;
                }
                Attr::GenerateDeserialize => {
                    generate_deserialize = true;
                }
            }
        }
        HolderAttr {
            table,
            field,
            place_holder,
            generate_deserialize,
        }
    }
}

#[derive(Debug)]
enum Attr {
    Table(syn::Path),
    Field(syn::Ident),
    PlaceHolder,
    GenerateDeserialize,
}

impl syn::parse::Parse for Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
            "table" => {
                let _eq: syn::Token![=] = input.parse()?;
                let path = input.parse()?;
                Ok(Attr::Table(path))
            }
            "field" => {
                let _eq: syn::Token![=] = input.parse()?;
                let ident = input.parse()?;
                Ok(Attr::Field(ident))
            }
            "use_place_holder" => Ok(Attr::PlaceHolder),
            "generate_deserialize" => Ok(Attr::GenerateDeserialize),
            _ => Err(syn::parse::Error::new(
                ident.span(),
                "expected `table`, `field`, or `use_place_holder`",
            )),
        }
    }
}

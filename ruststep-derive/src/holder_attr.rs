#[derive(Debug)]
struct TableAttr {
    table: syn::Ident,
    eq: syn::Token![=],
    name: syn::Ident,
}

impl syn::parse::Parse for TableAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(TableAttr {
            table: input.parse()?,
            eq: input.parse()?,
            name: input.parse()?,
        })
    }
}

/// To parse `#[holder(table = Table)]` attribute to get `Table`
pub fn get_table_ident(ast: &syn::DeriveInput) -> syn::Ident {
    for attr in &ast.attrs {
        if attr.path.is_ident("holder") {
            let table: TableAttr = attr.parse_args().unwrap();
            return table.name;
        }
    }
    panic!("Table is not specified for Holder")
}

/// Check a struct member has `#[holder(use_place_holder)]` attribute
pub fn is_use_place_holder(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        if attr.path.is_ident("holder") {
            let flag: syn::Ident = attr.parse_args().unwrap();
            return flag == "use_place_holder";
        }
    }
    false
}

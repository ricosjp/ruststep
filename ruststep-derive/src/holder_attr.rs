#[derive(Debug)]
struct TableAttrParse {
    table_prefix: syn::Ident,
    eq1: syn::Token![=],
    table: syn::Ident,
    comma: syn::Token![,],
    field_prefix: syn::Ident,
    eq2: syn::Token![=],
    field: syn::Ident,
}

impl syn::parse::Parse for TableAttrParse {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(TableAttrParse {
            table_prefix: input.parse()?,
            eq1: input.parse()?,
            table: input.parse()?,
            comma: input.parse()?,
            field_prefix: input.parse()?,
            eq2: input.parse()?,
            field: input.parse()?,
        })
    }
}

pub struct TableAttr {
    pub table: syn::Ident,
    pub field: syn::Ident,
}

/// To parse `#[holder(table = Table, field = a)]` attribute to get `Table`
pub fn parse_table_attr(ast: &syn::DeriveInput) -> TableAttr {
    for attr in &ast.attrs {
        if attr.path.is_ident("holder") {
            let TableAttrParse { table, field, .. } = attr.parse_args().unwrap();
            return TableAttr { table, field };
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

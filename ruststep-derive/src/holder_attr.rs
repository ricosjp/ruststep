#[derive(Debug)]
struct TableAttrParse {
    table: syn::Ident,
    eq1: syn::Token![=],
    table_name: syn::Ident,
    comma: syn::Token![,],
    field: syn::Ident,
    eq2: syn::Token![=],
    field_name: syn::Ident,
}

impl syn::parse::Parse for TableAttrParse {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(TableAttrParse {
            table: input.parse()?,
            eq1: input.parse()?,
            table_name: input.parse()?,
            comma: input.parse()?,
            field: input.parse()?,
            eq2: input.parse()?,
            field_name: input.parse()?,
        })
    }
}

pub struct TableAttr {
    pub table_name: syn::Ident,
    pub field_name: syn::Ident,
}

/// To parse `#[holder(table = Table, field = a)]` attribute to get `Table`
pub fn parse_table_attr(ast: &syn::DeriveInput) -> TableAttr {
    for attr in &ast.attrs {
        if attr.path.is_ident("holder") {
            let TableAttrParse {
                table_name,
                field_name,
                ..
            } = attr.parse_args().unwrap();
            return TableAttr {
                table_name,
                field_name,
            };
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

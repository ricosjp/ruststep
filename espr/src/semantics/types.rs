use super::entity::Entity;
use crate::parser::simple_data_type::SimpleType;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    SimpleType(SimpleType),

    // aggregate types
    Array {
        index_range: (isize, isize),
        optional: bool,
        unique: bool,
        base_type: String,
    },
    List {
        length_range: (usize, Option<usize>),
        unique: bool,
        base_type: String,
    },
    Bag {
        length_range: (usize, Option<usize>),
        base_type: String,
    },
    Set {
        length_range: (usize, Option<usize>),
        base_type: String,
    },

    // named types
    Entity(Entity),
    Defined {
        name: String,
        underlying: String,
    },

    // constructed types
    Enumerate {
        name: String,
        items: Vec<String>,
    },
    Select {
        name: String,
        items: Vec<String>,
    },

    // generalized types
    Generic,
    Aggrigate,
}

impl Type {
    pub fn is_simple(&self) -> bool {
        match self {
            Type::SimpleType(_) => true,
            _ => false,
        }
    }

    pub fn is_aggrigation(&self) -> bool {
        match self {
            Type::Array { .. } => true,
            Type::List { .. } => true,
            Type::Bag { .. } => true,
            Type::Set { .. } => true,
            _ => false,
        }
    }

    pub fn is_named(&self) -> bool {
        match self {
            Type::Entity { .. } => true,
            Type::Defined { .. } => true,
            _ => false,
        }
    }

    pub fn is_constructed(&self) -> bool {
        match self {
            Type::Enumerate { .. } => true,
            Type::Select { .. } => true,
            _ => false,
        }
    }

    pub fn is_generalized(&self) -> bool {
        match self {
            Type::Generic => true,
            Type::Aggrigate => true,
            _ => false,
        }
    }

    pub fn is_base(&self) -> bool {
        self.is_aggrigation() | self.is_simple() | self.is_named()
    }

    pub fn is_parameter(&self) -> bool {
        self.is_generalized() | self.is_named() | self.is_simple()
    }

    pub fn is_underlying(&self) -> bool {
        match self {
            Type::Defined { .. } => true,
            _ => self.is_constructed() | self.is_aggrigation() | self.is_simple(),
        }
    }
}

impl ToTokens for SimpleType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use SimpleType::*;
        match self {
            Number => tokens.append(format_ident!("f64")),
            Real => tokens.append(format_ident!("f64")),
            Integer => tokens.append(format_ident!("i64")),
            Logical => tokens.append_all(quote! { ::espr_runtime::Logial }),
            Boolen => tokens.append(format_ident!("bool")),
            _ => unimplemented!(),
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Type::SimpleType(ty) => {
                ty.to_tokens(tokens);
            }
            Type::Entity(entity) => {
                entity.to_tokens(tokens);
            }
            _ => unimplemented!(),
        }
    }
}

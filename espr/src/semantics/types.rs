use super::Entity;
use crate::parser::SimpleType;
use inflector::Inflector;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberVariant {
    pub name: String,
    pub type_name: String,
    pub optional: bool,
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

    pub fn rust_type_name(&self) -> String {
        match self {
            Type::SimpleType(ty) => format!("{:?}", ty),
            Type::Array {
                index_range,
                base_type,
                ..
            } => format!("[{}; {}]", base_type, index_range.1 - index_range.0),
            Type::List { base_type, .. } => format!("Vec<{}>", base_type),
            Type::Bag { base_type, .. } => format!("Vec<{}>", base_type),
            Type::Set { base_type, .. } => format!("HashSet<{}>", base_type),
            Type::Entity(entity) => entity.name.to_pascal_case(),
            Type::Defined { name, .. } => name.to_pascal_case(),
            Type::Enumerate { name, .. } => name.to_pascal_case(),
            Type::Select { name, .. } => name.to_pascal_case(),
            Type::Generic => "dyn Any".into(),
            Type::Aggrigate => "dyn Any".into(),
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
            Type::Defined { underlying, .. } => {
                tokens.append_all(defined_type_struct(&self.rust_type_name(), &underlying))
            }
            _ => unimplemented!(),
        }
    }
}

fn defined_type_struct(name: &str, underlying: &str) -> TokenStream {
    let name = format_ident!("{}", name);
    let underlying = format_ident!("{}", underlying);

    quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub struct #name ( #underlying );

        impl #name {
            pub fn new(entity: #underlying) -> Self {
                Self ( entity )
            }
        }
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    // simple types
    Number,
    Real,
    Integer,
    String,
    Boolean,
    Logical,
    Binary(Option<usize>),

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
    Entity {
        name: String,
        members: Vec<MemberVariant>,
    },
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
    #[inline(always)]
    pub fn is_simple(&self) -> bool {
        match self {
            Type::Number => true,
            Type::Real => true,
            Type::Integer => true,
            Type::String => true,
            Type::Boolean => true,
            Type::Logical => true,
            Type::Binary(_) => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_aggrigation(&self) -> bool {
        match self {
            Type::Array { .. } => true,
            Type::List { .. } => true,
            Type::Bag { .. } => true,
            Type::Set { .. } => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_named(&self) -> bool {
        match self {
            Type::Entity { .. } => true,
            Type::Defined { .. } => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_constructed(&self) -> bool {
        match self {
            Type::Enumerate { .. } => true,
            Type::Select { .. } => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_generalized(&self) -> bool {
        match self {
            Type::Generic => true,
            Type::Aggrigate => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_base(&self) -> bool { self.is_aggrigation() | self.is_simple() | self.is_named() }

    #[inline(always)]
    pub fn is_parameter(&self) -> bool {
        self.is_generalized() | self.is_named() | self.is_simple()
    }

    #[inline(always)]
    pub fn is_underlying(&self) -> bool {
        match self {
            Type::Defined { .. } => true,
            _ => self.is_constructed() | self.is_aggrigation() | self.is_simple(),
        }
    }

    pub fn rust_type_name(&self) -> String {
        match self {
            Type::Number => "f64".into(),
            Type::Real => "f64".into(),
            Type::Integer => "isize".into(),
            Type::String => "String".into(),
            Type::Boolean => "bool".into(),
            Type::Logical => "Logical".into(),
            Type::Binary(None) => "Vec<bool>".into(),
            Type::Binary(Some(len)) => format!("[bool; {}]", len),
            Type::Array {
                index_range,
                base_type,
                ..
            } => format!("[{}; {}]", base_type, index_range.1 - index_range.0),
            Type::List { base_type, .. } => format!("Vec<{}>", base_type),
            Type::Bag { base_type, .. } => format!("Vec<{}>", base_type),
            Type::Set { base_type, .. } => format!("HashSet<{}>", base_type),
            Type::Entity { name, .. } => name.to_pascal_case(),
            Type::Defined { name, .. } => name.to_pascal_case(),
            Type::Enumerate { name, .. } => name.to_pascal_case(),
            Type::Select { name, .. } => name.to_pascal_case(),
            Type::Generic => "dyn Any".into(),
            Type::Aggrigate => "dyn Any".into(),
        }
    }

    pub fn struct_definition(&self) -> String {
        match self {
            Type::Entity { members, .. } => {
                entity_struct_definition(self.rust_type_name(), members)
            }
            Type::Defined { underlying, .. } => {
                defined_type_struct(self.rust_type_name(), &underlying)
            }
            _ => String::new(),
        }
    }
}

fn entity_struct_definition(name: String, members: &Vec<MemberVariant>) -> String {
    let mut res = String::new();
    res += &format!("#[derive(Clone, Debug, PartialEq)]\npub struct {} {{ ", name);
    for member in members {
        let type_name = if member.optional {
            format!("Option<{}>", member.type_name.to_pascal_case())
        } else {
            member.type_name.to_pascal_case()
        };
        res += &format!("{}: {}, ", member.name, type_name)
    }
    res += &format!("}}\nimpl {} {{ pub fn new(", name);
    for member in members {
        let type_name = if member.optional {
            format!("Option<{}>", member.type_name.to_pascal_case())
        } else {
            member.type_name.to_pascal_case()
        };
        res += &format!("{}: {}, ", member.name, type_name)
    }
    res += &format!(") -> {} {{ {} {{ ", name, name);
    for member in members {
        res += &format!("{}, ", member.name);
    }
    res += "} } }\n";
    res
}

fn defined_type_struct(name: String, underlying: &String) -> String {
    let mut res = String::new();
    res += &format!("#[derive(Clone, Debug, PartialEq)]\n");
    res += &format!("pub struct {}({});\n", name, underlying);
    res += &format!(
        "impl {} {{ fn new(entity: {}) -> {} {{ {}(entity) }} }}\n",
        name, underlying, name, name
    );
    res
}

#[test]
fn print_entity_definition() {
    let entity = Type::Entity {
        name: "test_struct_type".into(),
        members: vec![
            MemberVariant {
                name: "m_int".into(),
                type_name: "usize".into(),
                optional: true,
            },
            MemberVariant {
                name: "m_float".into(),
                type_name: "f64".into(),
                optional: false,
            },
        ],
    };
    println!("{}", entity.struct_definition());
}

#[test]
fn print_defined_definition() {
    let defined = Type::Defined {
        name: "test_defined_type".into(),
        underlying: "FormerType".into(),
    };
    println!("{}", defined.struct_definition());
}

#[derive(Clone, Debug)]
pub struct Schema {
    pub name: String,
    pub types: Vec<Type>,
}

impl Schema {
    pub fn rust_code(&self) -> String {
        let mut res = String::new();
        res += &format!("mod {} {{\n", self.name.to_snake_case());
        for current_type in &self.types {
            res += &current_type.struct_definition();
        }
        res += "}";
        res
    }
}

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
        base_type: Box<Type>,
    },
    List {
        length_range: (usize, Option<usize>),
        unique: bool,
        base_type: Box<Type>,
    },
    Bag {
        length_range: (usize, Option<usize>),
        base_type: Box<Type>,
    },
    Set {
        length_range: (usize, Option<usize>),
        base_type: Box<Type>,
    },
    // named types
    Entity {
        name: String,
        members: Vec<MemberVariant>,
    },
    Defined {
        name: String,
        underlying: Box<Type>,
    },

    // constructed types
    Enumerate(Vec<String>),
    Select(Vec<Type>),

    // generalized types
    Generic,
    Aggrigate,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberVariant {
    name: String,
    member_type: Type,
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
            Type::Array {..} => true,
            Type::List {..} => true,
            Type::Bag {..} => true,
            Type::Set {..} => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_named(&self) -> bool {
        match self {
            Type::Entity {..} => true,
            Type::Defined {..} => true,
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_constructed(&self) -> bool {
        match self {
            Type::Enumerate {..} => true,
            Type::Select {..} => true,
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
    pub fn is_base(&self) -> bool {
        self.is_aggrigation() | self.is_simple() | self.is_named()
    }

    #[inline(always)]
    pub fn is_parameter(&self) -> bool {
        self.is_generalized() | self.is_named() | self.is_simple()
    }

    #[inline(always)]
    pub fn is_underlying(&self) -> bool {
        match self {
            Type::Defined {..} => true,
            _ => self.is_constructed() | self.is_aggrigation() | self.is_simple()
        }
    }

}
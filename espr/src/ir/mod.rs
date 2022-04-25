//! Intermediate Representation ([IR]) legalized (semantically analyzed) from [SyntaxTree]
//!
//! Legalize procedure consists of three steps:
//!
//! - Create [Namespace]
//! - Resolve SubType/SuperType consists to yield [Constraints]
//! - [Legalize] each AST portions
//!
//! First two step are global analysis, which make the last step local.
//!
//! Namespace creation
//! -------------------
//! Legalize phase starts with [Namespace] creation.
//! This step also introduces [Scope] and [Path] to represent scopes and names in EXPRESS schema.
//! Using [Namespace], we can look up corresponding [Path] to a local identifier appears in a [Scope].
//!
//! ```text
//! SCHEMA sc1;  -- Scope "sc1[schema]" starts
//!   ENTITY a;  -- Path "sc1[schema].a[entity]" is registered
//!     x: REAL;
//!     y: REAL;
//!   END_ENTITY;
//!
//!   ENTITY b;  -- Path "sc1[schema].b[entity]" is registered
//!     z: REAL;
//!     a: a;    -- Identifier "a" is resolved as a path "sc1[schema].a[entity]"
//!   END_ENTITY;
//! END_SCHEMA;  -- Scope "sc1[schema]" ends
//! ```
//!
//! SubType/SuperType constraints
//! ------------------------------
//! ENTITY may have subtype or supertype constraints.
//!
//! ```text
//! ENTITY person;
//!   name: STRING;
//! END_ENTITY;
//!
//! ENTITY employee SUBTYPE OF (person);
//!   pay: INTEGER;
//! END_ENTITY;
//!
//! ENTITY student SUBTYPE OF (person);
//!   school_name: STRING;
//! END_ENTITY;
//! ```
//!
//! This means `employee` has a field `pay` in addition to `name` inherited from `person`,
//! and an `employee` can be instantiated as a `person`.
//! Instances which contains two or more entity value are called "complex entity instance",
//! and they are mapped into exchange structures using one of two rules,
//! "internal mapping" or "external mapping":
//!
//! ```text
//! /* internal mapping */
//! #1 = EMPLOYEE('Hitori Goto', 10);
//! #2 = STUDENT('Ikuno Kita', 'Shuka');
//!
//! /* external mapping */
//! #3 = (PERSON('Hitori Goto') EMPLOYEE(10));
//! #4 = (PERSON('Ikuno Kita') STUDENT('Shuka));
//! #5 = (PERSON('Nizika Iziti') EMPLOYEE(15) STUDENT('Simokitazawa'))
//! ```
//!
//! Using internal mapping,
//! the inherited attributes (`name`) shall appear sequentially
//! prior to the explicit attributes (`pay`, `school_name`).
//! Using external mapping, an instance is represented by a list of
//! "partial complex entity value" enclosed by `()`.
//! The instances `#1` (`#2`) described by internal mapping
//! and `#3` (`#4`) described by external mapping are same value of `employee` (`student`) entity,
//! but internal mapping cannot describe `#5` case.
//! Different from usual Object-Oriented Programming (OOP) languages like C++ or Python,
//! `person` can be both `employee` and `student` simultaneously,
//! i.e. a `person` object may have both `pay` field and `school_name` field like as `#5`.
//!
//! This type of inheritance is called `ANDOR` in EXPRESS,
//! and it is the default constraint for supertype.
//! We can write this constraint explicitly in the entity declaration of `person`:
//!
//! ```text
//! ENTITY person SUPERTYPE OF (employee ANDOR student);
//!   name: STRING;
//! END_ENTITY;
//! ```
//!
//! or as a separate `SUBTYPE_CONSTRAINT` declaration:
//!
//! ```text
//! SUBTYPE_CONSTRAINT person_prop FOR person;
//!   employee ANDOR student;
//! END_SUBTYPE_CONSTRAINT;
//! ```
//!
//! We cannot determine the subtypes of an entity from its `ENTITY` declaration
//! due to default constraints.
//! `SUBTYPE OF` relation are gathered into [Constraints] struct
//! to look up subtype paths from supertype path before legalizing AST of entities.
//!
//! There is two other types of constraints. First is `ONEOF` constraint:
//!
//! ```text
//! ENTITY pet;
//!   name : pet_name;
//! END_ENTITY;
//!
//! SUBTYPE_CONSTRAINT separate_species FOR pet;
//!   ABSTRACT SUPERTYPE;
//!   ONEOF(cat, rabbit, dog);
//! END_SUBTYPE_CONSTRAINT;
//!
//! ENTITY cat SUBTYPE OF (pet);
//! END_ENTITY;
//!
//! ENTITY rabbit SUBTYPE OF (pet);
//! END_ENTITY;
//!
//! ENTITY dog SUBTYPE OF (pet);
//! END_ENTITY;
//! ```
//!
//! You know a pet cannot be both cat and rabbit in real world.
//! This is represented by `ONEOF` constraint in `SUBTYPE_CONSTRAINT` declaration.
//! Second is `AND` constraint:
//!
//! ```text
//! ENTITY person;
//! END_ENTITY;
//!
//! ENTITY male SUBTYPE OF (person);
//! END_ENTITY;
//!
//! ENTITY female SUBTYPE OF (person);
//! END_ENTITY;
//!
//! ENTITY citizen SUBTYPE OF (person);
//! END_ENTITY;
//!
//! ENTITY alien SUBTYPE OF (person);
//! END_ENTITY;
//!
//! SUBTYPE_CONSTRAINT person_prop FOR person;
//!   ONEOF(male, female) AND ONEOF(citizen, alien);
//! END_SUBTYPE_CONSTRAINT;
//! ```
//!
//! `AND` behaves as that for boolean.
//!
//! [Legalize] trait
//! -----------------
//! Most of structs in this sub-module implements [Legalize] trait
//! for creating it from a corresponding AST portion.
//! `Legalize::legalize` is called recursively while traversing AST.
//! These structs, called IRs (intermediate representations), are designed with
//! following rules:
//!
//! - Code generation only looks IRs, never looks AST.
//!   Every information required for code generation must be contained in IR.
//! - Code generation does not execute global analysis,
//!   e.g. check if a type reference refers a primitive type or not.
//!
//! This crate is motivated for generating Rust code,
//! but is designed to use for generating other contents,
//! e.g. Python code or HTML reference.
//!

mod complex_entity;
mod constraints;
mod entity;
mod namespace;
mod schema;
mod scope;
mod type_decl;
mod type_ref;

pub use complex_entity::*;
pub use constraints::*;
pub use entity::*;
pub use namespace::*;
pub use schema::*;
pub use scope::*;
pub use type_decl::*;
pub use type_ref::*;

use crate::ast::SyntaxTree;
use thiserror::Error;

/// Semantic errors
#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Not found the Type {name} referred in scope {scope}")]
    TypeNotFound { name: String, scope: Scope },

    #[error("Invalid path: {0}")]
    InvalidPath(Path),

    #[error("Same item ({0}) is declared multiple times")]
    DuplicatedDeclaration(Path),
}

/// Legalize partial AST input into corresponding intermediate representation
pub trait Legalize: Sized {
    /// AST portion
    type Input;

    fn legalize(
        namespace: &Namespace,
        constraints: &Constraints,
        scope: &Scope,
        input: &Self::Input,
    ) -> Result<Self, SemanticError>;
}

/// Intermediate Representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IR {
    pub schemas: Vec<Schema>,
}

impl IR {
    pub fn from_syntax_tree(st: &SyntaxTree) -> Result<Self, SemanticError> {
        let ns = Namespace::new(st);
        let ss = Constraints::new(&ns, st)?;
        let ir = Self::legalize(&ns, &ss, &Scope::root(), st)?;
        Ok(ir)
    }
}

impl Legalize for IR {
    type Input = SyntaxTree;
    fn legalize(
        ns: &Namespace,
        ss: &Constraints,
        scope: &Scope,
        syn: &SyntaxTree,
    ) -> Result<Self, SemanticError> {
        let schemas = syn
            .schemas
            .iter()
            .map(|schema| Schema::legalize(ns, ss, scope, schema))
            .collect::<Result<Vec<Schema>, SemanticError>>()?;
        Ok(IR { schemas })
    }
}

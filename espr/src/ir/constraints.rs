//! Partial complex entities described in ISO-10303-11 Annex B

use super::*;
use crate::ast;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Expression appears in `SUBTYPE_CONSTRAINT` with resolved [Path]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintExpr {
    Reference(Path),
    AndOr(Vec<ConstraintExpr>),
    And(Vec<ConstraintExpr>),
    OneOf(Vec<ConstraintExpr>),
}

impl ConstraintExpr {
    /// Check if `path` occurs in this expression
    pub fn is_in(&self, path: &Path) -> bool {
        use ConstraintExpr::*;
        match self {
            Reference(p) => p == path,
            AndOr(exprs) | And(exprs) | OneOf(exprs) => exprs.iter().any(|expr| expr.is_in(path)),
        }
    }

    pub fn andor(mut self, rhs: Self) -> Self {
        self.andor_mut(rhs);
        self
    }

    pub fn andor_mut(&mut self, rhs: Self) {
        use ConstraintExpr::*;
        match (self, rhs) {
            (AndOr(ref mut a), AndOr(mut b)) => {
                a.append(&mut b);
            }
            (AndOr(ref mut a), b @ _) => {
                a.push(b);
            }
            (a @ _, b @ AndOr(_)) => {
                let s = std::mem::replace(a, b);
                match a {
                    AndOr(factors) => factors.insert(0, s),
                    _ => unreachable!(),
                }
            }
            (a @ _, b @ _) => {
                let s = std::mem::replace(a, AndOr(vec![b]));
                match a {
                    AndOr(factors) => factors.insert(0, s),
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn from_ast_expr(
        ns: &Namespace,
        scope: &Scope,
        expr: &ast::SuperTypeExpression,
    ) -> Result<Self, SemanticError> {
        use ast::SuperTypeExpression::*;
        Ok(match expr {
            Reference(name) => Self::Reference(ns.resolve(scope, &name)?.0),
            AndOr { factors } => Self::AndOr(
                factors
                    .iter()
                    .map(|f| Self::from_ast_expr(ns, scope, f))
                    .collect::<Result<Vec<Self>, SemanticError>>()?,
            ),
            And { terms } => Self::And(
                terms
                    .iter()
                    .map(|f| Self::from_ast_expr(ns, scope, f))
                    .collect::<Result<Vec<Self>, SemanticError>>()?,
            ),
            OneOf { exprs } => Self::OneOf(
                exprs
                    .iter()
                    .map(|f| Self::from_ast_expr(ns, scope, f))
                    .collect::<Result<Vec<Self>, SemanticError>>()?,
            ),
        })
    }
}

/// Global constraints in EXPRESS components
#[derive(Debug, PartialEq, Eq)]
pub struct Constraints {
    /// Each super-type can be instantiable as its subtypes,
    /// but possible subtypes cannot be determined from local description in EXPRESS.
    pub instantiables: HashMap<Path, Vec<Vec<Path>>>,
}

// Execute b), c), and d) steps of the algorithm described in the section B.3
//
// - Step a) has been done while namespace creation.
//
pub fn gather_constraint_expr(
    ns: &Namespace,
    st: &SyntaxTree,
) -> Result<HashMap<Path, ConstraintExpr>, SemanticError> {
    let root = Scope::root();
    let mut exprs: HashMap<Path, ConstraintExpr> = HashMap::new();

    // b) Convert `SUPERTYPE OF` into `SUBTYPE_CONSTRAINT`
    //
    // Be sure that `SUPERTYPE OF` declaration with complex constraint
    // using `ONEOF`, `AND` and `ANDOR` are deprecated:
    //
    // ISO-10303-11 (2004, en) Page 56, Note 1
    // > In order that existing schemas remain valid,
    // > the declaration of subtype/supertype constraints
    // > that use the keywords ONEOF, ANDOR, or AND within
    // > the declaration of an entity, as described in this sub-clause,
    // > remains valid under this edition 2 of EXPRESS.
    // > However, its use is deprecated, and its removal is planned
    // > in future editions. The use of the subtype constraint (see 9.7)
    // > is encouraged instead.
    //
    for schema in &st.schemas {
        let scope = root.schema(&schema.name);
        for entity in &schema.entities {
            match &entity.constraint {
                Some(ast::Constraint::SuperTypeRule(expr)) => {
                    let result = exprs.insert(
                        Path::entity(&scope, &entity.name),
                        ConstraintExpr::from_ast_expr(ns, &scope, expr)?,
                    );
                    // This insert must be first time unless same ENTITY declaration exists
                    if result.is_some() {
                        return Err(SemanticError::DuplicatedDeclaration(Path::entity(
                            &scope,
                            &entity.name,
                        )));
                    }
                }
                _ => continue,
            }
        }
    }

    // d) Combine `SUBTYPE_CONSTRAINT` into single expression by `ANDOR`
    //
    // The step d) is done before c) because c) have to look up all `SUBTYPE_CONSTRAINT`.
    // New `SUBTYPE_CONSTRAINT` introduced in the step c) will be merged while c).
    //
    for schema in &st.schemas {
        let scope = root.schema(&schema.name);
        for constraint in &schema.subtype_constraints {
            if let Some(expr) = &constraint.expr {
                let (path, _index) = ns.resolve(&scope, &constraint.entity)?;
                let expr = ConstraintExpr::from_ast_expr(ns, &scope, expr)?;
                match exprs.entry(path) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().andor_mut(expr);
                    }
                    Entry::Vacant(e) => {
                        e.insert(expr);
                    }
                }
            }
        }
    }

    // c) Add default constraint (described in 9.2.5.6),
    //    i.e. convert `SUBTYPE OF` into `SUBTYPE_CONSTRAINT`
    //
    // We'd like to list up subtypes for each supertype,
    // but `SUBTYPE OF` description exists on subtype's `ENTITY` declaration.
    //
    // c-1) Thus, we first read every ENTITY to gather sub- to super-type dependencies,
    let mut super_to_sub: HashMap<Path /* super */, Vec<Path> /* sub */> = HashMap::new();
    for schema in &st.schemas {
        let scope = root.schema(&schema.name);
        for entity in &schema.entities {
            if let Some(subtype_decl) = &entity.subtype_of {
                for sup_name in &subtype_decl.entity_references {
                    let (sup, _) = ns.resolve(&scope, sup_name)?;
                    let subs = super_to_sub.entry(sup).or_default();
                    let sub = Path::entity(&scope, &entity.name);
                    subs.push(sub);
                }
            }
        }
    }
    // c-2) and reverse it.
    for (sup, subs) in super_to_sub {
        match exprs.entry(sup) {
            Entry::Occupied(mut e) => {
                // Gather subtype does not occur in other `SUBTYPE_CONSTRAINT`
                let subs: Vec<ConstraintExpr> = subs
                    .into_iter()
                    .filter_map(|sub| {
                        if e.get().is_in(&sub) {
                            None
                        } else {
                            Some(ConstraintExpr::Reference(sub))
                        }
                    })
                    .collect();
                if !subs.is_empty() {
                    e.get_mut().andor_mut(ConstraintExpr::AndOr(subs));
                }
            }
            Entry::Vacant(e) => {
                // No `SUBTYPE_CONSTRAINT` for this supertype
                let subs = subs.into_iter().map(ConstraintExpr::Reference).collect();
                e.insert(ConstraintExpr::AndOr(subs));
            }
        }
    }

    Ok(exprs)
}

impl Constraints {
    pub fn new(ns: &Namespace, st: &SyntaxTree) -> Result<Self, SemanticError> {
        let mut instantiables = HashMap::new();
        let root = Scope::root();
        for schema in &st.schemas {
            let scope = root.schema(&schema.name);

            // Be sure that `SUPERTYPE OF` declaration with complex constraint
            // using `ONEOF`, `AND` and `ANDOR` are deprecated:
            //
            // ISO-10303-11 (2004, en) Page 56, Note 1
            // > In order that existing schemas remain valid,
            // > the declaration of subtype/supertype constraints
            // > that use the keywords ONEOF, ANDOR, or AND within
            // > the declaration of an entity, as described in this sub-clause,
            // > remains valid under this edition 2 of EXPRESS.
            // > However, its use is deprecated, and its removal is planned
            // > in future editions. The use of the subtype constraint (see 9.7)
            // > is encouraged instead.
            //
            for entity in &schema.entities {
                match &entity.constraint {
                    Some(ast::Constraint::SuperTypeRule(expr)) => {
                        let path = Path::new(&scope, ScopeType::Entity, &entity.name);
                        instantiables.insert(path, Instantiables::from_expr(ns, &scope, expr)?);
                    }
                    _ => continue,
                }
            }

            for constraint in &schema.subtype_constraints {
                if let Some(expr) = &constraint.expr {
                    let (path, _index) = ns.resolve(&scope, &constraint.entity)?;
                    let is = Instantiables::from_expr(ns, &scope, expr)?;
                    match instantiables.entry(path) {
                        Entry::Occupied(mut e) => {
                            // ISO-10303-11 (2004, en) Annex B.3 "Interpreting the possible complex entity data types"
                            // > Combine the subtype expressions sxi of these constraints
                            // > into a single subtype constraint sti of the form:
                            // > (sx1 ANDOR sx2 ANDOR sx3 . . . ANDOR sxk ).
                            e.insert(Instantiables::andor(vec![e.get().clone(), is]));
                        }
                        Entry::Vacant(e) => {
                            e.insert(is);
                        }
                    }
                }
            }
        }

        // Add default constraints
        //
        // ISO-10303-11 (2004, en) 9.2.5.6 Default constraint between subtypes
        // > If no supertype constraint is mentioned in the declaration of an entity,
        // > the subtypes (if any) shall be mutually inclusive, that is,
        // > as if all subtypes were implicitly mentioned in an ANDOR construct.
        //
        let mut super_to_sub = HashMap::new();
        for schema in &st.schemas {
            let scope = root.schema(&schema.name);
            for entity in &schema.entities {
                let sub = Path::entity(&scope, &entity.name);
                let (_ast, index) = ns.get(&sub)?;
                if let Some(subtype_decl) = &entity.subtype_of {
                    for sup_name in &subtype_decl.entity_references {
                        let (sup, _) = ns.resolve(&scope, sup_name)?;
                        let indices: &mut Vec<usize> = super_to_sub.entry(sup).or_default();
                        indices.push(index);
                    }
                }
            }
        }
        // Apply ANDOR to gathered indices
        for (sup, indices) in super_to_sub {
            let mut is: Vec<_> = indices
                .into_iter()
                .map(|index| Instantiables::single(index))
                .collect();
            match instantiables.entry(sup) {
                Entry::Occupied(mut e) => {
                    is.push(e.get().clone());
                    e.insert(Instantiables::andor(is));
                }
                Entry::Vacant(e) => {
                    e.insert(Instantiables::andor(is));
                }
            }
        }

        // Replace indices to Path using Namespace
        let instantiables = instantiables
            .into_iter()
            .map(|(path, it)| {
                (
                    path,
                    it.parts
                        .iter()
                        .map(|pce| {
                            pce.indices
                                .iter()
                                .map(|index| {
                                    let (path, _ast) = &ns[*index];
                                    path.clone()
                                })
                                .collect()
                        })
                        .collect(),
                )
            })
            .collect();
        Ok(Constraints { instantiables })
    }

    pub fn is_supertype(&self, path: &Path) -> bool {
        self.instantiables.contains_key(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_expr_is_in() {
        let root = Scope::root();
        let a = ConstraintExpr::Reference(Path::entity(&root, "a"));
        assert!(a.is_in(&Path::entity(&root, "a")));
        assert!(!a.is_in(&Path::entity(&root, "b")));
    }

    #[test]
    fn constraint_expr_andor() {
        let root = Scope::root();
        // a ANDOR b
        let a = ConstraintExpr::Reference(Path::entity(&root, "a"));
        let b = ConstraintExpr::Reference(Path::entity(&root, "b"));
        assert_eq!(
            a.clone().andor(b.clone()),
            ConstraintExpr::AndOr(vec![a, b])
        );

        // (a ANDOR b) ANDOR c
        let a = ConstraintExpr::Reference(Path::entity(&root, "a"));
        let b = ConstraintExpr::Reference(Path::entity(&root, "b"));
        let c = ConstraintExpr::Reference(Path::entity(&root, "c"));
        let ab = a.clone().andor(b.clone());
        assert_eq!(ab.andor(c.clone()), ConstraintExpr::AndOr(vec![a, b, c]))
    }

    /// Based on `ONEOF` example in ISO-10303-11
    const PET: &str = r#"
    SCHEMA test_schema;
      ENTITY pet;
        name : pet_name;
      END_ENTITY;

      SUBTYPE_CONSTRAINT separate_species FOR pet;
        ABSTRACT SUPERTYPE;
        ONEOF(cat, rabbit, dog);
      END_SUBTYPE_CONSTRAINT;

      ENTITY cat SUBTYPE OF (pet);
      END_ENTITY;

      ENTITY rabbit SUBTYPE OF (pet);
      END_ENTITY;

      ENTITY dog SUBTYPE OF (pet);
      END_ENTITY;
    END_SCHEMA;
    "#;

    /// Based on `ANDOR` example in ISO-10303-11
    const PERSON_ANDOR: &str = r#"
    SCHEMA test_schema;
      ENTITY person SUPERTYPE OF (employee ANDOR student);
      END_ENTITY;

      ENTITY employee SUBTYPE OF (person);
      END_ENTITY;

      ENTITY student SUBTYPE OF (person);
      END_ENTITY;
    END_SCHEMA;
    "#;

    /// Based on `AND` example in ISO-10303-11
    const PERSON_AND: &str = r#"
    SCHEMA test_schema;
      ENTITY person
        SUPERTYPE OF (
          ONEOF(male,female) AND ONEOF(citizen,alien)
        );
      END_ENTITY;

      ENTITY male SUBTYPE OF (person);
      END_ENTITY;

      ENTITY female SUBTYPE OF (person);
      END_ENTITY;

      ENTITY citizen SUBTYPE OF (person);
      END_ENTITY;

      ENTITY alien SUBTYPE OF (person);
      END_ENTITY;
    END_SCHEMA;
    "#;

    /// Based on default constraint example in ISO-10303-11
    const PERSON_DEFAULT: &str = r#"
    SCHEMA test_schema;
      ENTITY person;
      END_ENTITY;

      ENTITY employee SUBTYPE OF (person);
      END_ENTITY;

      ENTITY student SUBTYPE OF (person);
      END_ENTITY;
    END_SCHEMA;
    "#;

    /// Example for using `SUPERTYPE OF` declaration
    const SUPERTYPE_OF: &str = r#"
    SCHEMA test_schema;
      ENTITY base SUPERTYPE OF (ONEOF (sub1, sub2));
        x: REAL;
      END_ENTITY;

      ENTITY sub1 SUBTYPE OF (base);
        y1: REAL;
      END_ENTITY;

      ENTITY sub2 SUBTYPE OF (base);
        y2: REAL;
      END_ENTITY;
    END_SCHEMA;
    "#;

    #[test]
    fn gather_constraint_expr_oneof() {
        let st = ast::SyntaxTree::parse(PET).unwrap();
        let ns = Namespace::new(&st);
        let exprs = gather_constraint_expr(&ns, &st).unwrap();

        let scope = Scope::root().schema("test_schema");
        let pet = Path::entity(&scope, "pet");
        let cat = Path::entity(&scope, "cat");
        let rabbit = Path::entity(&scope, "rabbit");
        let dog = Path::entity(&scope, "dog");
        assert_eq!(
            dbg!(exprs),
            maplit::hashmap! {
                pet => ConstraintExpr::OneOf(vec![
                    ConstraintExpr::Reference(cat),
                    ConstraintExpr::Reference(rabbit),
                    ConstraintExpr::Reference(dog),
                ])
            }
        );
    }

    #[test]
    fn constraint_oneof() {
        let st = ast::SyntaxTree::parse(PET).unwrap();
        let ns = Namespace::new(&st);
        let c = Constraints::new(&ns, &st).unwrap();
        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "pet") => vec![
                        vec![Path::entity(&scope, "cat")],
                        vec![Path::entity(&scope, "rabbit")],
                        vec![Path::entity(&scope, "dog")],
                    ]
                }
            }
        );
    }

    #[test]
    fn supertype_of_oneof() {
        let st = ast::SyntaxTree::parse(SUPERTYPE_OF).unwrap();
        let ns = Namespace::new(&st);
        let c = Constraints::new(&ns, &st).unwrap();
        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "base") => vec![
                        vec![Path::entity(&scope, "sub1")],
                        vec![Path::entity(&scope, "sub2")],
                    ]
                }
            }
        );
    }

    #[test]
    fn supertype_of_andor() {
        let st = ast::SyntaxTree::parse(PERSON_ANDOR).unwrap();
        let ns = Namespace::new(&st);
        let c = Constraints::new(&ns, &st).unwrap();
        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "person") => vec![
                        vec![Path::entity(&scope, "employee")],
                        vec![Path::entity(&scope, "employee"), Path::entity(&scope, "student")],
                        vec![Path::entity(&scope, "student")],
                    ]
                }
            }
        );
    }

    #[test]
    fn supertype_of_and() {
        let st = ast::SyntaxTree::parse(PERSON_AND).unwrap();
        let ns = Namespace::new(&st);
        let c = Constraints::new(&ns, &st).unwrap();
        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "person") => vec![
                        vec![Path::entity(&scope, "male"), Path::entity(&scope, "citizen")],
                        vec![Path::entity(&scope, "male"), Path::entity(&scope, "alien")],
                        vec![Path::entity(&scope, "female"), Path::entity(&scope, "citizen")],
                        vec![Path::entity(&scope, "female"), Path::entity(&scope, "alien")],
                    ]
                }
            }
        );
    }

    #[test]
    fn default_constraint() {
        let st = ast::SyntaxTree::parse(PERSON_DEFAULT).unwrap();
        let ns = Namespace::new(&st);
        let c = Constraints::new(&ns, &st).unwrap();
        let scope = Scope::root().schema("test_schema");
        assert_eq!(
            dbg!(c),
            Constraints {
                instantiables: maplit::hashmap! {
                    Path::entity(&scope, "person") => vec![
                        vec![Path::entity(&scope, "employee")],
                        vec![Path::entity(&scope, "employee"), Path::entity(&scope, "student")],
                        vec![Path::entity(&scope, "student")],
                    ]
                }
            }
        );
    }
}

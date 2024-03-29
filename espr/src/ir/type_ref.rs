use super::{namespace::*, scope::*, *};
use crate::ast;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimpleType(pub ast::SimpleType);

impl Legalize for SimpleType {
    type Input = ast::SimpleType;
    fn legalize(
        _ns: &Namespace,
        _ss: &Constraints,
        _scope: &Scope,
        input: &Self::Input,
    ) -> Result<Self, SemanticError> {
        Ok(SimpleType(*input))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bound {}

impl Legalize for Bound {
    type Input = ast::Bound;
    fn legalize(
        _ns: &Namespace,
        _ss: &Constraints,
        _scope: &Scope,
        _input: &Self::Input,
    ) -> Result<Self, SemanticError> {
        // FIXME
        Ok(Bound {})
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeRef {
    SimpleType(SimpleType),
    Named {
        name: String,

        /// Scope where the named type is declared
        scope: Scope,

        /// True if the underlying type of named type is a simple type:
        ///
        /// ```text
        /// TYPE a = INTEGER; ENDTYPE;
        /// TYPE b = a; ENDTYPE;
        /// ```
        ///
        /// Then both `a` and `b` are simple.
        ///
        is_simple: bool,
        /// Enumeration, declared by `TYPE a = ENUMERATION OF (..); END_TYPE;`.
        is_enumerate: bool,
    },

    /* Declared as `ENTITY` */
    Entity {
        name: String,
        scope: Scope,
        is_supertype: bool,
    },

    /* Aggregated */
    Set {
        base: Box<TypeRef>,
        bound: Option<Bound>,
    },
    List {
        base: Box<TypeRef>,
        bound: Option<Bound>,
        unique: bool,
    },
}

impl TypeRef {
    /// Returns `true` iff `self` is:
    /// - a simple type,
    /// - a named type whose underlying type is simple, or,
    /// - a set or list of a type `x` such that `x.is_simple() == true`.
    pub fn is_simple(&self) -> bool {
        match self {
            TypeRef::SimpleType(..) => true,
            TypeRef::Named { is_simple, .. } => *is_simple,
            TypeRef::Set { base, .. } | TypeRef::List { base, .. } => base.is_simple(),
            _ => false,
        }
    }

    pub fn from_path(ns: &Namespace, ss: &Constraints, path: &Path) -> Result<Self, SemanticError> {
        match path.ty {
            ScopeType::Entity => {
                let is_supertype = ss.is_supertype(path);
                Ok(TypeRef::Entity {
                    name: path.name.clone(),
                    scope: path.scope.clone(),
                    is_supertype,
                })
            }
            ScopeType::Type => {
                let mut p = path.clone();
                let is_simple = loop {
                    match ns.get(&p)?.0 {
                        Named::Type(ast::TypeDecl {
                            underlying_type, ..
                        }) => match underlying_type {
                            // Enumeration e.g.
                            //
                            // ```
                            // TYPE null_style = ENUMERATION OF (null); END_TYPE;
                            // ```
                            //
                            // should be simple because it will be expressed as single integer.
                            ast::Type::Simple(_) | ast::Type::Enumeration { .. } => break true,
                            ast::Type::Named(name) => {
                                p = ns.resolve(&p.scope, name)?.0;
                                continue;
                            }
                            _ => break false,
                        },
                        Named::Entity(_) => break false,
                    }
                };
                let is_enumerate = match ns.get(path)?.0 {
                    Named::Type(ast::TypeDecl {
                        underlying_type, ..
                    }) => match underlying_type {
                        // Enumeration e.g.
                        //
                        // ```
                        // TYPE null_style = ENUMERATION OF (null); END_TYPE;
                        // ```
                        //
                        // should be simple because it will be expressed as single integer.
                        ast::Type::Simple(_) => false,
                        ast::Type::Enumeration { .. } => true,
                        _ => false,
                    },
                    Named::Entity(_) => false,
                };
                Ok(TypeRef::Named {
                    scope: path.scope.clone(),
                    name: path.name.clone(),
                    is_simple,
                    is_enumerate,
                })
            }
            _ => unimplemented!("Path to TypeRef conversion only supports Entity and Types yet."),
        }
    }
}

impl Legalize for TypeRef {
    type Input = ast::Type;

    fn legalize(
        ns: &Namespace,
        ss: &Constraints,
        scope: &Scope,
        ty: &ast::Type,
    ) -> Result<Self, SemanticError> {
        use ast::Type::*;
        Ok(match ty {
            Simple(ty) => Self::SimpleType(SimpleType(*ty)),
            Named(name) => {
                let (path, _index) = ns.resolve(scope, name)?;
                Self::from_path(ns, ss, &path)?
            }
            Set { base, bound } => {
                let base = TypeRef::legalize(ns, ss, scope, base.as_ref())?;
                let bound = if let Some(bound) = bound {
                    Some(Legalize::legalize(ns, ss, scope, bound)?)
                } else {
                    None
                };
                Self::Set {
                    base: Box::new(base),
                    bound,
                }
            }
            List {
                base,
                bound,
                unique,
            } => {
                let base = TypeRef::legalize(ns, ss, scope, base.as_ref())?;
                let bound = if let Some(bound) = bound {
                    Some(Legalize::legalize(ns, ss, scope, bound)?)
                } else {
                    None
                };
                Self::List {
                    base: Box::new(base),
                    bound,
                    unique: *unique,
                }
            }
            _ => todo!(),
        })
    }
}

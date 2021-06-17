use super::*;
use crate::ast;

/// Rename of primitive type,
/// e.g. `TYPE label = STRING; ENDTYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simple {
    pub id: String,
    pub ty: SimpleType,
}

/// Rename of user defined type,
/// e.g. `TYPE box_height = positive_ratio_measure; END_TYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rename {
    pub id: String,
    pub ty: TypeRef,
}

/// Enumeration of values,
/// e.g. `TYPE text_path = ENUMERATION OF (up, right, down, left); END_TYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumeration {
    pub id: String,
    pub items: Vec<String>,
}

/// Select of user defined types,
/// e.g. `TYPE geometric_set_select = SELECT (point, curve); END_TYPE;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select {
    pub id: String,
    pub types: Vec<TypeRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDecl {
    Simple(Simple),
    Rename(Rename),
    Enumeration(Enumeration),
    Select(Select),
}

impl Legalize for TypeDecl {
    type Input = ast::types::TypeDecl;
    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        type_decl: &Self::Input,
    ) -> Result<Self, SemanticError> {
        use ast::types::UnderlyingType;
        let id = type_decl.type_id.clone();
        Ok(match &type_decl.underlying_type {
            UnderlyingType::Simple(ty) => TypeDecl::Simple(Simple {
                id,
                ty: SimpleType(*ty),
            }),
            UnderlyingType::Reference(name) => {
                let ty = ns.lookup_type(scope, name)?;
                TypeDecl::Rename(Rename { id, ty })
            }
            UnderlyingType::Enumeration {
                items,
                extensibility: _,
            } => TypeDecl::Enumeration(Enumeration {
                id,
                items: items.clone(),
            }),
            UnderlyingType::Select {
                types,
                extensibility: _,
            } => {
                let types = types
                    .iter()
                    .map(|ty| ns.lookup_type(scope, ty))
                    .collect::<Result<Vec<_>, _>>()?;
                TypeDecl::Select(Select { id, types })
            }
            _ => panic!(),
        })
    }
}

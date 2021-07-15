use std::convert::{TryFrom, TryInto};

/// espr-generated field type
#[derive(Debug, Clone)]
pub enum FieldType {
    /// Like `T`
    Path(syn::Path),
    /// Like `Option<T>`
    Optional(Box<FieldType>),
    /// Like `Vec<T>`
    List(Box<FieldType>),
}

#[derive(Debug, Clone)]
pub struct UnsupportedTypeError {}

impl Into<syn::Type> for FieldType {
    fn into(self) -> syn::Type {
        let path = match self {
            FieldType::Path(path) => path,
            FieldType::Optional(ty) => {
                let ty: syn::Type = (*ty).into();
                syn::parse_quote! { Option<#ty> }
            }
            FieldType::List(ty) => {
                let ty: syn::Type = (*ty).into();
                syn::parse_quote! { Vec<#ty> }
            }
        };
        syn::Type::Path(syn::TypePath { qself: None, path })
    }
}

impl TryFrom<syn::Type> for FieldType {
    type Error = UnsupportedTypeError;

    fn try_from(ty: syn::Type) -> Result<Self, Self::Error> {
        let path = if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
            path
        } else {
            return Err(UnsupportedTypeError {});
        };

        let syn::Path { segments, .. } = &path;
        let last_seg = segments.last().unwrap();

        match &last_seg.arguments {
            syn::PathArguments::None => Ok(FieldType::Path(path.clone())),
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => {
                assert_eq!(args.len(), 1);
                if let syn::GenericArgument::Type(ty) = &args[0] {
                    let ty = Box::new(ty.clone().try_into()?);
                    if last_seg.ident == "Option" {
                        return Ok(FieldType::Optional(ty));
                    }
                    if last_seg.ident == "Vec" {
                        return Ok(FieldType::List(ty));
                    }
                    return Err(UnsupportedTypeError {});
                } else {
                    return Err(UnsupportedTypeError {});
                }
            }
            _ => Err(UnsupportedTypeError {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_type_convert() {
        let ty: syn::Type = syn::parse_str("T").unwrap();
        let f = ty.clone().try_into().unwrap();
        assert!(matches!(f, FieldType::Path(_)));
        assert_eq!(<FieldType as Into<syn::Type>>::into(f), ty);

        let ty: syn::Type = syn::parse_str("Option<T>").unwrap();
        let f = ty.clone().try_into().unwrap();
        assert!(matches!(f, FieldType::Optional(_)));
        assert_eq!(<FieldType as Into<syn::Type>>::into(f), ty);

        let ty: syn::Type = syn::parse_str("Vec<T>").unwrap();
        let f = ty.clone().try_into().unwrap();
        assert!(matches!(f, FieldType::List(_)));
        assert_eq!(<FieldType as Into<syn::Type>>::into(f), ty);

        let ty: syn::Type = syn::parse_str("Option<Vec<T>>").unwrap();
        let f = ty.clone().try_into().unwrap();
        if let FieldType::Optional(ty) = &f {
            assert!(matches!(**ty, FieldType::List(_)));
        } else {
            panic!()
        }
        assert_eq!(<FieldType as Into<syn::Type>>::into(f), ty);
    }
}

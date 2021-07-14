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

impl FieldType {
    pub fn new(ty: &syn::Type) -> Self {
        let path = if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
            path
        } else {
            panic!("non-path field is not supported")
        };

        let syn::Path { segments, .. } = &path;
        let last_seg = segments.last().unwrap();

        match &last_seg.arguments {
            syn::PathArguments::None => FieldType::Path(path.clone()),
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) => {
                assert_eq!(args.len(), 1);
                if let syn::GenericArgument::Type(ty) = &args[0] {
                    let ty = Box::new(FieldType::new(&ty));
                    if last_seg.ident == "Option" {
                        return FieldType::Optional(ty);
                    }
                    if last_seg.ident == "Vec" {
                        return FieldType::List(ty);
                    }
                    unreachable!("espr emits unexpected code")
                } else {
                    unreachable!("espr emits unexpected code")
                }
            }
            _ => unreachable!("espr emits unexpected code"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_type_new() {
        let ty: syn::Type = syn::parse_str("T").unwrap();
        let f = FieldType::new(&ty);
        assert!(matches!(f, FieldType::Path(_)));

        let ty: syn::Type = syn::parse_str("Option<T>").unwrap();
        let f = FieldType::new(&ty);
        assert!(matches!(f, FieldType::Optional(_)));

        let ty: syn::Type = syn::parse_str("Vec<T>").unwrap();
        let f = FieldType::new(&ty);
        assert!(matches!(f, FieldType::List(_)));

        let ty: syn::Type = syn::parse_str("Option<Vec<T>>").unwrap();
        let f = FieldType::new(&ty);
        if let FieldType::Optional(ty) = f {
            assert!(matches!(*ty, FieldType::List(_)));
        } else {
            panic!()
        }
    }
}

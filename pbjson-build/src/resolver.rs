use crate::descriptor::{Package, TypePath};

#[derive(Debug)]
pub struct Resolver<'a> {
    extern_types: &'a [(String, String)],
    package: &'a Package,
}

impl<'a> Resolver<'a> {
    /// Creates a new resolver for `package`
    pub fn new(extern_types: &'a [(String, String)], package: &'a Package) -> Self {
        Resolver {
            extern_types,
            package,
        }
    }

    /// Lookup an extern type, returns the rust type followed by the remaining
    /// number of path segments to skip
    fn resolve_extern(&self, path: &TypePath) -> Option<(String, usize)> {
        let mut match_len = 0_usize;
        let mut match_idx = None;

        for (idx, (proto_path, _)) in self.extern_types.iter().enumerate() {
            match path.prefix_match(proto_path) {
                Some(m) if m >= match_len => {
                    match_idx = Some(idx);
                    match_len = m;
                }
                _ => {}
            }
        }

        let (_, rust_path) = &self.extern_types[match_idx?];

        if match_len == path.len() {
            Some((rust_path.clone(), match_len))
        } else {
            Some((format!("{}::", rust_path), match_len))
        }
    }

    /// Returns the rust type for `path` relative to `cur`
    pub fn rust_type(&self, path: &TypePath) -> String {
        let (mut ret, skip) = match self.resolve_extern(path) {
            Some(x) => x,
            None => {
                // Compute the amount the resolver and path have in common
                let shared_prefix = self
                    .package
                    .path()
                    .iter()
                    .zip(path.path())
                    .filter(|(a, b)| a == b)
                    .count();

                let super_count = self.package.path().len() - shared_prefix;

                let mut ret = String::new();
                for _ in 0..super_count {
                    ret.push_str("super::")
                }

                (ret, shared_prefix)
            }
        };

        let mut iter = path.path().skip(skip).peekable();
        while let Some(i) = iter.next() {
            match iter.peek() {
                Some(_) => {
                    ret.push_str(i.to_snake_case().as_str());
                    ret.push_str("::");
                }
                None => {
                    ret.push_str(i.to_camel_case().as_str());
                }
            }
        }
        ret
    }

    pub fn rust_variant(&self, enumeration: &TypePath, variant: &str) -> String {
        use heck::CamelCase;
        assert!(
            variant
                .chars()
                .all(|c| matches!(c, '0'..='9' | 'A'..='Z' | '_')),
            "illegal variant - {}",
            variant
        );

        // TODO: Config to disable stripping prefix

        let enumeration_name = enumeration.path().last().unwrap().to_shouty_snake_case();
        let variant = match variant.strip_prefix(&enumeration_name) {
            Some("") => variant,
            Some(stripped) => stripped,
            None => variant,
        };
        variant.to_camel_case()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::TypeName;

    #[test]
    fn test_resolver() {
        let resolver_package = Package::new("test.syntax3");
        let extern_types = &[
            (".test.external".to_string(), "foo::common".to_string()),
            (".test.external.Boo".to_string(), "foo::common".to_string()),
            (
                ".test.external.Fiz.Buz".to_string(),
                "foo::bar::Buz".to_string(),
            ),
        ];
        let resolver = Resolver::new(extern_types, &resolver_package);

        // A type in the same package
        let same_type = TypePath::new(resolver_package.clone()).child(TypeName::new("Foo"));
        assert_eq!(resolver.rust_type(&same_type), "Foo");

        let nested_type = TypePath::new(resolver_package.clone())
            .child(TypeName::new("Foo"))
            .child(TypeName::new("Bar"));

        assert_eq!(resolver.rust_type(&nested_type), "foo::Bar");

        let other_package = Package::new("test.common");

        // A type in another package
        let other_type = TypePath::new(other_package.clone()).child(TypeName::new("Bar"));
        assert_eq!(resolver.rust_type(&other_type), "super::common::Bar");

        let other_nested_type = TypePath::new(other_package)
            .child(TypeName::new("Foo"))
            .child(TypeName::new("Bar"));

        assert_eq!(
            resolver.rust_type(&other_nested_type),
            "super::common::foo::Bar"
        );

        let external_package = Package::new("test.external");

        // An external type
        let external_type = TypePath::new(external_package.clone()).child(TypeName::new("Fiz"));
        assert_eq!(resolver.rust_type(&external_type), "foo::common::Fiz");

        // A nested external type
        let nested_external_type = external_type.child(TypeName::new("Bar"));

        assert_eq!(
            resolver.rust_type(&nested_external_type),
            "foo::common::fiz::Bar"
        );

        // An external type within an external type
        let external_nested_type = external_type.child(TypeName::new("Buz"));
        assert_eq!(resolver.rust_type(&external_nested_type), "foo::bar::Buz");

        // An external type with a different rust path length
        let external_nested_type = TypePath::new(external_package)
            .child(TypeName::new("Boo"))
            .child(TypeName::new("Bar"));
        assert_eq!(
            resolver.rust_type(&external_nested_type),
            "foo::common::Bar"
        );
    }
}

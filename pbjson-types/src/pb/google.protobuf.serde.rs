impl serde::Serialize for Any {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.Any", len)?;
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("typeUrl", &self.type_url)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Any {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type_url",
            "typeUrl",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypeUrl,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Any;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Any")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Any, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut type_url__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(Any {
                    type_url: type_url__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.Any", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Api {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.methods.is_empty() {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if self.source_context.is_some() {
            len += 1;
        }
        if !self.mixins.is_empty() {
            len += 1;
        }
        if self.syntax != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.Api", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.methods.is_empty() {
            struct_ser.serialize_field("methods", &self.methods)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.source_context.as_ref() {
            struct_ser.serialize_field("sourceContext", v)?;
        }
        if !self.mixins.is_empty() {
            struct_ser.serialize_field("mixins", &self.mixins)?;
        }
        if self.syntax != 0 {
            let v = Syntax::from_i32(self.syntax)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.syntax)))?;
            struct_ser.serialize_field("syntax", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Api {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "methods",
            "options",
            "version",
            "source_context",
            "sourceContext",
            "mixins",
            "syntax",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Methods,
            Options,
            Version,
            SourceContext,
            Mixins,
            Syntax,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "methods" => Ok(GeneratedField::Methods),
                            "options" => Ok(GeneratedField::Options),
                            "version" => Ok(GeneratedField::Version),
                            "sourceContext" | "source_context" => Ok(GeneratedField::SourceContext),
                            "mixins" => Ok(GeneratedField::Mixins),
                            "syntax" => Ok(GeneratedField::Syntax),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Api;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Api")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Api, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut methods__ = None;
                let mut options__ = None;
                let mut version__ = None;
                let mut source_context__ = None;
                let mut mixins__ = None;
                let mut syntax__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Methods => {
                            if methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methods"));
                            }
                            methods__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceContext => {
                            if source_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceContext"));
                            }
                            source_context__ = Some(map.next_value()?);
                        }
                        GeneratedField::Mixins => {
                            if mixins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mixins"));
                            }
                            mixins__ = Some(map.next_value()?);
                        }
                        GeneratedField::Syntax => {
                            if syntax__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syntax"));
                            }
                            syntax__ = Some(map.next_value::<Syntax>()? as i32);
                        }
                    }
                }
                Ok(Api {
                    name: name__.unwrap_or_default(),
                    methods: methods__.unwrap_or_default(),
                    options: options__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    source_context: source_context__,
                    mixins: mixins__.unwrap_or_default(),
                    syntax: syntax__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.Api", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DescriptorProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if !self.field.is_empty() {
            len += 1;
        }
        if !self.extension.is_empty() {
            len += 1;
        }
        if !self.nested_type.is_empty() {
            len += 1;
        }
        if !self.enum_type.is_empty() {
            len += 1;
        }
        if !self.extension_range.is_empty() {
            len += 1;
        }
        if !self.oneof_decl.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if !self.reserved_range.is_empty() {
            len += 1;
        }
        if !self.reserved_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.DescriptorProto", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if !self.extension.is_empty() {
            struct_ser.serialize_field("extension", &self.extension)?;
        }
        if !self.nested_type.is_empty() {
            struct_ser.serialize_field("nestedType", &self.nested_type)?;
        }
        if !self.enum_type.is_empty() {
            struct_ser.serialize_field("enumType", &self.enum_type)?;
        }
        if !self.extension_range.is_empty() {
            struct_ser.serialize_field("extensionRange", &self.extension_range)?;
        }
        if !self.oneof_decl.is_empty() {
            struct_ser.serialize_field("oneofDecl", &self.oneof_decl)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if !self.reserved_range.is_empty() {
            struct_ser.serialize_field("reservedRange", &self.reserved_range)?;
        }
        if !self.reserved_name.is_empty() {
            struct_ser.serialize_field("reservedName", &self.reserved_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DescriptorProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "field",
            "extension",
            "nested_type",
            "nestedType",
            "enum_type",
            "enumType",
            "extension_range",
            "extensionRange",
            "oneof_decl",
            "oneofDecl",
            "options",
            "reserved_range",
            "reservedRange",
            "reserved_name",
            "reservedName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Field,
            Extension,
            NestedType,
            EnumType,
            ExtensionRange,
            OneofDecl,
            Options,
            ReservedRange,
            ReservedName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "field" => Ok(GeneratedField::Field),
                            "extension" => Ok(GeneratedField::Extension),
                            "nestedType" | "nested_type" => Ok(GeneratedField::NestedType),
                            "enumType" | "enum_type" => Ok(GeneratedField::EnumType),
                            "extensionRange" | "extension_range" => Ok(GeneratedField::ExtensionRange),
                            "oneofDecl" | "oneof_decl" => Ok(GeneratedField::OneofDecl),
                            "options" => Ok(GeneratedField::Options),
                            "reservedRange" | "reserved_range" => Ok(GeneratedField::ReservedRange),
                            "reservedName" | "reserved_name" => Ok(GeneratedField::ReservedName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DescriptorProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.DescriptorProto")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DescriptorProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut field__ = None;
                let mut extension__ = None;
                let mut nested_type__ = None;
                let mut enum_type__ = None;
                let mut extension_range__ = None;
                let mut oneof_decl__ = None;
                let mut options__ = None;
                let mut reserved_range__ = None;
                let mut reserved_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map.next_value()?);
                        }
                        GeneratedField::Extension => {
                            if extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            extension__ = Some(map.next_value()?);
                        }
                        GeneratedField::NestedType => {
                            if nested_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nestedType"));
                            }
                            nested_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnumType => {
                            if enum_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumType"));
                            }
                            enum_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExtensionRange => {
                            if extension_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionRange"));
                            }
                            extension_range__ = Some(map.next_value()?);
                        }
                        GeneratedField::OneofDecl => {
                            if oneof_decl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oneofDecl"));
                            }
                            oneof_decl__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReservedRange => {
                            if reserved_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservedRange"));
                            }
                            reserved_range__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReservedName => {
                            if reserved_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservedName"));
                            }
                            reserved_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DescriptorProto {
                    name: name__,
                    field: field__.unwrap_or_default(),
                    extension: extension__.unwrap_or_default(),
                    nested_type: nested_type__.unwrap_or_default(),
                    enum_type: enum_type__.unwrap_or_default(),
                    extension_range: extension_range__.unwrap_or_default(),
                    oneof_decl: oneof_decl__.unwrap_or_default(),
                    options: options__,
                    reserved_range: reserved_range__.unwrap_or_default(),
                    reserved_name: reserved_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.DescriptorProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for descriptor_proto::ExtensionRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.DescriptorProto.ExtensionRange", len)?;
        if let Some(v) = self.start.as_ref() {
            struct_ser.serialize_field("start", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for descriptor_proto::ExtensionRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
            Options,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = descriptor_proto::ExtensionRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.DescriptorProto.ExtensionRange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<descriptor_proto::ExtensionRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(descriptor_proto::ExtensionRange {
                    start: start__,
                    end: end__,
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.DescriptorProto.ExtensionRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for descriptor_proto::ReservedRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.DescriptorProto.ReservedRange", len)?;
        if let Some(v) = self.start.as_ref() {
            struct_ser.serialize_field("start", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for descriptor_proto::ReservedRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = descriptor_proto::ReservedRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.DescriptorProto.ReservedRange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<descriptor_proto::ReservedRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(descriptor_proto::ReservedRange {
                    start: start__,
                    end: end__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.DescriptorProto.ReservedRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Empty {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("google.protobuf.Empty", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Empty {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Empty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Empty")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Empty, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Empty {
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.Empty", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Enum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.enumvalue.is_empty() {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        if self.source_context.is_some() {
            len += 1;
        }
        if self.syntax != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.Enum", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.enumvalue.is_empty() {
            struct_ser.serialize_field("enumvalue", &self.enumvalue)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        if let Some(v) = self.source_context.as_ref() {
            struct_ser.serialize_field("sourceContext", v)?;
        }
        if self.syntax != 0 {
            let v = Syntax::from_i32(self.syntax)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.syntax)))?;
            struct_ser.serialize_field("syntax", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Enum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "enumvalue",
            "options",
            "source_context",
            "sourceContext",
            "syntax",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Enumvalue,
            Options,
            SourceContext,
            Syntax,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "enumvalue" => Ok(GeneratedField::Enumvalue),
                            "options" => Ok(GeneratedField::Options),
                            "sourceContext" | "source_context" => Ok(GeneratedField::SourceContext),
                            "syntax" => Ok(GeneratedField::Syntax),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Enum;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Enum")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Enum, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut enumvalue__ = None;
                let mut options__ = None;
                let mut source_context__ = None;
                let mut syntax__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Enumvalue => {
                            if enumvalue__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumvalue"));
                            }
                            enumvalue__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceContext => {
                            if source_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceContext"));
                            }
                            source_context__ = Some(map.next_value()?);
                        }
                        GeneratedField::Syntax => {
                            if syntax__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syntax"));
                            }
                            syntax__ = Some(map.next_value::<Syntax>()? as i32);
                        }
                    }
                }
                Ok(Enum {
                    name: name__.unwrap_or_default(),
                    enumvalue: enumvalue__.unwrap_or_default(),
                    options: options__.unwrap_or_default(),
                    source_context: source_context__,
                    syntax: syntax__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.Enum", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumDescriptorProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if !self.reserved_range.is_empty() {
            len += 1;
        }
        if !self.reserved_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.EnumDescriptorProto", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if !self.reserved_range.is_empty() {
            struct_ser.serialize_field("reservedRange", &self.reserved_range)?;
        }
        if !self.reserved_name.is_empty() {
            struct_ser.serialize_field("reservedName", &self.reserved_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumDescriptorProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
            "options",
            "reserved_range",
            "reservedRange",
            "reserved_name",
            "reservedName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
            Options,
            ReservedRange,
            ReservedName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "value" => Ok(GeneratedField::Value),
                            "options" => Ok(GeneratedField::Options),
                            "reservedRange" | "reserved_range" => Ok(GeneratedField::ReservedRange),
                            "reservedName" | "reserved_name" => Ok(GeneratedField::ReservedName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumDescriptorProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.EnumDescriptorProto")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnumDescriptorProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                let mut options__ = None;
                let mut reserved_range__ = None;
                let mut reserved_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReservedRange => {
                            if reserved_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservedRange"));
                            }
                            reserved_range__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReservedName => {
                            if reserved_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservedName"));
                            }
                            reserved_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EnumDescriptorProto {
                    name: name__,
                    value: value__.unwrap_or_default(),
                    options: options__,
                    reserved_range: reserved_range__.unwrap_or_default(),
                    reserved_name: reserved_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.EnumDescriptorProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for enum_descriptor_proto::EnumReservedRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.EnumDescriptorProto.EnumReservedRange", len)?;
        if let Some(v) = self.start.as_ref() {
            struct_ser.serialize_field("start", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for enum_descriptor_proto::EnumReservedRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = enum_descriptor_proto::EnumReservedRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.EnumDescriptorProto.EnumReservedRange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<enum_descriptor_proto::EnumReservedRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(enum_descriptor_proto::EnumReservedRange {
                    start: start__,
                    end: end__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.EnumDescriptorProto.EnumReservedRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allow_alias.is_some() {
            len += 1;
        }
        if self.deprecated.is_some() {
            len += 1;
        }
        if !self.uninterpreted_option.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.EnumOptions", len)?;
        if let Some(v) = self.allow_alias.as_ref() {
            struct_ser.serialize_field("allowAlias", v)?;
        }
        if let Some(v) = self.deprecated.as_ref() {
            struct_ser.serialize_field("deprecated", v)?;
        }
        if !self.uninterpreted_option.is_empty() {
            struct_ser.serialize_field("uninterpretedOption", &self.uninterpreted_option)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allow_alias",
            "allowAlias",
            "deprecated",
            "uninterpreted_option",
            "uninterpretedOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowAlias,
            Deprecated,
            UninterpretedOption,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "allowAlias" | "allow_alias" => Ok(GeneratedField::AllowAlias),
                            "deprecated" => Ok(GeneratedField::Deprecated),
                            "uninterpretedOption" | "uninterpreted_option" => Ok(GeneratedField::UninterpretedOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.EnumOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnumOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allow_alias__ = None;
                let mut deprecated__ = None;
                let mut uninterpreted_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowAlias => {
                            if allow_alias__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowAlias"));
                            }
                            allow_alias__ = Some(map.next_value()?);
                        }
                        GeneratedField::Deprecated => {
                            if deprecated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecated"));
                            }
                            deprecated__ = Some(map.next_value()?);
                        }
                        GeneratedField::UninterpretedOption => {
                            if uninterpreted_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uninterpretedOption"));
                            }
                            uninterpreted_option__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EnumOptions {
                    allow_alias: allow_alias__,
                    deprecated: deprecated__,
                    uninterpreted_option: uninterpreted_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.EnumOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.number != 0 {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.EnumValue", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.number != 0 {
            struct_ser.serialize_field("number", &self.number)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "number",
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Number,
            Options,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "number" => Ok(GeneratedField::Number),
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.EnumValue")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnumValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut number__ = None;
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EnumValue {
                    name: name__.unwrap_or_default(),
                    number: number__.unwrap_or_default(),
                    options: options__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.EnumValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumValueDescriptorProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.number.is_some() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.EnumValueDescriptorProto", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.number.as_ref() {
            struct_ser.serialize_field("number", v)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumValueDescriptorProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "number",
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Number,
            Options,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "number" => Ok(GeneratedField::Number),
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumValueDescriptorProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.EnumValueDescriptorProto")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnumValueDescriptorProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut number__ = None;
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EnumValueDescriptorProto {
                    name: name__,
                    number: number__,
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.EnumValueDescriptorProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumValueOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deprecated.is_some() {
            len += 1;
        }
        if !self.uninterpreted_option.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.EnumValueOptions", len)?;
        if let Some(v) = self.deprecated.as_ref() {
            struct_ser.serialize_field("deprecated", v)?;
        }
        if !self.uninterpreted_option.is_empty() {
            struct_ser.serialize_field("uninterpretedOption", &self.uninterpreted_option)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumValueOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deprecated",
            "uninterpreted_option",
            "uninterpretedOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deprecated,
            UninterpretedOption,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "deprecated" => Ok(GeneratedField::Deprecated),
                            "uninterpretedOption" | "uninterpreted_option" => Ok(GeneratedField::UninterpretedOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumValueOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.EnumValueOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnumValueOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deprecated__ = None;
                let mut uninterpreted_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deprecated => {
                            if deprecated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecated"));
                            }
                            deprecated__ = Some(map.next_value()?);
                        }
                        GeneratedField::UninterpretedOption => {
                            if uninterpreted_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uninterpretedOption"));
                            }
                            uninterpreted_option__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EnumValueOptions {
                    deprecated: deprecated__,
                    uninterpreted_option: uninterpreted_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.EnumValueOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtensionRangeOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uninterpreted_option.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.ExtensionRangeOptions", len)?;
        if !self.uninterpreted_option.is_empty() {
            struct_ser.serialize_field("uninterpretedOption", &self.uninterpreted_option)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtensionRangeOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uninterpreted_option",
            "uninterpretedOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UninterpretedOption,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uninterpretedOption" | "uninterpreted_option" => Ok(GeneratedField::UninterpretedOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtensionRangeOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.ExtensionRangeOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtensionRangeOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uninterpreted_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UninterpretedOption => {
                            if uninterpreted_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uninterpretedOption"));
                            }
                            uninterpreted_option__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExtensionRangeOptions {
                    uninterpreted_option: uninterpreted_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.ExtensionRangeOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Field {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind != 0 {
            len += 1;
        }
        if self.cardinality != 0 {
            len += 1;
        }
        if self.number != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.type_url.is_empty() {
            len += 1;
        }
        if self.oneof_index != 0 {
            len += 1;
        }
        if self.packed {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        if !self.json_name.is_empty() {
            len += 1;
        }
        if !self.default_value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.Field", len)?;
        if self.kind != 0 {
            let v = field::Kind::from_i32(self.kind)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if self.cardinality != 0 {
            let v = field::Cardinality::from_i32(self.cardinality)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.cardinality)))?;
            struct_ser.serialize_field("cardinality", &v)?;
        }
        if self.number != 0 {
            struct_ser.serialize_field("number", &self.number)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("typeUrl", &self.type_url)?;
        }
        if self.oneof_index != 0 {
            struct_ser.serialize_field("oneofIndex", &self.oneof_index)?;
        }
        if self.packed {
            struct_ser.serialize_field("packed", &self.packed)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        if !self.json_name.is_empty() {
            struct_ser.serialize_field("jsonName", &self.json_name)?;
        }
        if !self.default_value.is_empty() {
            struct_ser.serialize_field("defaultValue", &self.default_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Field {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "cardinality",
            "number",
            "name",
            "type_url",
            "typeUrl",
            "oneof_index",
            "oneofIndex",
            "packed",
            "options",
            "json_name",
            "jsonName",
            "default_value",
            "defaultValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Cardinality,
            Number,
            Name,
            TypeUrl,
            OneofIndex,
            Packed,
            Options,
            JsonName,
            DefaultValue,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "cardinality" => Ok(GeneratedField::Cardinality),
                            "number" => Ok(GeneratedField::Number),
                            "name" => Ok(GeneratedField::Name),
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "oneofIndex" | "oneof_index" => Ok(GeneratedField::OneofIndex),
                            "packed" => Ok(GeneratedField::Packed),
                            "options" => Ok(GeneratedField::Options),
                            "jsonName" | "json_name" => Ok(GeneratedField::JsonName),
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Field;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Field")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Field, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut cardinality__ = None;
                let mut number__ = None;
                let mut name__ = None;
                let mut type_url__ = None;
                let mut oneof_index__ = None;
                let mut packed__ = None;
                let mut options__ = None;
                let mut json_name__ = None;
                let mut default_value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map.next_value::<field::Kind>()? as i32);
                        }
                        GeneratedField::Cardinality => {
                            if cardinality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardinality"));
                            }
                            cardinality__ = Some(map.next_value::<field::Cardinality>()? as i32);
                        }
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::OneofIndex => {
                            if oneof_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oneofIndex"));
                            }
                            oneof_index__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Packed => {
                            if packed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packed"));
                            }
                            packed__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::JsonName => {
                            if json_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonName"));
                            }
                            json_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Field {
                    kind: kind__.unwrap_or_default(),
                    cardinality: cardinality__.unwrap_or_default(),
                    number: number__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    type_url: type_url__.unwrap_or_default(),
                    oneof_index: oneof_index__.unwrap_or_default(),
                    packed: packed__.unwrap_or_default(),
                    options: options__.unwrap_or_default(),
                    json_name: json_name__.unwrap_or_default(),
                    default_value: default_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.Field", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for field::Cardinality {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "CARDINALITY_UNKNOWN",
            Self::Optional => "CARDINALITY_OPTIONAL",
            Self::Required => "CARDINALITY_REQUIRED",
            Self::Repeated => "CARDINALITY_REPEATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for field::Cardinality {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CARDINALITY_UNKNOWN",
            "CARDINALITY_OPTIONAL",
            "CARDINALITY_REQUIRED",
            "CARDINALITY_REPEATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field::Cardinality;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field::Cardinality::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field::Cardinality::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CARDINALITY_UNKNOWN" => Ok(field::Cardinality::Unknown),
                    "CARDINALITY_OPTIONAL" => Ok(field::Cardinality::Optional),
                    "CARDINALITY_REQUIRED" => Ok(field::Cardinality::Required),
                    "CARDINALITY_REPEATED" => Ok(field::Cardinality::Repeated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for field::Kind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::TypeUnknown => "TYPE_UNKNOWN",
            Self::TypeDouble => "TYPE_DOUBLE",
            Self::TypeFloat => "TYPE_FLOAT",
            Self::TypeInt64 => "TYPE_INT64",
            Self::TypeUint64 => "TYPE_UINT64",
            Self::TypeInt32 => "TYPE_INT32",
            Self::TypeFixed64 => "TYPE_FIXED64",
            Self::TypeFixed32 => "TYPE_FIXED32",
            Self::TypeBool => "TYPE_BOOL",
            Self::TypeString => "TYPE_STRING",
            Self::TypeGroup => "TYPE_GROUP",
            Self::TypeMessage => "TYPE_MESSAGE",
            Self::TypeBytes => "TYPE_BYTES",
            Self::TypeUint32 => "TYPE_UINT32",
            Self::TypeEnum => "TYPE_ENUM",
            Self::TypeSfixed32 => "TYPE_SFIXED32",
            Self::TypeSfixed64 => "TYPE_SFIXED64",
            Self::TypeSint32 => "TYPE_SINT32",
            Self::TypeSint64 => "TYPE_SINT64",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for field::Kind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TYPE_UNKNOWN",
            "TYPE_DOUBLE",
            "TYPE_FLOAT",
            "TYPE_INT64",
            "TYPE_UINT64",
            "TYPE_INT32",
            "TYPE_FIXED64",
            "TYPE_FIXED32",
            "TYPE_BOOL",
            "TYPE_STRING",
            "TYPE_GROUP",
            "TYPE_MESSAGE",
            "TYPE_BYTES",
            "TYPE_UINT32",
            "TYPE_ENUM",
            "TYPE_SFIXED32",
            "TYPE_SFIXED64",
            "TYPE_SINT32",
            "TYPE_SINT64",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field::Kind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field::Kind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field::Kind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TYPE_UNKNOWN" => Ok(field::Kind::TypeUnknown),
                    "TYPE_DOUBLE" => Ok(field::Kind::TypeDouble),
                    "TYPE_FLOAT" => Ok(field::Kind::TypeFloat),
                    "TYPE_INT64" => Ok(field::Kind::TypeInt64),
                    "TYPE_UINT64" => Ok(field::Kind::TypeUint64),
                    "TYPE_INT32" => Ok(field::Kind::TypeInt32),
                    "TYPE_FIXED64" => Ok(field::Kind::TypeFixed64),
                    "TYPE_FIXED32" => Ok(field::Kind::TypeFixed32),
                    "TYPE_BOOL" => Ok(field::Kind::TypeBool),
                    "TYPE_STRING" => Ok(field::Kind::TypeString),
                    "TYPE_GROUP" => Ok(field::Kind::TypeGroup),
                    "TYPE_MESSAGE" => Ok(field::Kind::TypeMessage),
                    "TYPE_BYTES" => Ok(field::Kind::TypeBytes),
                    "TYPE_UINT32" => Ok(field::Kind::TypeUint32),
                    "TYPE_ENUM" => Ok(field::Kind::TypeEnum),
                    "TYPE_SFIXED32" => Ok(field::Kind::TypeSfixed32),
                    "TYPE_SFIXED64" => Ok(field::Kind::TypeSfixed64),
                    "TYPE_SINT32" => Ok(field::Kind::TypeSint32),
                    "TYPE_SINT64" => Ok(field::Kind::TypeSint64),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FieldDescriptorProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.number.is_some() {
            len += 1;
        }
        if self.label.is_some() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        if self.type_name.is_some() {
            len += 1;
        }
        if self.extendee.is_some() {
            len += 1;
        }
        if self.default_value.is_some() {
            len += 1;
        }
        if self.oneof_index.is_some() {
            len += 1;
        }
        if self.json_name.is_some() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if self.proto3_optional.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.FieldDescriptorProto", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.number.as_ref() {
            struct_ser.serialize_field("number", v)?;
        }
        if let Some(v) = self.label.as_ref() {
            let v = field_descriptor_proto::Label::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("label", &v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            let v = field_descriptor_proto::Type::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.type_name.as_ref() {
            struct_ser.serialize_field("typeName", v)?;
        }
        if let Some(v) = self.extendee.as_ref() {
            struct_ser.serialize_field("extendee", v)?;
        }
        if let Some(v) = self.default_value.as_ref() {
            struct_ser.serialize_field("defaultValue", v)?;
        }
        if let Some(v) = self.oneof_index.as_ref() {
            struct_ser.serialize_field("oneofIndex", v)?;
        }
        if let Some(v) = self.json_name.as_ref() {
            struct_ser.serialize_field("jsonName", v)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if let Some(v) = self.proto3_optional.as_ref() {
            struct_ser.serialize_field("proto3Optional", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldDescriptorProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "number",
            "label",
            "type",
            "type_name",
            "typeName",
            "extendee",
            "default_value",
            "defaultValue",
            "oneof_index",
            "oneofIndex",
            "json_name",
            "jsonName",
            "options",
            "proto3_optional",
            "proto3Optional",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Number,
            Label,
            Type,
            TypeName,
            Extendee,
            DefaultValue,
            OneofIndex,
            JsonName,
            Options,
            Proto3Optional,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "number" => Ok(GeneratedField::Number),
                            "label" => Ok(GeneratedField::Label),
                            "type" => Ok(GeneratedField::Type),
                            "typeName" | "type_name" => Ok(GeneratedField::TypeName),
                            "extendee" => Ok(GeneratedField::Extendee),
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            "oneofIndex" | "oneof_index" => Ok(GeneratedField::OneofIndex),
                            "jsonName" | "json_name" => Ok(GeneratedField::JsonName),
                            "options" => Ok(GeneratedField::Options),
                            "proto3Optional" | "proto3_optional" => Ok(GeneratedField::Proto3Optional),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldDescriptorProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.FieldDescriptorProto")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FieldDescriptorProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut number__ = None;
                let mut label__ = None;
                let mut r#type__ = None;
                let mut type_name__ = None;
                let mut extendee__ = None;
                let mut default_value__ = None;
                let mut oneof_index__ = None;
                let mut json_name__ = None;
                let mut options__ = None;
                let mut proto3_optional__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Label => {
                            if label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("label"));
                            }
                            label__ = Some(map.next_value::<field_descriptor_proto::Label>()? as i32);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<field_descriptor_proto::Type>()? as i32);
                        }
                        GeneratedField::TypeName => {
                            if type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeName"));
                            }
                            type_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Extendee => {
                            if extendee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extendee"));
                            }
                            extendee__ = Some(map.next_value()?);
                        }
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::OneofIndex => {
                            if oneof_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oneofIndex"));
                            }
                            oneof_index__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::JsonName => {
                            if json_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonName"));
                            }
                            json_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::Proto3Optional => {
                            if proto3_optional__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proto3Optional"));
                            }
                            proto3_optional__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FieldDescriptorProto {
                    name: name__,
                    number: number__,
                    label: label__,
                    r#type: r#type__,
                    type_name: type_name__,
                    extendee: extendee__,
                    default_value: default_value__,
                    oneof_index: oneof_index__,
                    json_name: json_name__,
                    options: options__,
                    proto3_optional: proto3_optional__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.FieldDescriptorProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for field_descriptor_proto::Label {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Optional => "LABEL_OPTIONAL",
            Self::Required => "LABEL_REQUIRED",
            Self::Repeated => "LABEL_REPEATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for field_descriptor_proto::Label {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LABEL_OPTIONAL",
            "LABEL_REQUIRED",
            "LABEL_REPEATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field_descriptor_proto::Label;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field_descriptor_proto::Label::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field_descriptor_proto::Label::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "LABEL_OPTIONAL" => Ok(field_descriptor_proto::Label::Optional),
                    "LABEL_REQUIRED" => Ok(field_descriptor_proto::Label::Required),
                    "LABEL_REPEATED" => Ok(field_descriptor_proto::Label::Repeated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for field_descriptor_proto::Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Double => "TYPE_DOUBLE",
            Self::Float => "TYPE_FLOAT",
            Self::Int64 => "TYPE_INT64",
            Self::Uint64 => "TYPE_UINT64",
            Self::Int32 => "TYPE_INT32",
            Self::Fixed64 => "TYPE_FIXED64",
            Self::Fixed32 => "TYPE_FIXED32",
            Self::Bool => "TYPE_BOOL",
            Self::String => "TYPE_STRING",
            Self::Group => "TYPE_GROUP",
            Self::Message => "TYPE_MESSAGE",
            Self::Bytes => "TYPE_BYTES",
            Self::Uint32 => "TYPE_UINT32",
            Self::Enum => "TYPE_ENUM",
            Self::Sfixed32 => "TYPE_SFIXED32",
            Self::Sfixed64 => "TYPE_SFIXED64",
            Self::Sint32 => "TYPE_SINT32",
            Self::Sint64 => "TYPE_SINT64",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for field_descriptor_proto::Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TYPE_DOUBLE",
            "TYPE_FLOAT",
            "TYPE_INT64",
            "TYPE_UINT64",
            "TYPE_INT32",
            "TYPE_FIXED64",
            "TYPE_FIXED32",
            "TYPE_BOOL",
            "TYPE_STRING",
            "TYPE_GROUP",
            "TYPE_MESSAGE",
            "TYPE_BYTES",
            "TYPE_UINT32",
            "TYPE_ENUM",
            "TYPE_SFIXED32",
            "TYPE_SFIXED64",
            "TYPE_SINT32",
            "TYPE_SINT64",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field_descriptor_proto::Type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field_descriptor_proto::Type::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field_descriptor_proto::Type::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TYPE_DOUBLE" => Ok(field_descriptor_proto::Type::Double),
                    "TYPE_FLOAT" => Ok(field_descriptor_proto::Type::Float),
                    "TYPE_INT64" => Ok(field_descriptor_proto::Type::Int64),
                    "TYPE_UINT64" => Ok(field_descriptor_proto::Type::Uint64),
                    "TYPE_INT32" => Ok(field_descriptor_proto::Type::Int32),
                    "TYPE_FIXED64" => Ok(field_descriptor_proto::Type::Fixed64),
                    "TYPE_FIXED32" => Ok(field_descriptor_proto::Type::Fixed32),
                    "TYPE_BOOL" => Ok(field_descriptor_proto::Type::Bool),
                    "TYPE_STRING" => Ok(field_descriptor_proto::Type::String),
                    "TYPE_GROUP" => Ok(field_descriptor_proto::Type::Group),
                    "TYPE_MESSAGE" => Ok(field_descriptor_proto::Type::Message),
                    "TYPE_BYTES" => Ok(field_descriptor_proto::Type::Bytes),
                    "TYPE_UINT32" => Ok(field_descriptor_proto::Type::Uint32),
                    "TYPE_ENUM" => Ok(field_descriptor_proto::Type::Enum),
                    "TYPE_SFIXED32" => Ok(field_descriptor_proto::Type::Sfixed32),
                    "TYPE_SFIXED64" => Ok(field_descriptor_proto::Type::Sfixed64),
                    "TYPE_SINT32" => Ok(field_descriptor_proto::Type::Sint32),
                    "TYPE_SINT64" => Ok(field_descriptor_proto::Type::Sint64),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FieldMask {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.paths.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.FieldMask", len)?;
        if !self.paths.is_empty() {
            struct_ser.serialize_field("paths", &self.paths)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldMask {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "paths",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Paths,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "paths" => Ok(GeneratedField::Paths),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldMask;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.FieldMask")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FieldMask, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut paths__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Paths => {
                            if paths__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paths"));
                            }
                            paths__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FieldMask {
                    paths: paths__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.FieldMask", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ctype.is_some() {
            len += 1;
        }
        if self.packed.is_some() {
            len += 1;
        }
        if self.jstype.is_some() {
            len += 1;
        }
        if self.lazy.is_some() {
            len += 1;
        }
        if self.deprecated.is_some() {
            len += 1;
        }
        if self.weak.is_some() {
            len += 1;
        }
        if !self.uninterpreted_option.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.FieldOptions", len)?;
        if let Some(v) = self.ctype.as_ref() {
            let v = field_options::CType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("ctype", &v)?;
        }
        if let Some(v) = self.packed.as_ref() {
            struct_ser.serialize_field("packed", v)?;
        }
        if let Some(v) = self.jstype.as_ref() {
            let v = field_options::JsType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("jstype", &v)?;
        }
        if let Some(v) = self.lazy.as_ref() {
            struct_ser.serialize_field("lazy", v)?;
        }
        if let Some(v) = self.deprecated.as_ref() {
            struct_ser.serialize_field("deprecated", v)?;
        }
        if let Some(v) = self.weak.as_ref() {
            struct_ser.serialize_field("weak", v)?;
        }
        if !self.uninterpreted_option.is_empty() {
            struct_ser.serialize_field("uninterpretedOption", &self.uninterpreted_option)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ctype",
            "packed",
            "jstype",
            "lazy",
            "deprecated",
            "weak",
            "uninterpreted_option",
            "uninterpretedOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ctype,
            Packed,
            Jstype,
            Lazy,
            Deprecated,
            Weak,
            UninterpretedOption,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ctype" => Ok(GeneratedField::Ctype),
                            "packed" => Ok(GeneratedField::Packed),
                            "jstype" => Ok(GeneratedField::Jstype),
                            "lazy" => Ok(GeneratedField::Lazy),
                            "deprecated" => Ok(GeneratedField::Deprecated),
                            "weak" => Ok(GeneratedField::Weak),
                            "uninterpretedOption" | "uninterpreted_option" => Ok(GeneratedField::UninterpretedOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.FieldOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FieldOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ctype__ = None;
                let mut packed__ = None;
                let mut jstype__ = None;
                let mut lazy__ = None;
                let mut deprecated__ = None;
                let mut weak__ = None;
                let mut uninterpreted_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ctype => {
                            if ctype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctype"));
                            }
                            ctype__ = Some(map.next_value::<field_options::CType>()? as i32);
                        }
                        GeneratedField::Packed => {
                            if packed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packed"));
                            }
                            packed__ = Some(map.next_value()?);
                        }
                        GeneratedField::Jstype => {
                            if jstype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jstype"));
                            }
                            jstype__ = Some(map.next_value::<field_options::JsType>()? as i32);
                        }
                        GeneratedField::Lazy => {
                            if lazy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lazy"));
                            }
                            lazy__ = Some(map.next_value()?);
                        }
                        GeneratedField::Deprecated => {
                            if deprecated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecated"));
                            }
                            deprecated__ = Some(map.next_value()?);
                        }
                        GeneratedField::Weak => {
                            if weak__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weak"));
                            }
                            weak__ = Some(map.next_value()?);
                        }
                        GeneratedField::UninterpretedOption => {
                            if uninterpreted_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uninterpretedOption"));
                            }
                            uninterpreted_option__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FieldOptions {
                    ctype: ctype__,
                    packed: packed__,
                    jstype: jstype__,
                    lazy: lazy__,
                    deprecated: deprecated__,
                    weak: weak__,
                    uninterpreted_option: uninterpreted_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.FieldOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for field_options::CType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::String => "STRING",
            Self::Cord => "CORD",
            Self::StringPiece => "STRING_PIECE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for field_options::CType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STRING",
            "CORD",
            "STRING_PIECE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field_options::CType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field_options::CType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field_options::CType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STRING" => Ok(field_options::CType::String),
                    "CORD" => Ok(field_options::CType::Cord),
                    "STRING_PIECE" => Ok(field_options::CType::StringPiece),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for field_options::JsType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::JsNormal => "JS_NORMAL",
            Self::JsString => "JS_STRING",
            Self::JsNumber => "JS_NUMBER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for field_options::JsType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "JS_NORMAL",
            "JS_STRING",
            "JS_NUMBER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = field_options::JsType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field_options::JsType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(field_options::JsType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "JS_NORMAL" => Ok(field_options::JsType::JsNormal),
                    "JS_STRING" => Ok(field_options::JsType::JsString),
                    "JS_NUMBER" => Ok(field_options::JsType::JsNumber),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FileDescriptorProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.package.is_some() {
            len += 1;
        }
        if !self.dependency.is_empty() {
            len += 1;
        }
        if !self.public_dependency.is_empty() {
            len += 1;
        }
        if !self.weak_dependency.is_empty() {
            len += 1;
        }
        if !self.message_type.is_empty() {
            len += 1;
        }
        if !self.enum_type.is_empty() {
            len += 1;
        }
        if !self.service.is_empty() {
            len += 1;
        }
        if !self.extension.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if self.source_code_info.is_some() {
            len += 1;
        }
        if self.syntax.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.FileDescriptorProto", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.package.as_ref() {
            struct_ser.serialize_field("package", v)?;
        }
        if !self.dependency.is_empty() {
            struct_ser.serialize_field("dependency", &self.dependency)?;
        }
        if !self.public_dependency.is_empty() {
            struct_ser.serialize_field("publicDependency", &self.public_dependency)?;
        }
        if !self.weak_dependency.is_empty() {
            struct_ser.serialize_field("weakDependency", &self.weak_dependency)?;
        }
        if !self.message_type.is_empty() {
            struct_ser.serialize_field("messageType", &self.message_type)?;
        }
        if !self.enum_type.is_empty() {
            struct_ser.serialize_field("enumType", &self.enum_type)?;
        }
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        if !self.extension.is_empty() {
            struct_ser.serialize_field("extension", &self.extension)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if let Some(v) = self.source_code_info.as_ref() {
            struct_ser.serialize_field("sourceCodeInfo", v)?;
        }
        if let Some(v) = self.syntax.as_ref() {
            struct_ser.serialize_field("syntax", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileDescriptorProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "package",
            "dependency",
            "public_dependency",
            "publicDependency",
            "weak_dependency",
            "weakDependency",
            "message_type",
            "messageType",
            "enum_type",
            "enumType",
            "service",
            "extension",
            "options",
            "source_code_info",
            "sourceCodeInfo",
            "syntax",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Package,
            Dependency,
            PublicDependency,
            WeakDependency,
            MessageType,
            EnumType,
            Service,
            Extension,
            Options,
            SourceCodeInfo,
            Syntax,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "package" => Ok(GeneratedField::Package),
                            "dependency" => Ok(GeneratedField::Dependency),
                            "publicDependency" | "public_dependency" => Ok(GeneratedField::PublicDependency),
                            "weakDependency" | "weak_dependency" => Ok(GeneratedField::WeakDependency),
                            "messageType" | "message_type" => Ok(GeneratedField::MessageType),
                            "enumType" | "enum_type" => Ok(GeneratedField::EnumType),
                            "service" => Ok(GeneratedField::Service),
                            "extension" => Ok(GeneratedField::Extension),
                            "options" => Ok(GeneratedField::Options),
                            "sourceCodeInfo" | "source_code_info" => Ok(GeneratedField::SourceCodeInfo),
                            "syntax" => Ok(GeneratedField::Syntax),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileDescriptorProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.FileDescriptorProto")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileDescriptorProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut package__ = None;
                let mut dependency__ = None;
                let mut public_dependency__ = None;
                let mut weak_dependency__ = None;
                let mut message_type__ = None;
                let mut enum_type__ = None;
                let mut service__ = None;
                let mut extension__ = None;
                let mut options__ = None;
                let mut source_code_info__ = None;
                let mut syntax__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Package => {
                            if package__.is_some() {
                                return Err(serde::de::Error::duplicate_field("package"));
                            }
                            package__ = Some(map.next_value()?);
                        }
                        GeneratedField::Dependency => {
                            if dependency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dependency"));
                            }
                            dependency__ = Some(map.next_value()?);
                        }
                        GeneratedField::PublicDependency => {
                            if public_dependency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicDependency"));
                            }
                            public_dependency__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::WeakDependency => {
                            if weak_dependency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weakDependency"));
                            }
                            weak_dependency__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::MessageType => {
                            if message_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageType"));
                            }
                            message_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnumType => {
                            if enum_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumType"));
                            }
                            enum_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map.next_value()?);
                        }
                        GeneratedField::Extension => {
                            if extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            extension__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceCodeInfo => {
                            if source_code_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceCodeInfo"));
                            }
                            source_code_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::Syntax => {
                            if syntax__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syntax"));
                            }
                            syntax__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FileDescriptorProto {
                    name: name__,
                    package: package__,
                    dependency: dependency__.unwrap_or_default(),
                    public_dependency: public_dependency__.unwrap_or_default(),
                    weak_dependency: weak_dependency__.unwrap_or_default(),
                    message_type: message_type__.unwrap_or_default(),
                    enum_type: enum_type__.unwrap_or_default(),
                    service: service__.unwrap_or_default(),
                    extension: extension__.unwrap_or_default(),
                    options: options__,
                    source_code_info: source_code_info__,
                    syntax: syntax__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.FileDescriptorProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileDescriptorSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.FileDescriptorSet", len)?;
        if !self.file.is_empty() {
            struct_ser.serialize_field("file", &self.file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileDescriptorSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            File,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "file" => Ok(GeneratedField::File),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileDescriptorSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.FileDescriptorSet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileDescriptorSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::File => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            file__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FileDescriptorSet {
                    file: file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.FileDescriptorSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.java_package.is_some() {
            len += 1;
        }
        if self.java_outer_classname.is_some() {
            len += 1;
        }
        if self.java_multiple_files.is_some() {
            len += 1;
        }
        if self.java_generate_equals_and_hash.is_some() {
            len += 1;
        }
        if self.java_string_check_utf8.is_some() {
            len += 1;
        }
        if self.optimize_for.is_some() {
            len += 1;
        }
        if self.go_package.is_some() {
            len += 1;
        }
        if self.cc_generic_services.is_some() {
            len += 1;
        }
        if self.java_generic_services.is_some() {
            len += 1;
        }
        if self.py_generic_services.is_some() {
            len += 1;
        }
        if self.php_generic_services.is_some() {
            len += 1;
        }
        if self.deprecated.is_some() {
            len += 1;
        }
        if self.cc_enable_arenas.is_some() {
            len += 1;
        }
        if self.objc_class_prefix.is_some() {
            len += 1;
        }
        if self.csharp_namespace.is_some() {
            len += 1;
        }
        if self.swift_prefix.is_some() {
            len += 1;
        }
        if self.php_class_prefix.is_some() {
            len += 1;
        }
        if self.php_namespace.is_some() {
            len += 1;
        }
        if self.php_metadata_namespace.is_some() {
            len += 1;
        }
        if self.ruby_package.is_some() {
            len += 1;
        }
        if !self.uninterpreted_option.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.FileOptions", len)?;
        if let Some(v) = self.java_package.as_ref() {
            struct_ser.serialize_field("javaPackage", v)?;
        }
        if let Some(v) = self.java_outer_classname.as_ref() {
            struct_ser.serialize_field("javaOuterClassname", v)?;
        }
        if let Some(v) = self.java_multiple_files.as_ref() {
            struct_ser.serialize_field("javaMultipleFiles", v)?;
        }
        if let Some(v) = self.java_generate_equals_and_hash.as_ref() {
            struct_ser.serialize_field("javaGenerateEqualsAndHash", v)?;
        }
        if let Some(v) = self.java_string_check_utf8.as_ref() {
            struct_ser.serialize_field("javaStringCheckUtf8", v)?;
        }
        if let Some(v) = self.optimize_for.as_ref() {
            let v = file_options::OptimizeMode::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("optimizeFor", &v)?;
        }
        if let Some(v) = self.go_package.as_ref() {
            struct_ser.serialize_field("goPackage", v)?;
        }
        if let Some(v) = self.cc_generic_services.as_ref() {
            struct_ser.serialize_field("ccGenericServices", v)?;
        }
        if let Some(v) = self.java_generic_services.as_ref() {
            struct_ser.serialize_field("javaGenericServices", v)?;
        }
        if let Some(v) = self.py_generic_services.as_ref() {
            struct_ser.serialize_field("pyGenericServices", v)?;
        }
        if let Some(v) = self.php_generic_services.as_ref() {
            struct_ser.serialize_field("phpGenericServices", v)?;
        }
        if let Some(v) = self.deprecated.as_ref() {
            struct_ser.serialize_field("deprecated", v)?;
        }
        if let Some(v) = self.cc_enable_arenas.as_ref() {
            struct_ser.serialize_field("ccEnableArenas", v)?;
        }
        if let Some(v) = self.objc_class_prefix.as_ref() {
            struct_ser.serialize_field("objcClassPrefix", v)?;
        }
        if let Some(v) = self.csharp_namespace.as_ref() {
            struct_ser.serialize_field("csharpNamespace", v)?;
        }
        if let Some(v) = self.swift_prefix.as_ref() {
            struct_ser.serialize_field("swiftPrefix", v)?;
        }
        if let Some(v) = self.php_class_prefix.as_ref() {
            struct_ser.serialize_field("phpClassPrefix", v)?;
        }
        if let Some(v) = self.php_namespace.as_ref() {
            struct_ser.serialize_field("phpNamespace", v)?;
        }
        if let Some(v) = self.php_metadata_namespace.as_ref() {
            struct_ser.serialize_field("phpMetadataNamespace", v)?;
        }
        if let Some(v) = self.ruby_package.as_ref() {
            struct_ser.serialize_field("rubyPackage", v)?;
        }
        if !self.uninterpreted_option.is_empty() {
            struct_ser.serialize_field("uninterpretedOption", &self.uninterpreted_option)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "java_package",
            "javaPackage",
            "java_outer_classname",
            "javaOuterClassname",
            "java_multiple_files",
            "javaMultipleFiles",
            "java_generate_equals_and_hash",
            "javaGenerateEqualsAndHash",
            "java_string_check_utf8",
            "javaStringCheckUtf8",
            "optimize_for",
            "optimizeFor",
            "go_package",
            "goPackage",
            "cc_generic_services",
            "ccGenericServices",
            "java_generic_services",
            "javaGenericServices",
            "py_generic_services",
            "pyGenericServices",
            "php_generic_services",
            "phpGenericServices",
            "deprecated",
            "cc_enable_arenas",
            "ccEnableArenas",
            "objc_class_prefix",
            "objcClassPrefix",
            "csharp_namespace",
            "csharpNamespace",
            "swift_prefix",
            "swiftPrefix",
            "php_class_prefix",
            "phpClassPrefix",
            "php_namespace",
            "phpNamespace",
            "php_metadata_namespace",
            "phpMetadataNamespace",
            "ruby_package",
            "rubyPackage",
            "uninterpreted_option",
            "uninterpretedOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JavaPackage,
            JavaOuterClassname,
            JavaMultipleFiles,
            JavaGenerateEqualsAndHash,
            JavaStringCheckUtf8,
            OptimizeFor,
            GoPackage,
            CcGenericServices,
            JavaGenericServices,
            PyGenericServices,
            PhpGenericServices,
            Deprecated,
            CcEnableArenas,
            ObjcClassPrefix,
            CsharpNamespace,
            SwiftPrefix,
            PhpClassPrefix,
            PhpNamespace,
            PhpMetadataNamespace,
            RubyPackage,
            UninterpretedOption,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "javaPackage" | "java_package" => Ok(GeneratedField::JavaPackage),
                            "javaOuterClassname" | "java_outer_classname" => Ok(GeneratedField::JavaOuterClassname),
                            "javaMultipleFiles" | "java_multiple_files" => Ok(GeneratedField::JavaMultipleFiles),
                            "javaGenerateEqualsAndHash" | "java_generate_equals_and_hash" => Ok(GeneratedField::JavaGenerateEqualsAndHash),
                            "javaStringCheckUtf8" | "java_string_check_utf8" => Ok(GeneratedField::JavaStringCheckUtf8),
                            "optimizeFor" | "optimize_for" => Ok(GeneratedField::OptimizeFor),
                            "goPackage" | "go_package" => Ok(GeneratedField::GoPackage),
                            "ccGenericServices" | "cc_generic_services" => Ok(GeneratedField::CcGenericServices),
                            "javaGenericServices" | "java_generic_services" => Ok(GeneratedField::JavaGenericServices),
                            "pyGenericServices" | "py_generic_services" => Ok(GeneratedField::PyGenericServices),
                            "phpGenericServices" | "php_generic_services" => Ok(GeneratedField::PhpGenericServices),
                            "deprecated" => Ok(GeneratedField::Deprecated),
                            "ccEnableArenas" | "cc_enable_arenas" => Ok(GeneratedField::CcEnableArenas),
                            "objcClassPrefix" | "objc_class_prefix" => Ok(GeneratedField::ObjcClassPrefix),
                            "csharpNamespace" | "csharp_namespace" => Ok(GeneratedField::CsharpNamespace),
                            "swiftPrefix" | "swift_prefix" => Ok(GeneratedField::SwiftPrefix),
                            "phpClassPrefix" | "php_class_prefix" => Ok(GeneratedField::PhpClassPrefix),
                            "phpNamespace" | "php_namespace" => Ok(GeneratedField::PhpNamespace),
                            "phpMetadataNamespace" | "php_metadata_namespace" => Ok(GeneratedField::PhpMetadataNamespace),
                            "rubyPackage" | "ruby_package" => Ok(GeneratedField::RubyPackage),
                            "uninterpretedOption" | "uninterpreted_option" => Ok(GeneratedField::UninterpretedOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.FileOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut java_package__ = None;
                let mut java_outer_classname__ = None;
                let mut java_multiple_files__ = None;
                let mut java_generate_equals_and_hash__ = None;
                let mut java_string_check_utf8__ = None;
                let mut optimize_for__ = None;
                let mut go_package__ = None;
                let mut cc_generic_services__ = None;
                let mut java_generic_services__ = None;
                let mut py_generic_services__ = None;
                let mut php_generic_services__ = None;
                let mut deprecated__ = None;
                let mut cc_enable_arenas__ = None;
                let mut objc_class_prefix__ = None;
                let mut csharp_namespace__ = None;
                let mut swift_prefix__ = None;
                let mut php_class_prefix__ = None;
                let mut php_namespace__ = None;
                let mut php_metadata_namespace__ = None;
                let mut ruby_package__ = None;
                let mut uninterpreted_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::JavaPackage => {
                            if java_package__.is_some() {
                                return Err(serde::de::Error::duplicate_field("javaPackage"));
                            }
                            java_package__ = Some(map.next_value()?);
                        }
                        GeneratedField::JavaOuterClassname => {
                            if java_outer_classname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("javaOuterClassname"));
                            }
                            java_outer_classname__ = Some(map.next_value()?);
                        }
                        GeneratedField::JavaMultipleFiles => {
                            if java_multiple_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("javaMultipleFiles"));
                            }
                            java_multiple_files__ = Some(map.next_value()?);
                        }
                        GeneratedField::JavaGenerateEqualsAndHash => {
                            if java_generate_equals_and_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("javaGenerateEqualsAndHash"));
                            }
                            java_generate_equals_and_hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::JavaStringCheckUtf8 => {
                            if java_string_check_utf8__.is_some() {
                                return Err(serde::de::Error::duplicate_field("javaStringCheckUtf8"));
                            }
                            java_string_check_utf8__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptimizeFor => {
                            if optimize_for__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optimizeFor"));
                            }
                            optimize_for__ = Some(map.next_value::<file_options::OptimizeMode>()? as i32);
                        }
                        GeneratedField::GoPackage => {
                            if go_package__.is_some() {
                                return Err(serde::de::Error::duplicate_field("goPackage"));
                            }
                            go_package__ = Some(map.next_value()?);
                        }
                        GeneratedField::CcGenericServices => {
                            if cc_generic_services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ccGenericServices"));
                            }
                            cc_generic_services__ = Some(map.next_value()?);
                        }
                        GeneratedField::JavaGenericServices => {
                            if java_generic_services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("javaGenericServices"));
                            }
                            java_generic_services__ = Some(map.next_value()?);
                        }
                        GeneratedField::PyGenericServices => {
                            if py_generic_services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pyGenericServices"));
                            }
                            py_generic_services__ = Some(map.next_value()?);
                        }
                        GeneratedField::PhpGenericServices => {
                            if php_generic_services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phpGenericServices"));
                            }
                            php_generic_services__ = Some(map.next_value()?);
                        }
                        GeneratedField::Deprecated => {
                            if deprecated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecated"));
                            }
                            deprecated__ = Some(map.next_value()?);
                        }
                        GeneratedField::CcEnableArenas => {
                            if cc_enable_arenas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ccEnableArenas"));
                            }
                            cc_enable_arenas__ = Some(map.next_value()?);
                        }
                        GeneratedField::ObjcClassPrefix => {
                            if objc_class_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objcClassPrefix"));
                            }
                            objc_class_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::CsharpNamespace => {
                            if csharp_namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("csharpNamespace"));
                            }
                            csharp_namespace__ = Some(map.next_value()?);
                        }
                        GeneratedField::SwiftPrefix => {
                            if swift_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("swiftPrefix"));
                            }
                            swift_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::PhpClassPrefix => {
                            if php_class_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phpClassPrefix"));
                            }
                            php_class_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::PhpNamespace => {
                            if php_namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phpNamespace"));
                            }
                            php_namespace__ = Some(map.next_value()?);
                        }
                        GeneratedField::PhpMetadataNamespace => {
                            if php_metadata_namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phpMetadataNamespace"));
                            }
                            php_metadata_namespace__ = Some(map.next_value()?);
                        }
                        GeneratedField::RubyPackage => {
                            if ruby_package__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rubyPackage"));
                            }
                            ruby_package__ = Some(map.next_value()?);
                        }
                        GeneratedField::UninterpretedOption => {
                            if uninterpreted_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uninterpretedOption"));
                            }
                            uninterpreted_option__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FileOptions {
                    java_package: java_package__,
                    java_outer_classname: java_outer_classname__,
                    java_multiple_files: java_multiple_files__,
                    java_generate_equals_and_hash: java_generate_equals_and_hash__,
                    java_string_check_utf8: java_string_check_utf8__,
                    optimize_for: optimize_for__,
                    go_package: go_package__,
                    cc_generic_services: cc_generic_services__,
                    java_generic_services: java_generic_services__,
                    py_generic_services: py_generic_services__,
                    php_generic_services: php_generic_services__,
                    deprecated: deprecated__,
                    cc_enable_arenas: cc_enable_arenas__,
                    objc_class_prefix: objc_class_prefix__,
                    csharp_namespace: csharp_namespace__,
                    swift_prefix: swift_prefix__,
                    php_class_prefix: php_class_prefix__,
                    php_namespace: php_namespace__,
                    php_metadata_namespace: php_metadata_namespace__,
                    ruby_package: ruby_package__,
                    uninterpreted_option: uninterpreted_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.FileOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for file_options::OptimizeMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Speed => "SPEED",
            Self::CodeSize => "CODE_SIZE",
            Self::LiteRuntime => "LITE_RUNTIME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for file_options::OptimizeMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SPEED",
            "CODE_SIZE",
            "LITE_RUNTIME",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = file_options::OptimizeMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(file_options::OptimizeMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(file_options::OptimizeMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SPEED" => Ok(file_options::OptimizeMode::Speed),
                    "CODE_SIZE" => Ok(file_options::OptimizeMode::CodeSize),
                    "LITE_RUNTIME" => Ok(file_options::OptimizeMode::LiteRuntime),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GeneratedCodeInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.GeneratedCodeInfo", len)?;
        if !self.annotation.is_empty() {
            struct_ser.serialize_field("annotation", &self.annotation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GeneratedCodeInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "annotation" => Ok(GeneratedField::Annotation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GeneratedCodeInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.GeneratedCodeInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GeneratedCodeInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Annotation => {
                            if annotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            annotation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GeneratedCodeInfo {
                    annotation: annotation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.GeneratedCodeInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for generated_code_info::Annotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.source_file.is_some() {
            len += 1;
        }
        if self.begin.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.GeneratedCodeInfo.Annotation", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.source_file.as_ref() {
            struct_ser.serialize_field("sourceFile", v)?;
        }
        if let Some(v) = self.begin.as_ref() {
            struct_ser.serialize_field("begin", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for generated_code_info::Annotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "source_file",
            "sourceFile",
            "begin",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            SourceFile,
            Begin,
            End,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            "sourceFile" | "source_file" => Ok(GeneratedField::SourceFile),
                            "begin" => Ok(GeneratedField::Begin),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = generated_code_info::Annotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.GeneratedCodeInfo.Annotation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<generated_code_info::Annotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut source_file__ = None;
                let mut begin__ = None;
                let mut end__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::SourceFile => {
                            if source_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceFile"));
                            }
                            source_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::Begin => {
                            if begin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("begin"));
                            }
                            begin__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(generated_code_info::Annotation {
                    path: path__.unwrap_or_default(),
                    source_file: source_file__,
                    begin: begin__,
                    end: end__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.GeneratedCodeInfo.Annotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.message_set_wire_format.is_some() {
            len += 1;
        }
        if self.no_standard_descriptor_accessor.is_some() {
            len += 1;
        }
        if self.deprecated.is_some() {
            len += 1;
        }
        if self.map_entry.is_some() {
            len += 1;
        }
        if !self.uninterpreted_option.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.MessageOptions", len)?;
        if let Some(v) = self.message_set_wire_format.as_ref() {
            struct_ser.serialize_field("messageSetWireFormat", v)?;
        }
        if let Some(v) = self.no_standard_descriptor_accessor.as_ref() {
            struct_ser.serialize_field("noStandardDescriptorAccessor", v)?;
        }
        if let Some(v) = self.deprecated.as_ref() {
            struct_ser.serialize_field("deprecated", v)?;
        }
        if let Some(v) = self.map_entry.as_ref() {
            struct_ser.serialize_field("mapEntry", v)?;
        }
        if !self.uninterpreted_option.is_empty() {
            struct_ser.serialize_field("uninterpretedOption", &self.uninterpreted_option)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_set_wire_format",
            "messageSetWireFormat",
            "no_standard_descriptor_accessor",
            "noStandardDescriptorAccessor",
            "deprecated",
            "map_entry",
            "mapEntry",
            "uninterpreted_option",
            "uninterpretedOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessageSetWireFormat,
            NoStandardDescriptorAccessor,
            Deprecated,
            MapEntry,
            UninterpretedOption,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "messageSetWireFormat" | "message_set_wire_format" => Ok(GeneratedField::MessageSetWireFormat),
                            "noStandardDescriptorAccessor" | "no_standard_descriptor_accessor" => Ok(GeneratedField::NoStandardDescriptorAccessor),
                            "deprecated" => Ok(GeneratedField::Deprecated),
                            "mapEntry" | "map_entry" => Ok(GeneratedField::MapEntry),
                            "uninterpretedOption" | "uninterpreted_option" => Ok(GeneratedField::UninterpretedOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.MessageOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MessageOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_set_wire_format__ = None;
                let mut no_standard_descriptor_accessor__ = None;
                let mut deprecated__ = None;
                let mut map_entry__ = None;
                let mut uninterpreted_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MessageSetWireFormat => {
                            if message_set_wire_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageSetWireFormat"));
                            }
                            message_set_wire_format__ = Some(map.next_value()?);
                        }
                        GeneratedField::NoStandardDescriptorAccessor => {
                            if no_standard_descriptor_accessor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noStandardDescriptorAccessor"));
                            }
                            no_standard_descriptor_accessor__ = Some(map.next_value()?);
                        }
                        GeneratedField::Deprecated => {
                            if deprecated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecated"));
                            }
                            deprecated__ = Some(map.next_value()?);
                        }
                        GeneratedField::MapEntry => {
                            if map_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mapEntry"));
                            }
                            map_entry__ = Some(map.next_value()?);
                        }
                        GeneratedField::UninterpretedOption => {
                            if uninterpreted_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uninterpretedOption"));
                            }
                            uninterpreted_option__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MessageOptions {
                    message_set_wire_format: message_set_wire_format__,
                    no_standard_descriptor_accessor: no_standard_descriptor_accessor__,
                    deprecated: deprecated__,
                    map_entry: map_entry__,
                    uninterpreted_option: uninterpreted_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.MessageOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Method {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.request_type_url.is_empty() {
            len += 1;
        }
        if self.request_streaming {
            len += 1;
        }
        if !self.response_type_url.is_empty() {
            len += 1;
        }
        if self.response_streaming {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        if self.syntax != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.Method", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.request_type_url.is_empty() {
            struct_ser.serialize_field("requestTypeUrl", &self.request_type_url)?;
        }
        if self.request_streaming {
            struct_ser.serialize_field("requestStreaming", &self.request_streaming)?;
        }
        if !self.response_type_url.is_empty() {
            struct_ser.serialize_field("responseTypeUrl", &self.response_type_url)?;
        }
        if self.response_streaming {
            struct_ser.serialize_field("responseStreaming", &self.response_streaming)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        if self.syntax != 0 {
            let v = Syntax::from_i32(self.syntax)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.syntax)))?;
            struct_ser.serialize_field("syntax", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Method {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "request_type_url",
            "requestTypeUrl",
            "request_streaming",
            "requestStreaming",
            "response_type_url",
            "responseTypeUrl",
            "response_streaming",
            "responseStreaming",
            "options",
            "syntax",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            RequestTypeUrl,
            RequestStreaming,
            ResponseTypeUrl,
            ResponseStreaming,
            Options,
            Syntax,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "requestTypeUrl" | "request_type_url" => Ok(GeneratedField::RequestTypeUrl),
                            "requestStreaming" | "request_streaming" => Ok(GeneratedField::RequestStreaming),
                            "responseTypeUrl" | "response_type_url" => Ok(GeneratedField::ResponseTypeUrl),
                            "responseStreaming" | "response_streaming" => Ok(GeneratedField::ResponseStreaming),
                            "options" => Ok(GeneratedField::Options),
                            "syntax" => Ok(GeneratedField::Syntax),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Method;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Method")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Method, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut request_type_url__ = None;
                let mut request_streaming__ = None;
                let mut response_type_url__ = None;
                let mut response_streaming__ = None;
                let mut options__ = None;
                let mut syntax__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestTypeUrl => {
                            if request_type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTypeUrl"));
                            }
                            request_type_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestStreaming => {
                            if request_streaming__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestStreaming"));
                            }
                            request_streaming__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseTypeUrl => {
                            if response_type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseTypeUrl"));
                            }
                            response_type_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseStreaming => {
                            if response_streaming__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseStreaming"));
                            }
                            response_streaming__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::Syntax => {
                            if syntax__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syntax"));
                            }
                            syntax__ = Some(map.next_value::<Syntax>()? as i32);
                        }
                    }
                }
                Ok(Method {
                    name: name__.unwrap_or_default(),
                    request_type_url: request_type_url__.unwrap_or_default(),
                    request_streaming: request_streaming__.unwrap_or_default(),
                    response_type_url: response_type_url__.unwrap_or_default(),
                    response_streaming: response_streaming__.unwrap_or_default(),
                    options: options__.unwrap_or_default(),
                    syntax: syntax__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.Method", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MethodDescriptorProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.input_type.is_some() {
            len += 1;
        }
        if self.output_type.is_some() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if self.client_streaming.is_some() {
            len += 1;
        }
        if self.server_streaming.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.MethodDescriptorProto", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.input_type.as_ref() {
            struct_ser.serialize_field("inputType", v)?;
        }
        if let Some(v) = self.output_type.as_ref() {
            struct_ser.serialize_field("outputType", v)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if let Some(v) = self.client_streaming.as_ref() {
            struct_ser.serialize_field("clientStreaming", v)?;
        }
        if let Some(v) = self.server_streaming.as_ref() {
            struct_ser.serialize_field("serverStreaming", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MethodDescriptorProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "input_type",
            "inputType",
            "output_type",
            "outputType",
            "options",
            "client_streaming",
            "clientStreaming",
            "server_streaming",
            "serverStreaming",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            InputType,
            OutputType,
            Options,
            ClientStreaming,
            ServerStreaming,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "inputType" | "input_type" => Ok(GeneratedField::InputType),
                            "outputType" | "output_type" => Ok(GeneratedField::OutputType),
                            "options" => Ok(GeneratedField::Options),
                            "clientStreaming" | "client_streaming" => Ok(GeneratedField::ClientStreaming),
                            "serverStreaming" | "server_streaming" => Ok(GeneratedField::ServerStreaming),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MethodDescriptorProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.MethodDescriptorProto")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MethodDescriptorProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut input_type__ = None;
                let mut output_type__ = None;
                let mut options__ = None;
                let mut client_streaming__ = None;
                let mut server_streaming__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::InputType => {
                            if input_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputType"));
                            }
                            input_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutputType => {
                            if output_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputType"));
                            }
                            output_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClientStreaming => {
                            if client_streaming__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStreaming"));
                            }
                            client_streaming__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServerStreaming => {
                            if server_streaming__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverStreaming"));
                            }
                            server_streaming__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MethodDescriptorProto {
                    name: name__,
                    input_type: input_type__,
                    output_type: output_type__,
                    options: options__,
                    client_streaming: client_streaming__,
                    server_streaming: server_streaming__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.MethodDescriptorProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MethodOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deprecated.is_some() {
            len += 1;
        }
        if self.idempotency_level.is_some() {
            len += 1;
        }
        if !self.uninterpreted_option.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.MethodOptions", len)?;
        if let Some(v) = self.deprecated.as_ref() {
            struct_ser.serialize_field("deprecated", v)?;
        }
        if let Some(v) = self.idempotency_level.as_ref() {
            let v = method_options::IdempotencyLevel::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("idempotencyLevel", &v)?;
        }
        if !self.uninterpreted_option.is_empty() {
            struct_ser.serialize_field("uninterpretedOption", &self.uninterpreted_option)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MethodOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deprecated",
            "idempotency_level",
            "idempotencyLevel",
            "uninterpreted_option",
            "uninterpretedOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deprecated,
            IdempotencyLevel,
            UninterpretedOption,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "deprecated" => Ok(GeneratedField::Deprecated),
                            "idempotencyLevel" | "idempotency_level" => Ok(GeneratedField::IdempotencyLevel),
                            "uninterpretedOption" | "uninterpreted_option" => Ok(GeneratedField::UninterpretedOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MethodOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.MethodOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MethodOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deprecated__ = None;
                let mut idempotency_level__ = None;
                let mut uninterpreted_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deprecated => {
                            if deprecated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecated"));
                            }
                            deprecated__ = Some(map.next_value()?);
                        }
                        GeneratedField::IdempotencyLevel => {
                            if idempotency_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idempotencyLevel"));
                            }
                            idempotency_level__ = Some(map.next_value::<method_options::IdempotencyLevel>()? as i32);
                        }
                        GeneratedField::UninterpretedOption => {
                            if uninterpreted_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uninterpretedOption"));
                            }
                            uninterpreted_option__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MethodOptions {
                    deprecated: deprecated__,
                    idempotency_level: idempotency_level__,
                    uninterpreted_option: uninterpreted_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.MethodOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for method_options::IdempotencyLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::IdempotencyUnknown => "IDEMPOTENCY_UNKNOWN",
            Self::NoSideEffects => "NO_SIDE_EFFECTS",
            Self::Idempotent => "IDEMPOTENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for method_options::IdempotencyLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IDEMPOTENCY_UNKNOWN",
            "NO_SIDE_EFFECTS",
            "IDEMPOTENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = method_options::IdempotencyLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(method_options::IdempotencyLevel::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(method_options::IdempotencyLevel::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "IDEMPOTENCY_UNKNOWN" => Ok(method_options::IdempotencyLevel::IdempotencyUnknown),
                    "NO_SIDE_EFFECTS" => Ok(method_options::IdempotencyLevel::NoSideEffects),
                    "IDEMPOTENT" => Ok(method_options::IdempotencyLevel::Idempotent),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Mixin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.root.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.Mixin", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.root.is_empty() {
            struct_ser.serialize_field("root", &self.root)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Mixin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "root",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Root,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "root" => Ok(GeneratedField::Root),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Mixin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Mixin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Mixin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut root__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Root => {
                            if root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            root__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Mixin {
                    name: name__.unwrap_or_default(),
                    root: root__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.Mixin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OneofDescriptorProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.OneofDescriptorProto", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OneofDescriptorProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Options,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OneofDescriptorProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.OneofDescriptorProto")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OneofDescriptorProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OneofDescriptorProto {
                    name: name__,
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.OneofDescriptorProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OneofOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uninterpreted_option.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.OneofOptions", len)?;
        if !self.uninterpreted_option.is_empty() {
            struct_ser.serialize_field("uninterpretedOption", &self.uninterpreted_option)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OneofOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uninterpreted_option",
            "uninterpretedOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UninterpretedOption,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uninterpretedOption" | "uninterpreted_option" => Ok(GeneratedField::UninterpretedOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OneofOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.OneofOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OneofOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uninterpreted_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UninterpretedOption => {
                            if uninterpreted_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uninterpretedOption"));
                            }
                            uninterpreted_option__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OneofOptions {
                    uninterpreted_option: uninterpreted_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.OneofOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Option {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.Option", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Option {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Option;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Option")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Option, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Option {
                    name: name__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.Option", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceDescriptorProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if !self.method.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.ServiceDescriptorProto", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if !self.method.is_empty() {
            struct_ser.serialize_field("method", &self.method)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceDescriptorProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "method",
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Method,
            Options,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "method" => Ok(GeneratedField::Method),
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceDescriptorProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.ServiceDescriptorProto")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceDescriptorProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut method__ = None;
                let mut options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ServiceDescriptorProto {
                    name: name__,
                    method: method__.unwrap_or_default(),
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.ServiceDescriptorProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deprecated.is_some() {
            len += 1;
        }
        if !self.uninterpreted_option.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.ServiceOptions", len)?;
        if let Some(v) = self.deprecated.as_ref() {
            struct_ser.serialize_field("deprecated", v)?;
        }
        if !self.uninterpreted_option.is_empty() {
            struct_ser.serialize_field("uninterpretedOption", &self.uninterpreted_option)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deprecated",
            "uninterpreted_option",
            "uninterpretedOption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deprecated,
            UninterpretedOption,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "deprecated" => Ok(GeneratedField::Deprecated),
                            "uninterpretedOption" | "uninterpreted_option" => Ok(GeneratedField::UninterpretedOption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.ServiceOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deprecated__ = None;
                let mut uninterpreted_option__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deprecated => {
                            if deprecated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecated"));
                            }
                            deprecated__ = Some(map.next_value()?);
                        }
                        GeneratedField::UninterpretedOption => {
                            if uninterpreted_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uninterpretedOption"));
                            }
                            uninterpreted_option__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ServiceOptions {
                    deprecated: deprecated__,
                    uninterpreted_option: uninterpreted_option__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.ServiceOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceCodeInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.SourceCodeInfo", len)?;
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceCodeInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "location",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Location,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "location" => Ok(GeneratedField::Location),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceCodeInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.SourceCodeInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SourceCodeInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut location__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SourceCodeInfo {
                    location: location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.SourceCodeInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for source_code_info::Location {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.span.is_empty() {
            len += 1;
        }
        if self.leading_comments.is_some() {
            len += 1;
        }
        if self.trailing_comments.is_some() {
            len += 1;
        }
        if !self.leading_detached_comments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.SourceCodeInfo.Location", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.span.is_empty() {
            struct_ser.serialize_field("span", &self.span)?;
        }
        if let Some(v) = self.leading_comments.as_ref() {
            struct_ser.serialize_field("leadingComments", v)?;
        }
        if let Some(v) = self.trailing_comments.as_ref() {
            struct_ser.serialize_field("trailingComments", v)?;
        }
        if !self.leading_detached_comments.is_empty() {
            struct_ser.serialize_field("leadingDetachedComments", &self.leading_detached_comments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for source_code_info::Location {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "span",
            "leading_comments",
            "leadingComments",
            "trailing_comments",
            "trailingComments",
            "leading_detached_comments",
            "leadingDetachedComments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Span,
            LeadingComments,
            TrailingComments,
            LeadingDetachedComments,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            "span" => Ok(GeneratedField::Span),
                            "leadingComments" | "leading_comments" => Ok(GeneratedField::LeadingComments),
                            "trailingComments" | "trailing_comments" => Ok(GeneratedField::TrailingComments),
                            "leadingDetachedComments" | "leading_detached_comments" => Ok(GeneratedField::LeadingDetachedComments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = source_code_info::Location;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.SourceCodeInfo.Location")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<source_code_info::Location, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut span__ = None;
                let mut leading_comments__ = None;
                let mut trailing_comments__ = None;
                let mut leading_detached_comments__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::Span => {
                            if span__.is_some() {
                                return Err(serde::de::Error::duplicate_field("span"));
                            }
                            span__ = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::LeadingComments => {
                            if leading_comments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leadingComments"));
                            }
                            leading_comments__ = Some(map.next_value()?);
                        }
                        GeneratedField::TrailingComments => {
                            if trailing_comments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trailingComments"));
                            }
                            trailing_comments__ = Some(map.next_value()?);
                        }
                        GeneratedField::LeadingDetachedComments => {
                            if leading_detached_comments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leadingDetachedComments"));
                            }
                            leading_detached_comments__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(source_code_info::Location {
                    path: path__.unwrap_or_default(),
                    span: span__.unwrap_or_default(),
                    leading_comments: leading_comments__,
                    trailing_comments: trailing_comments__,
                    leading_detached_comments: leading_detached_comments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.SourceCodeInfo.Location", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.SourceContext", len)?;
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_name",
            "fileName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.SourceContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SourceContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SourceContext {
                    file_name: file_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.SourceContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Syntax {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Proto2 => "SYNTAX_PROTO2",
            Self::Proto3 => "SYNTAX_PROTO3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Syntax {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SYNTAX_PROTO2",
            "SYNTAX_PROTO3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Syntax;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(Syntax::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(Syntax::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SYNTAX_PROTO2" => Ok(Syntax::Proto2),
                    "SYNTAX_PROTO3" => Ok(Syntax::Proto3),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.fields.is_empty() {
            len += 1;
        }
        if !self.oneofs.is_empty() {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        if self.source_context.is_some() {
            len += 1;
        }
        if self.syntax != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.Type", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        if !self.oneofs.is_empty() {
            struct_ser.serialize_field("oneofs", &self.oneofs)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        if let Some(v) = self.source_context.as_ref() {
            struct_ser.serialize_field("sourceContext", v)?;
        }
        if self.syntax != 0 {
            let v = Syntax::from_i32(self.syntax)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.syntax)))?;
            struct_ser.serialize_field("syntax", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "fields",
            "oneofs",
            "options",
            "source_context",
            "sourceContext",
            "syntax",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Fields,
            Oneofs,
            Options,
            SourceContext,
            Syntax,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "fields" => Ok(GeneratedField::Fields),
                            "oneofs" => Ok(GeneratedField::Oneofs),
                            "options" => Ok(GeneratedField::Options),
                            "sourceContext" | "source_context" => Ok(GeneratedField::SourceContext),
                            "syntax" => Ok(GeneratedField::Syntax),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.Type")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Type, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut fields__ = None;
                let mut oneofs__ = None;
                let mut options__ = None;
                let mut source_context__ = None;
                let mut syntax__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map.next_value()?);
                        }
                        GeneratedField::Oneofs => {
                            if oneofs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oneofs"));
                            }
                            oneofs__ = Some(map.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceContext => {
                            if source_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceContext"));
                            }
                            source_context__ = Some(map.next_value()?);
                        }
                        GeneratedField::Syntax => {
                            if syntax__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syntax"));
                            }
                            syntax__ = Some(map.next_value::<Syntax>()? as i32);
                        }
                    }
                }
                Ok(Type {
                    name: name__.unwrap_or_default(),
                    fields: fields__.unwrap_or_default(),
                    oneofs: oneofs__.unwrap_or_default(),
                    options: options__.unwrap_or_default(),
                    source_context: source_context__,
                    syntax: syntax__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.Type", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UninterpretedOption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.identifier_value.is_some() {
            len += 1;
        }
        if self.positive_int_value.is_some() {
            len += 1;
        }
        if self.negative_int_value.is_some() {
            len += 1;
        }
        if self.double_value.is_some() {
            len += 1;
        }
        if self.string_value.is_some() {
            len += 1;
        }
        if self.aggregate_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.protobuf.UninterpretedOption", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.identifier_value.as_ref() {
            struct_ser.serialize_field("identifierValue", v)?;
        }
        if let Some(v) = self.positive_int_value.as_ref() {
            struct_ser.serialize_field("positiveIntValue", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.negative_int_value.as_ref() {
            struct_ser.serialize_field("negativeIntValue", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.double_value.as_ref() {
            struct_ser.serialize_field("doubleValue", v)?;
        }
        if let Some(v) = self.string_value.as_ref() {
            struct_ser.serialize_field("stringValue", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.aggregate_value.as_ref() {
            struct_ser.serialize_field("aggregateValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UninterpretedOption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "identifier_value",
            "identifierValue",
            "positive_int_value",
            "positiveIntValue",
            "negative_int_value",
            "negativeIntValue",
            "double_value",
            "doubleValue",
            "string_value",
            "stringValue",
            "aggregate_value",
            "aggregateValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            IdentifierValue,
            PositiveIntValue,
            NegativeIntValue,
            DoubleValue,
            StringValue,
            AggregateValue,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "identifierValue" | "identifier_value" => Ok(GeneratedField::IdentifierValue),
                            "positiveIntValue" | "positive_int_value" => Ok(GeneratedField::PositiveIntValue),
                            "negativeIntValue" | "negative_int_value" => Ok(GeneratedField::NegativeIntValue),
                            "doubleValue" | "double_value" => Ok(GeneratedField::DoubleValue),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "aggregateValue" | "aggregate_value" => Ok(GeneratedField::AggregateValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UninterpretedOption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.UninterpretedOption")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UninterpretedOption, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut identifier_value__ = None;
                let mut positive_int_value__ = None;
                let mut negative_int_value__ = None;
                let mut double_value__ = None;
                let mut string_value__ = None;
                let mut aggregate_value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::IdentifierValue => {
                            if identifier_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifierValue"));
                            }
                            identifier_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::PositiveIntValue => {
                            if positive_int_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("positiveIntValue"));
                            }
                            positive_int_value__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::NegativeIntValue => {
                            if negative_int_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("negativeIntValue"));
                            }
                            negative_int_value__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::DoubleValue => {
                            if double_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doubleValue"));
                            }
                            double_value__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::StringValue => {
                            if string_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            string_value__ = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::AggregateValue => {
                            if aggregate_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregateValue"));
                            }
                            aggregate_value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UninterpretedOption {
                    name: name__.unwrap_or_default(),
                    identifier_value: identifier_value__,
                    positive_int_value: positive_int_value__,
                    negative_int_value: negative_int_value__,
                    double_value: double_value__,
                    string_value: string_value__,
                    aggregate_value: aggregate_value__,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.UninterpretedOption", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for uninterpreted_option::NamePart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 2;
        let mut struct_ser = serializer.serialize_struct("google.protobuf.UninterpretedOption.NamePart", len)?;
        struct_ser.serialize_field("namePart", &self.name_part)?;
        struct_ser.serialize_field("isExtension", &self.is_extension)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for uninterpreted_option::NamePart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name_part",
            "namePart",
            "is_extension",
            "isExtension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamePart,
            IsExtension,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "namePart" | "name_part" => Ok(GeneratedField::NamePart),
                            "isExtension" | "is_extension" => Ok(GeneratedField::IsExtension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = uninterpreted_option::NamePart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.protobuf.UninterpretedOption.NamePart")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<uninterpreted_option::NamePart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name_part__ = None;
                let mut is_extension__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NamePart => {
                            if name_part__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namePart"));
                            }
                            name_part__ = Some(map.next_value()?);
                        }
                        GeneratedField::IsExtension => {
                            if is_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExtension"));
                            }
                            is_extension__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(uninterpreted_option::NamePart {
                    name_part: name_part__.ok_or_else(|| serde::de::Error::missing_field("namePart"))?,
                    is_extension: is_extension__.ok_or_else(|| serde::de::Error::missing_field("isExtension"))?,
                })
            }
        }
        deserializer.deserialize_struct("google.protobuf.UninterpretedOption.NamePart", FIELDS, GeneratedVisitor)
    }
}

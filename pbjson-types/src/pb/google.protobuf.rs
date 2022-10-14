#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Any {
    #[prost(string, tag="1")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(bytes="bytes", tag="2")]
    pub value: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceContext {
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    #[prost(string, repeated, tag="3")]
    pub oneofs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(message, optional, tag="5")]
    pub source_context: ::core::option::Option<SourceContext>,
    #[prost(enumeration="Syntax", tag="6")]
    pub syntax: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(enumeration="field::Kind", tag="1")]
    pub kind: i32,
    #[prost(enumeration="field::Cardinality", tag="2")]
    pub cardinality: i32,
    #[prost(int32, tag="3")]
    pub number: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(int32, tag="7")]
    pub oneof_index: i32,
    #[prost(bool, tag="8")]
    pub packed: bool,
    #[prost(message, repeated, tag="9")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(string, tag="10")]
    pub json_name: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub default_value: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Field`.
pub mod field {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        TypeUnknown = 0,
        TypeDouble = 1,
        TypeFloat = 2,
        TypeInt64 = 3,
        TypeUint64 = 4,
        TypeInt32 = 5,
        TypeFixed64 = 6,
        TypeFixed32 = 7,
        TypeBool = 8,
        TypeString = 9,
        TypeGroup = 10,
        TypeMessage = 11,
        TypeBytes = 12,
        TypeUint32 = 13,
        TypeEnum = 14,
        TypeSfixed32 = 15,
        TypeSfixed64 = 16,
        TypeSint32 = 17,
        TypeSint64 = 18,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::TypeUnknown => "TYPE_UNKNOWN",
                Kind::TypeDouble => "TYPE_DOUBLE",
                Kind::TypeFloat => "TYPE_FLOAT",
                Kind::TypeInt64 => "TYPE_INT64",
                Kind::TypeUint64 => "TYPE_UINT64",
                Kind::TypeInt32 => "TYPE_INT32",
                Kind::TypeFixed64 => "TYPE_FIXED64",
                Kind::TypeFixed32 => "TYPE_FIXED32",
                Kind::TypeBool => "TYPE_BOOL",
                Kind::TypeString => "TYPE_STRING",
                Kind::TypeGroup => "TYPE_GROUP",
                Kind::TypeMessage => "TYPE_MESSAGE",
                Kind::TypeBytes => "TYPE_BYTES",
                Kind::TypeUint32 => "TYPE_UINT32",
                Kind::TypeEnum => "TYPE_ENUM",
                Kind::TypeSfixed32 => "TYPE_SFIXED32",
                Kind::TypeSfixed64 => "TYPE_SFIXED64",
                Kind::TypeSint32 => "TYPE_SINT32",
                Kind::TypeSint64 => "TYPE_SINT64",
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Cardinality {
        Unknown = 0,
        Optional = 1,
        Required = 2,
        Repeated = 3,
    }
    impl Cardinality {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Cardinality::Unknown => "CARDINALITY_UNKNOWN",
                Cardinality::Optional => "CARDINALITY_OPTIONAL",
                Cardinality::Required => "CARDINALITY_REQUIRED",
                Cardinality::Repeated => "CARDINALITY_REPEATED",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Enum {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub enumvalue: ::prost::alloc::vec::Vec<EnumValue>,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(message, optional, tag="4")]
    pub source_context: ::core::option::Option<SourceContext>,
    #[prost(enumeration="Syntax", tag="5")]
    pub syntax: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValue {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub number: i32,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Option {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<Any>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Syntax {
    Proto2 = 0,
    Proto3 = 1,
}
impl Syntax {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Syntax::Proto2 => "SYNTAX_PROTO2",
            Syntax::Proto3 => "SYNTAX_PROTO3",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Api {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub methods: ::prost::alloc::vec::Vec<Method>,
    #[prost(message, repeated, tag="3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub source_context: ::core::option::Option<SourceContext>,
    #[prost(message, repeated, tag="6")]
    pub mixins: ::prost::alloc::vec::Vec<Mixin>,
    #[prost(enumeration="Syntax", tag="7")]
    pub syntax: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Method {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub request_type_url: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub request_streaming: bool,
    #[prost(string, tag="4")]
    pub response_type_url: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub response_streaming: bool,
    #[prost(message, repeated, tag="6")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(enumeration="Syntax", tag="7")]
    pub syntax: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mixin {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub root: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDescriptorSet {
    #[prost(message, repeated, tag="1")]
    pub file: ::prost::alloc::vec::Vec<FileDescriptorProto>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub package: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub dependency: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed="false", tag="10")]
    pub public_dependency: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed="false", tag="11")]
    pub weak_dependency: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag="4")]
    pub message_type: ::prost::alloc::vec::Vec<DescriptorProto>,
    #[prost(message, repeated, tag="5")]
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProto>,
    #[prost(message, repeated, tag="6")]
    pub service: ::prost::alloc::vec::Vec<ServiceDescriptorProto>,
    #[prost(message, repeated, tag="7")]
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, optional, tag="8")]
    pub options: ::core::option::Option<FileOptions>,
    #[prost(message, optional, tag="9")]
    pub source_code_info: ::core::option::Option<SourceCodeInfo>,
    #[prost(string, optional, tag="12")]
    pub syntax: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub field: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, repeated, tag="6")]
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, repeated, tag="3")]
    pub nested_type: ::prost::alloc::vec::Vec<DescriptorProto>,
    #[prost(message, repeated, tag="4")]
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProto>,
    #[prost(message, repeated, tag="5")]
    pub extension_range: ::prost::alloc::vec::Vec<descriptor_proto::ExtensionRange>,
    #[prost(message, repeated, tag="8")]
    pub oneof_decl: ::prost::alloc::vec::Vec<OneofDescriptorProto>,
    #[prost(message, optional, tag="7")]
    pub options: ::core::option::Option<MessageOptions>,
    #[prost(message, repeated, tag="9")]
    pub reserved_range: ::prost::alloc::vec::Vec<descriptor_proto::ReservedRange>,
    #[prost(string, repeated, tag="10")]
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DescriptorProto`.
pub mod descriptor_proto {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExtensionRange {
        #[prost(int32, optional, tag="1")]
        pub start: ::core::option::Option<i32>,
        #[prost(int32, optional, tag="2")]
        pub end: ::core::option::Option<i32>,
        #[prost(message, optional, tag="3")]
        pub options: ::core::option::Option<super::ExtensionRangeOptions>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReservedRange {
        #[prost(int32, optional, tag="1")]
        pub start: ::core::option::Option<i32>,
        #[prost(int32, optional, tag="2")]
        pub end: ::core::option::Option<i32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionRangeOptions {
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub number: ::core::option::Option<i32>,
    #[prost(enumeration="field_descriptor_proto::Label", optional, tag="4")]
    pub label: ::core::option::Option<i32>,
    #[prost(enumeration="field_descriptor_proto::Type", optional, tag="5")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="6")]
    pub type_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub extendee: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub default_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="9")]
    pub oneof_index: ::core::option::Option<i32>,
    #[prost(string, optional, tag="10")]
    pub json_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub options: ::core::option::Option<FieldOptions>,
    #[prost(bool, optional, tag="17")]
    pub proto3_optional: ::core::option::Option<bool>,
}
/// Nested message and enum types in `FieldDescriptorProto`.
pub mod field_descriptor_proto {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Double = 1,
        Float = 2,
        Int64 = 3,
        Uint64 = 4,
        Int32 = 5,
        Fixed64 = 6,
        Fixed32 = 7,
        Bool = 8,
        String = 9,
        Group = 10,
        Message = 11,
        Bytes = 12,
        Uint32 = 13,
        Enum = 14,
        Sfixed32 = 15,
        Sfixed64 = 16,
        Sint32 = 17,
        Sint64 = 18,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Double => "TYPE_DOUBLE",
                Type::Float => "TYPE_FLOAT",
                Type::Int64 => "TYPE_INT64",
                Type::Uint64 => "TYPE_UINT64",
                Type::Int32 => "TYPE_INT32",
                Type::Fixed64 => "TYPE_FIXED64",
                Type::Fixed32 => "TYPE_FIXED32",
                Type::Bool => "TYPE_BOOL",
                Type::String => "TYPE_STRING",
                Type::Group => "TYPE_GROUP",
                Type::Message => "TYPE_MESSAGE",
                Type::Bytes => "TYPE_BYTES",
                Type::Uint32 => "TYPE_UINT32",
                Type::Enum => "TYPE_ENUM",
                Type::Sfixed32 => "TYPE_SFIXED32",
                Type::Sfixed64 => "TYPE_SFIXED64",
                Type::Sint32 => "TYPE_SINT32",
                Type::Sint64 => "TYPE_SINT64",
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Label {
        Optional = 1,
        Required = 2,
        Repeated = 3,
    }
    impl Label {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Label::Optional => "LABEL_OPTIONAL",
                Label::Required => "LABEL_REQUIRED",
                Label::Repeated => "LABEL_REPEATED",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneofDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub options: ::core::option::Option<OneofOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub value: ::prost::alloc::vec::Vec<EnumValueDescriptorProto>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<EnumOptions>,
    #[prost(message, repeated, tag="4")]
    pub reserved_range: ::prost::alloc::vec::Vec<enum_descriptor_proto::EnumReservedRange>,
    #[prost(string, repeated, tag="5")]
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `EnumDescriptorProto`.
pub mod enum_descriptor_proto {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumReservedRange {
        #[prost(int32, optional, tag="1")]
        pub start: ::core::option::Option<i32>,
        #[prost(int32, optional, tag="2")]
        pub end: ::core::option::Option<i32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValueDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="2")]
    pub number: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<EnumValueOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub method: ::prost::alloc::vec::Vec<MethodDescriptorProto>,
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<ServiceOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodDescriptorProto {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub input_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub output_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub options: ::core::option::Option<MethodOptions>,
    #[prost(bool, optional, tag="5", default="false")]
    pub client_streaming: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="6", default="false")]
    pub server_streaming: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileOptions {
    #[prost(string, optional, tag="1")]
    pub java_package: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub java_outer_classname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="10", default="false")]
    pub java_multiple_files: ::core::option::Option<bool>,
    #[deprecated]
    #[prost(bool, optional, tag="20")]
    pub java_generate_equals_and_hash: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="27", default="false")]
    pub java_string_check_utf8: ::core::option::Option<bool>,
    #[prost(enumeration="file_options::OptimizeMode", optional, tag="9", default="Speed")]
    pub optimize_for: ::core::option::Option<i32>,
    #[prost(string, optional, tag="11")]
    pub go_package: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="16", default="false")]
    pub cc_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="17", default="false")]
    pub java_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="18", default="false")]
    pub py_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="42", default="false")]
    pub php_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="23", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="31", default="true")]
    pub cc_enable_arenas: ::core::option::Option<bool>,
    #[prost(string, optional, tag="36")]
    pub objc_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="37")]
    pub csharp_namespace: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="39")]
    pub swift_prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="40")]
    pub php_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="41")]
    pub php_namespace: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="44")]
    pub php_metadata_namespace: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="45")]
    pub ruby_package: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
/// Nested message and enum types in `FileOptions`.
pub mod file_options {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OptimizeMode {
        Speed = 1,
        CodeSize = 2,
        LiteRuntime = 3,
    }
    impl OptimizeMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OptimizeMode::Speed => "SPEED",
                OptimizeMode::CodeSize => "CODE_SIZE",
                OptimizeMode::LiteRuntime => "LITE_RUNTIME",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageOptions {
    #[prost(bool, optional, tag="1", default="false")]
    pub message_set_wire_format: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2", default="false")]
    pub no_standard_descriptor_accessor: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="7")]
    pub map_entry: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldOptions {
    #[prost(enumeration="field_options::CType", optional, tag="1", default="String")]
    pub ctype: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="2")]
    pub packed: ::core::option::Option<bool>,
    #[prost(enumeration="field_options::JsType", optional, tag="6", default="JsNormal")]
    pub jstype: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="5", default="false")]
    pub lazy: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="10", default="false")]
    pub weak: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
/// Nested message and enum types in `FieldOptions`.
pub mod field_options {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CType {
        String = 0,
        Cord = 1,
        StringPiece = 2,
    }
    impl CType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CType::String => "STRING",
                CType::Cord => "CORD",
                CType::StringPiece => "STRING_PIECE",
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JsType {
        JsNormal = 0,
        JsString = 1,
        JsNumber = 2,
    }
    impl JsType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JsType::JsNormal => "JS_NORMAL",
                JsType::JsString => "JS_STRING",
                JsType::JsNumber => "JS_NUMBER",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneofOptions {
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumOptions {
    #[prost(bool, optional, tag="2")]
    pub allow_alias: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValueOptions {
    #[prost(bool, optional, tag="1", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceOptions {
    #[prost(bool, optional, tag="33", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodOptions {
    #[prost(bool, optional, tag="33", default="false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(enumeration="method_options::IdempotencyLevel", optional, tag="34", default="IdempotencyUnknown")]
    pub idempotency_level: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
/// Nested message and enum types in `MethodOptions`.
pub mod method_options {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IdempotencyLevel {
        IdempotencyUnknown = 0,
        NoSideEffects = 1,
        Idempotent = 2,
    }
    impl IdempotencyLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IdempotencyLevel::IdempotencyUnknown => "IDEMPOTENCY_UNKNOWN",
                IdempotencyLevel::NoSideEffects => "NO_SIDE_EFFECTS",
                IdempotencyLevel::Idempotent => "IDEMPOTENT",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UninterpretedOption {
    #[prost(message, repeated, tag="2")]
    pub name: ::prost::alloc::vec::Vec<uninterpreted_option::NamePart>,
    #[prost(string, optional, tag="3")]
    pub identifier_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="4")]
    pub positive_int_value: ::core::option::Option<u64>,
    #[prost(int64, optional, tag="5")]
    pub negative_int_value: ::core::option::Option<i64>,
    #[prost(double, optional, tag="6")]
    pub double_value: ::core::option::Option<f64>,
    #[prost(bytes="bytes", optional, tag="7")]
    pub string_value: ::core::option::Option<::prost::bytes::Bytes>,
    #[prost(string, optional, tag="8")]
    pub aggregate_value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `UninterpretedOption`.
pub mod uninterpreted_option {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NamePart {
        #[prost(string, required, tag="1")]
        pub name_part: ::prost::alloc::string::String,
        #[prost(bool, required, tag="2")]
        pub is_extension: bool,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCodeInfo {
    #[prost(message, repeated, tag="1")]
    pub location: ::prost::alloc::vec::Vec<source_code_info::Location>,
}
/// Nested message and enum types in `SourceCodeInfo`.
pub mod source_code_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Location {
        #[prost(int32, repeated, tag="1")]
        pub path: ::prost::alloc::vec::Vec<i32>,
        #[prost(int32, repeated, tag="2")]
        pub span: ::prost::alloc::vec::Vec<i32>,
        #[prost(string, optional, tag="3")]
        pub leading_comments: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag="4")]
        pub trailing_comments: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, repeated, tag="6")]
        pub leading_detached_comments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratedCodeInfo {
    #[prost(message, repeated, tag="1")]
    pub annotation: ::prost::alloc::vec::Vec<generated_code_info::Annotation>,
}
/// Nested message and enum types in `GeneratedCodeInfo`.
pub mod generated_code_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Annotation {
        #[prost(int32, repeated, tag="1")]
        pub path: ::prost::alloc::vec::Vec<i32>,
        #[prost(string, optional, tag="2")]
        pub source_file: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag="3")]
        pub begin: ::core::option::Option<i32>,
        #[prost(int32, optional, tag="4")]
        pub end: ::core::option::Option<i32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Duration {
    #[prost(int64, tag="1")]
    pub seconds: i64,
    #[prost(int32, tag="2")]
    pub nanos: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldMask {
    #[prost(string, repeated, tag="1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {
    #[prost(map="string, message", tag="1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof="value::Kind", tags="1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(enumeration="super::NullValue", tag="1")]
        NullValue(i32),
        #[prost(double, tag="2")]
        NumberValue(f64),
        #[prost(string, tag="3")]
        StringValue(::prost::alloc::string::String),
        #[prost(bool, tag="4")]
        BoolValue(bool),
        #[prost(message, tag="5")]
        StructValue(super::Struct),
        #[prost(message, tag="6")]
        ListValue(super::ListValue),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NullValue {
    NullValue = 0,
}
impl NullValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NullValue::NullValue => "NULL_VALUE",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    #[prost(int64, tag="1")]
    pub seconds: i64,
    #[prost(int32, tag="2")]
    pub nanos: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleValue {
    #[prost(double, tag="1")]
    pub value: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatValue {
    #[prost(float, tag="1")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Value {
    #[prost(int64, tag="1")]
    pub value: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt64Value {
    #[prost(uint64, tag="1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Value {
    #[prost(int32, tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32Value {
    #[prost(uint32, tag="1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolValue {
    #[prost(bool, tag="1")]
    pub value: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringValue {
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesValue {
    #[prost(bytes="bytes", tag="1")]
    pub value: ::prost::bytes::Bytes,
}

use serde::{Deserialize, Serialize};

/// A test of an externally defined message
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct ExternMessage {}

/// A test of an externally defined enumeration
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
    Serialize,
    Deserialize,
)]
#[repr(i32)]
pub enum ExternEnumeration {
    Unknown = 0,
}

pub mod test {
    pub mod syntax3 {
        include!(concat!(env!("OUT_DIR"), "/test.syntax3.rs"));
        include!(concat!(env!("OUT_DIR"), "/test.syntax3.serde.rs"));
    }
    pub mod common {
        include!(concat!(env!("OUT_DIR"), "/test.common.rs"));
        include!(concat!(env!("OUT_DIR"), "/test.common.serde.rs"));
    }
    pub mod duplicate_name {
        include!(concat!(env!("OUT_DIR"), "/test.duplicate_name.rs"));
        include!(concat!(env!("OUT_DIR"), "/test.duplicate_name.serde.rs"));
    }
    pub mod escape {
        include!(concat!(
            env!("OUT_DIR"),
            "/test.r#abstract.r#type.escape.rs"
        ));
        include!(concat!(
            env!("OUT_DIR"),
            "/test.r#abstract.r#type.escape.serde.rs"
        ));
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::test::syntax3::kitchen_sink::MixedCase;
    use chrono::TimeZone;
    use pbjson_types::{Duration, Timestamp};
    use test::syntax3::*;

    /// A struct holding the expected json-encoded representation of a test step.
    ///
    /// The json can come in two forms: fields with lowerCamelCase (typical expected) or with the
    /// original name defined in the `.proto` schema. When decoding json, both forms must be
    /// accepted by the parser. When encoding json, the form will be chosen based on the builder's
    /// `preserve_proto_field_names` option, which during testing is toggled with a feature flag
    #[derive(Debug, Clone, Copy)]
    struct EncodedStrings {
        expected: &'static str,

        /// the proto-preserving encoding; None if equal to the standard encoding
        expected_preserved_proto: Option<&'static str>,
    }

    impl From<&'static str> for EncodedStrings {
        fn from(expected: &'static str) -> Self {
            EncodedStrings {
                expected,
                expected_preserved_proto: None,
            }
        }
    }
    impl From<(&'static str, &'static str)> for EncodedStrings {
        fn from((expected, expected_preserved_proto): (&'static str, &'static str)) -> Self {
            EncodedStrings {
                expected,
                expected_preserved_proto: Some(expected_preserved_proto),
            }
        }
    }

    fn verify_encode(decoded: &KitchenSink, expected: impl Into<EncodedStrings>) {
        let encoded = expected.into();

        // the proto-preserved encoding is only used if the flag is set and the alternate form was
        // actually provided
        let expected = match (
            cfg!(feature = "preserve-proto-field-names"),
            encoded.expected_preserved_proto,
        ) {
            (true, Some(preserved_proto)) => preserved_proto,
            _ => encoded.expected,
        };

        assert_eq!(serde_json::to_string(&decoded).unwrap().as_str(), expected);
    }

    fn verify_decode(decoded: &KitchenSink, expected: impl Into<EncodedStrings>) {
        let encoded = expected.into();

        // Decode from a string first
        assert_eq!(decoded, &serde_json::from_str(encoded.expected).unwrap());

        // Then, try decoding from a Reader: this can catch issues when trying to borrow data
        // from the input, which is not possible when deserializing from a Reader (e.g. an opened
        // file).
        assert_eq!(
            decoded,
            &serde_json::from_reader(encoded.expected.as_bytes()).unwrap()
        );

        //both the lowerCamelCase and proto-preserved encodings must be accepted during decode
        if let Some(expected_preserved_proto) = encoded.expected_preserved_proto {
            assert_eq!(
                decoded,
                &serde_json::from_str(expected_preserved_proto).unwrap()
            );
            assert_eq!(
                decoded,
                &serde_json::from_reader(expected_preserved_proto.as_bytes()).unwrap()
            );
        }
    }

    fn verify(decoded: &KitchenSink, expected: impl Into<EncodedStrings>) {
        let expected = expected.into();
        verify_encode(decoded, expected);
        verify_decode(decoded, expected);
    }

    #[test]
    #[cfg(not(feature = "ignore-unknown-fields"))]
    fn test_unknown_field_error() {
        let message = Empty {};

        let encoded = serde_json::to_string(&message).unwrap();
        let _decoded: Empty = serde_json::from_str(&encoded).unwrap();

        let err = serde_json::from_str::<Empty>("343").unwrap_err();
        assert_eq!(
            err.to_string().as_str(),
            "invalid type: integer `343`, expected struct test.syntax3.Empty at line 1 column 3"
        );

        let err = serde_json::from_str::<Empty>("{\"foo\": \"bar\"}").unwrap_err();
        assert_eq!(
            err.to_string().as_str(),
            "unknown field `foo`, there are no fields at line 1 column 6"
        );
    }

    #[test]
    #[cfg(feature = "ignore-unknown-fields")]
    fn test_ignore_unknown_field() {
        let message = Empty {};

        let encoded = serde_json::to_string(&message).unwrap();
        let _decoded: Empty = serde_json::from_str(&encoded).unwrap();

        let empty = serde_json::from_str::<Empty>("{\n \"foo\": \"bar\"\n}").unwrap();
        assert_eq!(empty, Empty {});
    }

    #[test]
    #[cfg(feature = "btree")]
    fn test_btree() {
        use std::collections::BTreeMap;

        let decoded = serde_json::from_str::<KitchenSink>("{}").unwrap();
        assert_eq!(decoded.string_dict, BTreeMap::new());
    }

    #[test]
    #[cfg(feature = "emit-fields")]
    fn test_emit_fields() {
        let mut decoded: KitchenSink = serde_json::from_str("{}").unwrap();
        assert_ne!(serde_json::to_string(&decoded).unwrap().as_str(), "{}");
    }

    #[test]
    #[cfg(all(not(feature = "emit-fields"), feature = "use-integers-for-enums"))]
    fn test_use_integers_for_enums() {
        let mut decoded: KitchenSink = serde_json::from_str("{}").unwrap();
        assert_eq!(serde_json::to_string(&decoded).unwrap().as_str(), "{}");
        decoded.value = kitchen_sink::Value::A as i32;
        verify(&decoded, r#"{"value":45}"#);
    }

    #[test]
    #[cfg(not(any(feature = "emit-fields", feature = "use-integers-for-enums")))]
    fn test_kitchen_sink() {
        let mut decoded: KitchenSink = serde_json::from_str("{}").unwrap();

        verify(&decoded, "{}");
        decoded.i32 = 24;
        verify(&decoded, r#"{"i32":24}"#);
        decoded.i32 = 0;
        verify_decode(&decoded, "{}");

        // Explicit optional fields can distinguish between no value and default value
        decoded.optional_i32 = Some(2);
        verify(&decoded, (r#"{"optionalI32":2}"#, r#"{"optional_i32":2}"#));

        decoded.optional_i32 = Some(0);
        verify(&decoded, (r#"{"optionalI32":0}"#, r#"{"optional_i32":0}"#));

        // Can also decode from string
        verify_decode(
            &decoded,
            (r#"{"optionalI32":"0"}"#, r#"{"optional_i32":"0"}"#),
        );

        decoded.optional_i32 = None;
        verify_decode(&decoded, "{}");

        // 64-bit integers are encoded as strings
        decoded.i64 = 123125;
        verify(&decoded, r#"{"i64":"123125"}"#);

        decoded.i64 = 0;
        verify_decode(&decoded, "{}");

        decoded.optional_i64 = Some(532);
        verify(
            &decoded,
            (r#"{"optionalI64":"532"}"#, r#"{"optional_i64":"532"}"#),
        );

        decoded.optional_i64 = Some(0);
        verify(
            &decoded,
            (r#"{"optionalI64":"0"}"#, r#"{"optional_i64":"0"}"#),
        );

        // Can also decode from non-string
        verify_decode(&decoded, (r#"{"optionalI64":0}"#, r#"{"optional_i64":0}"#));

        decoded.optional_i64 = None;
        verify_decode(&decoded, "{}");

        decoded.u64 = 34346;
        decoded.u32 = 567094456;
        decoded.optional_u32 = Some(0);
        decoded.optional_u64 = Some(3);
        verify(
            &decoded,
            (
                r#"{"u32":567094456,"optionalU32":0,"u64":"34346","optionalU64":"3"}"#,
                r#"{"u32":567094456,"optional_u32":0,"u64":"34346","optional_u64":"3"}"#,
            ),
        );

        decoded.u64 = 0;
        decoded.u32 = 0;
        decoded.optional_u32 = None;
        decoded.optional_u64 = None;
        verify_decode(&decoded, "{}");

        decoded.repeated_i32 = vec![0, 23, 5, 6, 2, 34];
        verify(
            &decoded,
            (
                r#"{"repeatedI32":[0,23,5,6,2,34]}"#,
                r#"{"repeated_i32":[0,23,5,6,2,34]}"#,
            ),
        );
        // Can also mix in some strings
        verify_decode(
            &decoded,
            (
                r#"{"repeatedI32":[0,"23",5,6,"2",34]}"#,
                r#"{"repeated_i32":[0,"23",5,6,"2",34]}"#,
            ),
        );

        decoded.repeated_i32 = vec![];
        verify_decode(&decoded, "{}");

        decoded.repeated_u64 = vec![0, 532, 2];
        verify(
            &decoded,
            (
                r#"{"repeatedU64":["0","532","2"]}"#,
                r#"{"repeated_u64":["0","532","2"]}"#,
            ),
        );
        // Can also mix in some non-strings
        verify_decode(
            &decoded,
            (
                r#"{"repeatedU64":["0",532,"2"]}"#,
                r#"{"repeated_u64":["0",532,"2"]}"#,
            ),
        );

        decoded.repeated_u64 = vec![];
        verify_decode(&decoded, "{}");

        // Enumerations should be encoded as strings
        decoded.value = kitchen_sink::Value::A as i32;
        verify(&decoded, r#"{"value":"VALUE_A"}"#);

        // Can also use variant number
        verify_decode(&decoded, r#"{"value":45}"#);

        decoded.value = kitchen_sink::Value::Unknown as i32;
        verify_decode(&decoded, "{}");

        decoded.optional_value = Some(kitchen_sink::Value::Unknown as i32);
        verify(
            &decoded,
            (
                r#"{"optionalValue":"VALUE_UNKNOWN"}"#,
                r#"{"optional_value":"VALUE_UNKNOWN"}"#,
            ),
        );

        // Can also use variant number
        verify_decode(
            &decoded,
            (r#"{"optionalValue":0}"#, r#"{"optional_value":0}"#),
        );

        decoded.optional_value = None;
        verify_decode(&decoded, "{}");

        decoded
            .string_dict
            .insert("foo".to_string(), "bar".to_string());
        verify(
            &decoded,
            (
                r#"{"stringDict":{"foo":"bar"}}"#,
                r#"{"string_dict":{"foo":"bar"}}"#,
            ),
        );

        decoded.string_dict = Default::default();
        verify_decode(&decoded, "{}");

        decoded
            .int32_dict
            .insert(343, kitchen_sink::Prefix::A as i32);
        // Dictionary keys should always be strings
        // Enum dictionary values should be encoded as strings
        verify(
            &decoded,
            (
                r#"{"int32Dict":{"343":"A"}}"#,
                r#"{"int32_dict":{"343":"A"}}"#,
            ),
        );
        // Enum dictionary values can be decoded from integers
        verify_decode(
            &decoded,
            (
                r#"{"int32Dict":{"343":66}}"#,
                r#"{"int32_dict":{"343":66}}"#,
            ),
        );

        decoded.int32_dict = Default::default();
        verify_decode(&decoded, "{}");

        // 64-bit dictionary values should be encoded as strings
        decoded.integer_dict.insert(12, 13);
        verify(
            &decoded,
            (
                r#"{"integerDict":{"12":"13"}}"#,
                r#"{"integer_dict":{"12":"13"}}"#,
            ),
        );
        // 64-bit dictionary values can be decoded from numeric types
        verify_decode(
            &decoded,
            (
                r#"{"integerDict":{"12":13}}"#,
                r#"{"integer_dict":{"12":13}}"#,
            ),
        );

        decoded.integer_dict = Default::default();
        verify_decode(&decoded, "{}");

        decoded.one_of = Some(kitchen_sink::OneOf::OneOfI32(0));
        verify(&decoded, (r#"{"oneOfI32":0}"#, r#"{"one_of_i32":0}"#));
        // Can also specify string
        verify_decode(&decoded, (r#"{"oneOfI32":"0"}"#, r#"{"one_of_i32":"0"}"#));

        decoded.one_of = Some(kitchen_sink::OneOf::OneOfI32(12));
        verify(&decoded, (r#"{"oneOfI32":12}"#, r#"{"one_of_i32":12}"#));

        decoded.one_of = Some(kitchen_sink::OneOf::OneOfBool(false));
        verify(
            &decoded,
            (r#"{"oneOfBool":false}"#, r#"{"one_of_bool":false}"#),
        );

        decoded.one_of = Some(kitchen_sink::OneOf::OneOfBool(true));
        verify(
            &decoded,
            (r#"{"oneOfBool":true}"#, r#"{"one_of_bool":true}"#),
        );

        decoded.one_of = Some(kitchen_sink::OneOf::OneOfValue(
            kitchen_sink::Value::B as i32,
        ));
        verify(
            &decoded,
            (
                r#"{"oneOfValue":"VALUE_B"}"#,
                r#"{"one_of_value":"VALUE_B"}"#,
            ),
        );
        // Can also specify enum variant
        verify_decode(&decoded, (r#"{"oneOfValue":63}"#, r#"{"one_of_value":63}"#));

        decoded.one_of = None;
        verify_decode(&decoded, "{}");

        decoded.repeated_value = vec![
            kitchen_sink::Value::B as i32,
            kitchen_sink::Value::B as i32,
            kitchen_sink::Value::A as i32,
        ];
        verify(
            &decoded,
            (
                r#"{"repeatedValue":["VALUE_B","VALUE_B","VALUE_A"]}"#,
                r#"{"repeated_value":["VALUE_B","VALUE_B","VALUE_A"]}"#,
            ),
        );
        verify_decode(
            &decoded,
            (
                r#"{"repeatedValue":[63,"VALUE_B","VALUE_A"]}"#,
                r#"{"repeated_value":[63,"VALUE_B","VALUE_A"]}"#,
            ),
        );

        decoded.repeated_value = Default::default();
        verify_decode(&decoded, "{}");

        decoded.bytes = prost::bytes::Bytes::from_static(b"kjkjkj");
        verify(&decoded, r#"{"bytes":"a2pramtq"}"#);

        decoded.bytes = Default::default();
        verify_decode(&decoded, "{}");

        decoded.optional_bytes = Some(prost::bytes::Bytes::from_static(b"kjkjkj"));
        verify(
            &decoded,
            (
                r#"{"optionalBytes":"a2pramtq"}"#,
                r#"{"optional_bytes":"a2pramtq"}"#,
            ),
        );

        decoded.optional_bytes = Some(Default::default());
        verify(
            &decoded,
            (r#"{"optionalBytes":""}"#, r#"{"optional_bytes":""}"#),
        );

        decoded.optional_bytes = None;
        verify_decode(&decoded, "{}");

        decoded.repeated_bytes = vec![
            prost::bytes::Bytes::from_static(b"sdfsd"),
            prost::bytes::Bytes::from_static(b"fghfg"),
        ];
        verify(
            &decoded,
            (
                r#"{"repeatedBytes":["c2Rmc2Q=","ZmdoZmc="]}"#,
                r#"{"repeated_bytes":["c2Rmc2Q=","ZmdoZmc="]}"#,
            ),
        );

        decoded.repeated_bytes = Default::default();
        verify_decode(&decoded, "{}");

        decoded.string_bytes_dict.insert(
            "test".to_string(),
            prost::bytes::Bytes::from_static(b"asdf"),
        );
        verify(
            &decoded,
            (
                r#"{"stringBytesDict":{"test":"YXNkZg=="}}"#,
                r#"{"string_bytes_dict":{"test":"YXNkZg=="}}"#,
            ),
        );

        decoded.string_bytes_dict = Default::default();
        verify_decode(&decoded, "{}");

        decoded
            .int_bytes_dict
            .insert(43, prost::bytes::Bytes::from_static(b"343dfgd"));
        verify(
            &decoded,
            (
                r#"{"intBytesDict":{"43":"MzQzZGZnZA=="}}"#,
                r#"{"int_bytes_dict":{"43":"MzQzZGZnZA=="}}"#,
            ),
        );

        decoded.int_bytes_dict = Default::default();
        verify_decode(&decoded, "{}");

        decoded.string = "test".to_string();
        verify(&decoded, r#"{"string":"test"}"#);

        decoded.string = Default::default();
        verify_decode(&decoded, "{}");

        decoded.optional_string = Some(String::new());
        verify(
            &decoded,
            (r#"{"optionalString":""}"#, r#"{"optional_string":""}"#),
        );

        decoded.optional_string = None;
        verify_decode(&decoded, "{}");

        let date = chrono::Utc.ymd(2072, 3, 1).and_hms_milli(5, 2, 5, 30);
        decoded.timestamp = Some(Timestamp {
            seconds: date.timestamp(),
            nanos: date.timestamp_subsec_nanos() as i32,
        });

        verify(&decoded, r#"{"timestamp":"2072-03-01T05:02:05.030+00:00"}"#);

        decoded.timestamp = None;
        verify_decode(&decoded, "{}");

        decoded.duration = Some(Duration {
            seconds: 40502002,
            nanos: 5049,
        });
        verify(&decoded, r#"{"duration":"40502002.000005049s"}"#);

        decoded.duration = None;
        verify_decode(&decoded, "{}");

        decoded.mixed_case = MixedCase::MixedCasea as _;
        verify(
            &decoded,
            (
                r#"{"mixedCase":"MixedCASEA"}"#,
                r#"{"mixed_case":"MixedCASEA"}"#,
            ),
        );

        decoded.mixed_case = MixedCase::MixEdCaseB as _;
        verify(
            &decoded,
            (
                r#"{"mixedCase":"MixEdCaseB"}"#,
                r#"{"mixed_case":"MixEdCaseB"}"#,
            ),
        );

        decoded.mixed_case = MixedCase::C as _;
        verify(&decoded, (r#"{"mixedCase":"c"}"#, r#"{"mixed_case":"c"}"#));

        decoded.mixed_case = MixedCase::Unknown as _;
        verify(&decoded, r#"{}"#);

        decoded.bool_value = Some(true.into());
        verify(
            &decoded,
            (r#"{"boolValue":true}"#, r#"{"bool_value":true}"#),
        );

        decoded.bool_value = Some(false.into());
        verify(
            &decoded,
            (r#"{"boolValue":false}"#, r#"{"bool_value":false}"#),
        );

        decoded.bool_value = None;
        verify(&decoded, r#"{}"#);

        decoded.bytes_value = Some(prost::bytes::Bytes::from_static(b"kjkjkj").into());
        verify(
            &decoded,
            (
                r#"{"bytesValue":"a2pramtq"}"#,
                r#"{"bytes_value":"a2pramtq"}"#,
            ),
        );

        decoded.bytes_value = Some(prost::bytes::Bytes::new().into());
        verify(&decoded, (r#"{"bytesValue":""}"#, r#"{"bytes_value":""}"#));

        decoded.bytes_value = None;
        verify(&decoded, r#"{}"#);

        decoded.double_value = Some(1.1.into());
        verify(
            &decoded,
            (r#"{"doubleValue":1.1}"#, r#"{"double_value":1.1}"#),
        );

        decoded.double_value = Some(0.0.into());
        verify(
            &decoded,
            (r#"{"doubleValue":0.0}"#, r#"{"double_value":0.0}"#),
        );

        decoded.double_value = None;
        verify(&decoded, r#"{}"#);

        decoded.uint32_value = Some(1.into());
        verify(&decoded, (r#"{"uint32Value":1}"#, r#"{"uint32_value":1}"#));

        decoded.uint32_value = Some(0.into());
        verify(&decoded, (r#"{"uint32Value":0}"#, r#"{"uint32_value":0}"#));

        decoded.uint32_value = None;
        verify(&decoded, r#"{}"#);

        decoded.uint64_value = Some(1.into());
        verify(
            &decoded,
            (r#"{"uint64Value":"1"}"#, r#"{"uint64_value":"1"}"#),
        );

        decoded.uint64_value = Some(0.into());
        verify(
            &decoded,
            (r#"{"uint64Value":"0"}"#, r#"{"uint64_value":"0"}"#),
        );

        decoded.uint64_value = None;
        verify(&decoded, r#"{}"#);

        decoded.string_value = Some(String::from("1").into());
        verify(
            &decoded,
            (r#"{"stringValue":"1"}"#, r#"{"string_value":"1"}"#),
        );

        decoded.string_value = Some(String::new().into());
        verify(
            &decoded,
            (r#"{"stringValue":""}"#, r#"{"string_value":""}"#),
        );

        decoded.string_value = None;
        verify(&decoded, r#"{}"#);
    }

    #[test]
    fn test_escaped() -> Result<(), Box<dyn Error>> {
        use super::test::escape::{Abstract, Target, Type};

        let r#type = Type { example: true };
        let r#abstract = Abstract {
            r#type: Some(r#type),
        };
        let target = Target {
            r#abstract: Some(r#abstract),
        };

        let encoded = serde_json::to_string(&target)?;

        let expected = r#"{"abstract":{"type":{"example":true}}}"#;
        assert_eq!(encoded, expected);

        let decoded = serde_json::from_str::<Target>(&encoded)?;
        assert_eq!(decoded, target);

        Ok(())
    }
}

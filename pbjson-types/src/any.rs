const TYPE_FIELD: &str = "@type";
const VALUE_FIELD: &str = "value";

impl serde::Serialize for crate::Any {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::{SerializeMap, SerializeStruct};

        const NAME: &str = "google.protobuf.Any";

        let value = <crate::Value as prost::Message>::decode(self.value.clone()).map_err(|_| {
            serde::ser::Error::custom(
                "Couldn't transcode google.protobuf.Any value into google.protobuf.Value",
            )
        })?;

        let mut field_length = 1;
        match value.kind {
            Some(crate::value::Kind::StructValue(map)) => {
                field_length += map.len();
                let mut map_ser = serializer.serialize_map(Some(field_length))?;
                map_ser.serialize_entry(TYPE_FIELD, &self.type_url)?;

                for (k, v) in &map {
                    map_ser.serialize_entry(k, v)?;
                }

                map_ser.end()
            }
            _ => {
                let mut struct_ser = serializer.serialize_struct(NAME, 2)?;
                struct_ser.serialize_field(TYPE_FIELD, &self.type_url)?;
                struct_ser.serialize_field(VALUE_FIELD, &base64::encode(&self.value))?;
                struct_ser.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for crate::Any {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(AnyVisitor)
    }
}

struct AnyVisitor;

impl<'de> serde::de::Visitor<'de> for AnyVisitor {
    type Value = crate::Any;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("struct google.protobuf.Any")
    }

    fn visit_map<V>(self, mut map_access: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        enum BytesOrValue {
            Bytes(bytes::Bytes),
            Value(crate::Value),
        }

        let mut type_url = None;
        let mut value: Option<BytesOrValue> = None;
        let mut map = std::collections::HashMap::new();
        while let Some(k) = dbg!(map_access.next_key())? {
            match k {
                AnyField::TypeUrl => {
                    if type_url.is_some() {
                        return Err(serde::de::Error::duplicate_field(TYPE_FIELD));
                    }
                    type_url = Some(map_access.next_value()?);
                }
                AnyField::Value => {
                    if value.is_some() {
                        return Err(serde::de::Error::duplicate_field(VALUE_FIELD));
                    }

                    value = if let Ok(bytes) =
                        dbg!(map_access.next_value::<::pbjson::private::BytesDeserialize<_>>())
                    {
                        Some(BytesOrValue::Bytes(bytes.0))
                    } else {
                        Some(BytesOrValue::Value(map_access.next_value()?))
                    };
                }
                AnyField::Unknown(key) => {
                    if map.contains_key(&key) {
                        return Err(serde::de::Error::custom(format!(
                            "Duplicate field: {}",
                            &key
                        )));
                    }

                    map.insert(key, map_access.next_value::<crate::Value>()?);
                }
            }
        }

        macro_rules! encode_map {
            () => {{
                use prost::Message;

                let mut buffer = Vec::new();
                crate::Value::from(map)
                    .encode(&mut buffer)
                    .map_err(serde::de::Error::custom)?;
                buffer.into()
            }};
        }

        let value = match value {
            Some(BytesOrValue::Bytes(bytes)) => bytes,
            Some(BytesOrValue::Value(value)) => {
                map.insert("value".into(), value);
                encode_map!()
            }
            None if map.is_empty() => return Err(serde::de::Error::missing_field(VALUE_FIELD)),
            None => encode_map!(),
        };

        Ok(crate::Any {
            type_url: type_url.ok_or_else(|| serde::de::Error::missing_field(TYPE_FIELD))?,
            value,
        })
    }
}

#[derive(Debug)]
enum AnyField {
    TypeUrl,
    Value,
    Unknown(String),
}

impl<'de> serde::Deserialize<'de> for AnyField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AnyFieldVisitor;

        impl<'de> serde::de::Visitor<'de> for AnyFieldVisitor {
            type Value = AnyField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected a string key")
            }

            fn visit_str<E>(self, value: &str) -> Result<AnyField, E>
            where
                E: serde::de::Error,
            {
                match value {
                    TYPE_FIELD => Ok(AnyField::TypeUrl),
                    VALUE_FIELD => Ok(AnyField::Value),
                    value => Ok(AnyField::Unknown(value.to_owned())),
                }
            }
        }

        deserializer.deserialize_identifier(AnyFieldVisitor)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use prost::Message;

    #[test]
    fn object() {
        let map = crate::Value::from(std::collections::HashMap::from([
            (String::from("bool"), true.into()),
            (String::from("unit"), crate::Value::from(None)),
            (String::from("number"), 5.0.into()),
            (String::from("string"), "string".into()),
            (String::from("list"), vec![1.0.into(), 2.0.into()].into()),
            (
                String::from("map"),
                std::collections::HashMap::from([(String::from("key"), "value".into())]).into(),
            ),
        ]));

        let any = crate::Any {
            type_url: "google.protobuf.Value".into(),
            value: map.encode_to_vec().into(),
        };

        let json = serde_json::to_value(&any).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "@type": "google.protobuf.Value",
                "bool": true,
                "unit": null,
                "number": 5.0,
                "string": "string",
                "list": [1.0, 2.0],
                "map": {
                    "key": "value",
                }
            })
        );

        let decoded = serde_json::from_value::<crate::Any>(json).unwrap();
        assert_eq!(decoded.type_url, any.type_url);
        assert_eq!(
            crate::Value::decode(decoded.value).unwrap(),
            crate::Value::decode(any.value).unwrap()
        );
    }

    #[test]
    fn primitive_value() {
        let boolean = crate::Value::from(true);
        let protobuf_encoding = boolean.encode_to_vec();

        let any = crate::Any {
            type_url: "google.protobuf.Value".into(),
            value: protobuf_encoding.clone().into(),
        };

        let json = dbg!(serde_json::to_value(&any).unwrap());
        let expected = serde_json::json!({
            "@type": "google.protobuf.Value",
            "value": base64::encode(&protobuf_encoding),
        });

        assert_eq!(json, expected);

        let decoded = serde_json::from_value::<crate::Any>(expected).unwrap();
        assert_eq!(decoded.type_url, any.type_url);
        assert_eq!(
            crate::Value::decode(decoded.value).unwrap(),
            crate::Value::decode(any.value).unwrap()
        );
    }
}

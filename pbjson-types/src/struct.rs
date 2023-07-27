use crate::Struct;

use alloc::string::String;

#[cfg(feature = "std")]
impl From<std::collections::HashMap<String, crate::Value>> for Struct {
    fn from(fields: std::collections::HashMap<String, crate::Value>) -> Self {
        Self { fields }
    }
}

#[cfg(not(feature = "std"))]
impl From<alloc::collections::BTreeMap<String, crate::Value>> for Struct {
    fn from(fields: alloc::collections::BTreeMap<String, crate::Value>) -> Self {
        Self { fields }
    }
}

impl FromIterator<(String, crate::Value)> for Struct {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, crate::Value)>,
    {
        Self {
            fields: iter.into_iter().collect(),
        }
    }
}

impl serde::Serialize for Struct {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.fields.serialize(ser)
    }
}

impl<'de> serde::Deserialize<'de> for Struct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(StructVisitor)
    }
}

struct StructVisitor;

impl<'de> serde::de::Visitor<'de> for StructVisitor {
    type Value = Struct;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("google.protobuf.Struct")
    }

    fn visit_map<A>(self, mut map_access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        #[cfg(feature = "std")]
        let mut map = std::collections::HashMap::new();

        #[cfg(not(feature = "std"))]
        let mut map = alloc::collections::BTreeMap::new();

        while let Some((key, value)) = map_access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn it_works() {
        let map: crate::Struct = std::collections::HashMap::from([
            (String::from("bool"), crate::Value::from(true)),
            (
                String::from("unit"),
                crate::value::Kind::NullValue(0).into(),
            ),
            (String::from("number"), 5.0.into()),
            (String::from("string"), "string".into()),
            (String::from("list"), vec![1.0.into(), 2.0.into()].into()),
            (
                String::from("map"),
                std::collections::HashMap::from([(String::from("key"), "value".into())]).into(),
            ),
        ])
        .into();

        assert_eq!(
            serde_json::to_value(map).unwrap(),
            serde_json::json!({
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
    }

    #[cfg(not(feature = "std"))]
    #[test]
    fn it_works_on_no_std() {
        let btree_map = alloc::collections::BTreeMap::from([
            (String::from("bool"), crate::Value::from(true)),
            (
                String::from("unit"),
                crate::value::Kind::NullValue(0).into(),
            ),
            (String::from("number"), 5.0.into()),
            (String::from("string"), "string".into()),
            (
                String::from("list"),
                alloc::vec![1.0.into(), 2.0.into()].into(),
            ),
            (
                String::from("map"),
                alloc::collections::BTreeMap::from([(String::from("key"), "value".into())]).into(),
            ),
        ]);

        let map = crate::Struct::from(btree_map);

        let json_string = r#"{"bool":true,"list":[1.0,2.0],"number":5.0,"string":"string","unit":null,"map":{"key":"value"}}"#;

        assert_eq!(
            serde_json::to_value(map).unwrap(),
            serde_json::from_str::<serde_json::Value>(json_string).unwrap()
        );
    }
}

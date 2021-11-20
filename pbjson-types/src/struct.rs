impl From<std::collections::HashMap<String, crate::Value>> for crate::Struct {
    fn from(fields: std::collections::HashMap<String, crate::Value>) -> Self {
        Self { fields }
    }
}

impl FromIterator<(String, crate::Value)> for crate::Struct {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, crate::Value)>,
    {
        Self {
            fields: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let map = std::collections::HashMap::from([
            (String::from("bool"), true.into()),
            (String::from("unit"), crate::value::Kind::NullValue(0)),
            (String::from("number"), 5.0.into()),
            (String::from("string"), "string".into()),
            (String::from("list"), vec![1.0.into(), 2.0.into()].into()),
            (
                String::from("map"),
                std::collections::HashMap::from([(String::from("key"), "value".into())]).into(),
            ),
        ]);

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
}

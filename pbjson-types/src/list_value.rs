impl From<Vec<crate::Value>> for crate::ListValue {
    fn from(values: Vec<crate::Value>) -> Self {
        Self { values }
    }
}

impl FromIterator<crate::value::Kind> for crate::ListValue {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = crate::value::Kind>,
    {
        Self {
            values: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromIterator<crate::Value> for crate::ListValue {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = crate::Value>,
    {
        Self {
            values: iter.into_iter().collect(),
        }
    }
}

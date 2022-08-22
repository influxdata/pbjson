macro_rules! serde_scalar_value {
    ($typ: ty) => {
        impl serde::Serialize for $typ {
            fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.value.serialize(ser)
            }
        }
        impl<'de> serde::Deserialize<'de> for $typ {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde::Deserialize::deserialize(deserializer)?;
                Ok(Self { value })
            }
        }
    };
}
serde_scalar_value!(crate::BoolValue);
serde_scalar_value!(crate::DoubleValue);
serde_scalar_value!(crate::FloatValue);
serde_scalar_value!(crate::Int32Value);
serde_scalar_value!(crate::Int64Value);
serde_scalar_value!(crate::StringValue);
serde_scalar_value!(crate::UInt32Value);
serde_scalar_value!(crate::UInt64Value);

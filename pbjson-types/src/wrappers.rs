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

macro_rules! convert_scalar_value {
    ($scalar: ty, $typ: ty) => {
        impl From<$scalar> for $typ {
            fn from(value: $scalar) -> Self {
                Self { value }
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

convert_scalar_value!(bool, crate::BoolValue);
convert_scalar_value!(f64, crate::DoubleValue);
convert_scalar_value!(f32, crate::FloatValue);
convert_scalar_value!(i32, crate::Int32Value);
convert_scalar_value!(i64, crate::Int64Value);
convert_scalar_value!(String, crate::StringValue);
convert_scalar_value!(u32, crate::UInt32Value);
convert_scalar_value!(u64, crate::UInt64Value);

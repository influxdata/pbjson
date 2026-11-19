use crate::Timestamp;
use std::fmt::Write;

use serde::Serialize;
use serde::de::Visitor;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

impl TryFrom<Timestamp> for OffsetDateTime {
    type Error = &'static str;
    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        let Timestamp { seconds, nanos } = value;

        if !(-62_135_596_800..=253_402_300_799).contains(&seconds)
            || !(0..1_000_000_000).contains(&nanos)
        {
            return Err("invalid or out-of-range datetime");
        }
        Self::from_unix_timestamp_nanos(i128::from(seconds) * 1_000_000_000 + i128::from(nanos))
            .map_err(|_| "invalid or out-of-range datetime")
    }
}

impl From<OffsetDateTime> for Timestamp {
    fn from(value: OffsetDateTime) -> Self {
        Self {
            seconds: value.unix_timestamp(),
            nanos: value.nanosecond() as i32,
        }
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let t: OffsetDateTime = (*self).try_into().map_err(serde::ser::Error::custom)?;
        let mut encoded = t
            .format(time::macros::format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second]"
            ))
            .map_err(serde::ser::Error::custom)?;
        // Preserve the existing UTC suffix and 0/3/6/9 fractional-digit output.
        let nanos = t.nanosecond();
        if nanos != 0 {
            let (fraction, width) = if nanos.is_multiple_of(1_000_000) {
                (nanos / 1_000_000, 3)
            } else if nanos.is_multiple_of(1_000) {
                (nanos / 1_000, 6)
            } else {
                (nanos, 9)
            };
            write!(encoded, ".{fraction:0width$}").map_err(serde::ser::Error::custom)?;
        }
        encoded.push_str("+00:00");
        serializer.serialize_str(&encoded)
    }
}

struct TimestampVisitor;

impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a date string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let d = OffsetDateTime::parse(s, &Rfc3339).map_err(serde::de::Error::custom)?;
        let timestamp = Timestamp::from(d);
        OffsetDateTime::try_from(timestamp).map_err(serde::de::Error::custom)?;
        Ok(timestamp)
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TimestampVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde::de::value::{BorrowedStrDeserializer, Error};

    #[test]
    fn test_date() {
        let datetime = OffsetDateTime::parse("2016-11-08T21:07:09+05:00", &Rfc3339).unwrap();
        let encoded = datetime.format(&Rfc3339).unwrap();
        assert_eq!(&encoded, "2016-11-08T21:07:09+05:00");

        let utc = datetime.to_offset(time::UtcOffset::UTC);
        let utc_encoded = "2016-11-08T16:07:09+00:00";

        let deserializer = BorrowedStrDeserializer::<'_, Error>::new(&encoded);
        let a: Timestamp = Timestamp::deserialize(deserializer).unwrap();
        assert_eq!(a.seconds, utc.unix_timestamp());
        assert_eq!(a.nanos, utc.nanosecond() as i32);

        let encoded = serde_json::to_string(&a).unwrap();
        assert_eq!(encoded, format!("\"{}\"", utc_encoded));
    }

    #[test]
    fn timestamp_boundaries_and_fractional_precision() {
        for (seconds, nanos, encoded) in [
            (-62_135_596_800, 0, "0001-01-01T00:00:00+00:00"),
            (-1, 0, "1969-12-31T23:59:59+00:00"),
            (-1, 1, "1969-12-31T23:59:59.000000001+00:00"),
            (-1, 999_999_999, "1969-12-31T23:59:59.999999999+00:00"),
            (0, 0, "1970-01-01T00:00:00+00:00"),
            (0, 30_000_000, "1970-01-01T00:00:00.030+00:00"),
            (0, 30_000, "1970-01-01T00:00:00.000030+00:00"),
            (
                253_402_300_799,
                999_999_999,
                "9999-12-31T23:59:59.999999999+00:00",
            ),
        ] {
            let timestamp = Timestamp { seconds, nanos };
            let native = OffsetDateTime::try_from(timestamp).unwrap();
            assert_eq!(Timestamp::from(native), timestamp);
            let json = serde_json::to_string(&timestamp).unwrap();
            assert_eq!(json, format!("\"{encoded}\""));
            assert_eq!(serde_json::from_str::<Timestamp>(&json).unwrap(), timestamp);
        }
    }

    #[test]
    fn invalid_timestamp_fields_are_rejected() {
        for timestamp in [
            Timestamp {
                seconds: -62_135_596_801,
                nanos: 999_999_999,
            },
            Timestamp {
                seconds: 253_402_300_800,
                nanos: 0,
            },
            Timestamp {
                seconds: i64::MIN,
                nanos: 0,
            },
            Timestamp {
                seconds: i64::MAX,
                nanos: 0,
            },
            Timestamp {
                seconds: 0,
                nanos: -1,
            },
            Timestamp {
                seconds: 0,
                nanos: 1_000_000_000,
            },
        ] {
            assert!(OffsetDateTime::try_from(timestamp).is_err());
            assert!(serde_json::to_string(&timestamp).is_err());
        }
        for encoded in [
            "0000-12-31T23:59:59Z",
            "0001-01-01T00:00:00+01:00",
            "9999-12-31T23:59:59-01:00",
            "2024-02-30T00:00:00Z",
        ] {
            assert!(serde_json::from_str::<Timestamp>(&format!("\"{encoded}\"")).is_err());
        }
    }

    #[test]
    fn offsets_before_the_epoch_keep_nonnegative_nanos() {
        let expected = Timestamp {
            seconds: -1,
            nanos: 123_456_789,
        };
        for encoded in [
            "1969-12-31T23:59:59.123456789Z",
            "1970-01-01T00:59:59.123456789+01:00",
            "1969-12-31T18:59:59.123456789-05:00",
        ] {
            let timestamp = serde_json::from_str::<Timestamp>(&format!("\"{encoded}\"")).unwrap();
            assert_eq!(timestamp, expected);
            assert_eq!(
                Timestamp::from(OffsetDateTime::parse(encoded, &Rfc3339).unwrap()),
                expected
            );
        }
    }
}

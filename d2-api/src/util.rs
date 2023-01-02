pub mod serde {
    use serde::de::IntoDeserializer;
    use serde::Deserialize;

    pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        let opt = Option::<String>::deserialize(de)?;
        let opt = opt.as_ref().map(String::as_str);
        match opt {
            None | Some("") => Ok(None),
            Some(s) => T::deserialize(s.into_deserializer()).map(Some),
        }
    }

    pub fn zero_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        let opt = Option::<i64>::deserialize(de)?;
        match opt {
            None | Some(0) => Ok(None),
            Some(s) => T::deserialize(s.into_deserializer()).map(Some),
        }
    }

    pub fn zero_date_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        let opt = Option::<String>::deserialize(de)?;
        let opt = opt.as_ref().map(String::as_str);
        match opt {
            None | Some("0001-01-01T00:00:00Z") => Ok(None),
            Some(s) => T::deserialize(s.into_deserializer()).map(Some),
        }
    }

    pub mod long {
        use std::fmt::Display;
        use std::str::FromStr;

        use serde::{de, Deserialize, Deserializer, Serializer};

        pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: Display,
            S: Serializer,
        {
            serializer.collect_str(value)
        }

        pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where
            T: FromStr,
            T::Err: Display,
            D: Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(de::Error::custom)
        }
    }
}

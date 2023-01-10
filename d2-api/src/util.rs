pub mod serde {
    use serde::de::IntoDeserializer;
    use serde::Deserialize;

    pub mod empty_string_as_none {
        use serde::de::IntoDeserializer;
        use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
        use std::fmt::Display;

        pub fn deserialize<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
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

        pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
            where
                T: Serialize,
                S: Serializer,
        {
            match value {
                None => serializer.serialize_str(""),
                Some(v) => serializer.serialize_some(&v),
            }
        }
    }


    pub mod zero_as_none {
        use serde::de::IntoDeserializer;
        use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
        use std::fmt::Display;

        pub fn deserialize<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
        where
            D: serde::Deserializer<'de>,
            T: serde::Deserialize<'de>,
        {
            let opt = Option::<i32>::deserialize(de)?;
            match opt {
                None | Some(0) => Ok(None),
                Some(s) => T::deserialize(s.into_deserializer()).map(Some),
            }
        }

        pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: Serialize,
            S: Serializer,
        {
            match value {
                None => serializer.serialize_i32(0),
                Some(v) => serializer.serialize_some(&v),
            }
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

    pub mod default_as_none {
        use std::fmt::Display;
        use std::str::FromStr;

        use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

        pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: Serialize + Default,
            S: Serializer,
        {
            match *value {
                Some(ref v) => serializer.serialize_some(v),
                None => serializer.serialize_some(&T::default()),
            }
        }

        pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
        where
            T: serde::Deserialize<'de> + Default + Eq,
            D: Deserializer<'de>,
        {
            let t = T::deserialize(deserializer)?;
            if t == T::default() {
                Ok(None)
            } else {
                Ok(Some(t))
            }
        }
    }
}

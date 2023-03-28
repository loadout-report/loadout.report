use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

/// A newtype wrapper around an `i64` that represents an ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id<T>(pub i128, PhantomData<T>);

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        Ok(Id(Deserialize::deserialize(deserializer)?, PhantomData))
    }
}

impl<T> Id<T> {
    pub fn new(id: i128) -> Id<T> {
        Id(id, PhantomData)
    }
}

/// A trait that allows an `Id` to be resolved to a concrete type.
pub trait ResolveTo<T> {
    type Error;
    fn resolve(id: &Id<T>) -> Result<T, Self::Error>;
}

/// A method wrapper around `T::resolve`.
impl<T: ResolveTo<T>> Id<T>
{
    pub fn resolve(&self) -> Result<T, T::Error> {
        T::resolve(self)
    }
}

pub trait AsyncResolveTo<T> {
    type Error;
    async fn async_resolve(id: &Id<T>) -> Result<T, Self::Error>;
}

impl<T: AsyncResolveTo<T>> Id<T> {
    pub async fn async_resolve(&self) -> Result<T, T::Error> {
        T::async_resolve(self).await
    }
}

#[cfg(test)]
mod test {
    use std::future::Future;
    use serde::{Deserialize, Serialize};

    use crate::id::{AsyncResolveTo, Id, ResolveTo};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Resource {
        pub org_unit_id: Id<OrgUnit>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OrgUnit {
        pub id: Id<OrgUnit>,
    }

    impl ResolveTo<OrgUnit> for OrgUnit {
        type Error = ();
        fn resolve(id: &Id<OrgUnit>) -> Result<OrgUnit, ()> {
            Ok(OrgUnit {
                id: id.to_owned(),
            })
        }
    }

    impl AsyncResolveTo<OrgUnit> for OrgUnit {
        type Error = ();
        async fn async_resolve(id: &Id<OrgUnit>) -> Result<OrgUnit, ()> {
            Ok(OrgUnit {
                    id: id.to_owned(),
                })
        }
    }

    #[test]
    pub fn test_compile() {
        let id: Id<OrgUnit> = Id::new(1);
        let resource = Resource {
            org_unit_id: id,
        };
        let org_unit = resource.org_unit_id.resolve().unwrap();
    }

    #[test]
    pub fn test_deserialise() {
        let json = r#"{"orgUnitId": 1}"#;
        let resource: Resource = serde_json::from_str(json).unwrap();
        let org_unit = resource.org_unit_id.resolve().unwrap();
    }

    #[tokio::test]
    pub async fn test_async_resolve() {
        let id: Id<OrgUnit> = Id::new(1);
        let resource = Resource {
            org_unit_id: id,
        };
        let org_unit = resource.org_unit_id.async_resolve().await.unwrap();
    }
}

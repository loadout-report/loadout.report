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
impl<T: ResolveTo<T>> Id<T> {
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

pub trait Resolver<T> {
    type Error;
    fn resolve(&self, id: &Id<T>) -> Result<T, Self::Error>;
}

pub trait AsyncResolver<T> {
    type Error;
    async fn async_resolve(&self, id: &Id<T>) -> Result<T, Self::Error>;
}

#[cfg(test)]
mod test {
    use std::future::Future;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Item {
        pub category_id: Id<Category>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Category {
        pub id: Id<Category>,
    }

    pub struct Client {

    }

    impl Resolver<Category> for Client {
        type Error = ();
        fn resolve(&self, id: &Id<Category>) -> Result<Category, ()> {
            Ok(Category {
                id: id.to_owned(),
            })
        }
    }

    impl AsyncResolver<Category> for Client {
        type Error = ();
        async fn async_resolve(&self, id: &Id<Category>) -> Result<Category, ()> {
            Ok(Category {
                id: id.to_owned(),
            })
        }
    }

    impl ResolveTo<Category> for Category {
        type Error = ();
        fn resolve(id: &Id<Category>) -> Result<Category, ()> {
            Ok(Category {
                id: id.to_owned(),
            })
        }
    }

    impl AsyncResolveTo<Category> for Category {
        type Error = ();
        async fn async_resolve(id: &Id<Category>) -> Result<Category, ()> {
            Ok(Category {
                id: id.to_owned(),
            })
        }
    }

    #[test]
    pub fn test_compile() {
        let id: Id<Category> = Id::new(1);
        let resource = Item {
            category_id: id,
        };
        let category = resource.category_id.resolve().unwrap();
    }

    #[test]
    pub fn test_deserialise() {
        let json = r#"{"orgUnitId": 1}"#;
        let resource: Item = serde_json::from_str(json).unwrap();
        let category = resource.category_id.resolve().unwrap();
    }

    #[tokio::test]
    pub async fn test_async_resolve() {
        let id: Id<Category> = Id::new(1);
        let resource = Item {
            category_id: id,
        };
        let category = resource.category_id.async_resolve().await.unwrap();
    }

    #[test]
    pub fn test_client_resolve() {
        let id: Id<Category> = Id::new(1);
        let resource = Item {
            category_id: id,
        };
        let client = Client{};
        let category = client.resolve(&resource.category_id).unwrap();
    }

    #[tokio::test]
    pub async fn test_client_async_resolve() {
        let id: Id<Category> = Id::new(1);
        let resource = Item {
            category_id: id,
        };
        let client = Client{};
        let category = client.async_resolve(&resource.category_id).await.unwrap();
    }
}

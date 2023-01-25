use std::collections::HashMap;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Spec {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub paths: Paths,
    pub components: Components,
    pub tags: Vec<Tag>,
    pub external_docs: Option<ExternalDocs>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub description: String,
    pub terms_of_service: String,
    pub contact: Contact,
    pub license: License,
    pub version: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Contact {
    pub name: String,
    pub url: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct License {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Server {
    pub url: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Paths(pub HashMap<String, Path>);

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Path {
    pub summary: String,
    pub description: String,
    pub get: Option<Operation>,
    pub post: Option<Operation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    // todo: impl
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    pub schemas: HashMap<String, Schema>,
    // responses: HashMap<String, Response>,
    // headers: HashMap<String, Header>,
    // security_schemes: HashMap<String, SecurityScheme>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
    #[serde(rename = "x-enum-values")]
    pub enum_values: Option<Vec<EnumValue>>,
    #[serde(rename = "x-enum-is-bitmask", default)]
    pub enum_is_bitmask: bool,
    #[serde(rename = "x-enum-reference")]
    pub enum_reference: Option<Box<Schema>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub format: Option<String>,
    /// properties if type == object
    pub properties: Option<HashMap<String, Schema>>,
    /// items if type == array
    pub items: Option<Box<Schema>>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    pub nullable: Option<bool>,
    /// in the case of a map
    pub additional_properties: Option<Box<Schema>>,
    #[serde(rename = "x-dictionary-key")]
    pub dictionary_key: Option<Box<Schema>>,
    #[serde(rename = "x-mapped-definition")]
    pub mapped_definition: Option<Box<Schema>>,
    pub all_of: Option<Vec<Schema>>,
    #[serde(rename = "x-mobile-manifest-name")]
    pub mobile_manifest_name: Option<String>,
    #[serde(rename = "x-destiny-component-type-dependency")]
    pub component_type_dependency: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EnumValue {
    pub numeric_value: String,
    pub identifier: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub description: String,
    pub external_docs: Option<ExternalDocs>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ExternalDocs {
    pub description: String,
    pub url: String,
}


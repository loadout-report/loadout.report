use criterion::criterion_group;

pub mod json;

criterion_group!(benches, json::deserialize_archive);

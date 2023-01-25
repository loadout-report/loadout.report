use serde::Deserialize;

#[derive(Deserialize)]
pub struct FetchPlayerOptions {
    pub(crate) cache: Option<bool>
}

#[derive(Deserialize)]
pub struct FetchLoadoutOptions {
    pub(crate) cache: Option<bool>
}

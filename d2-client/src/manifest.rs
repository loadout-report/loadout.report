use std::collections::HashMap;
use std::str::FromStr;
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::header::IF_MODIFIED_SINCE;
use reqwest::{header, StatusCode};

const LAST_MODIFIED_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT";

pub struct ManifestApi {
    pub(crate) client: super::D2Api,
}

impl ManifestApi {

}

#[cfg(test)]
mod tests {
    use tokio_test::assert_ok;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_fetch_content_paths() {
        let client = crate::D2Api::new("");
        for _ in 1..2 {
            let manifest = tokio_test::block_on(client.manifest().fetch_content_paths());
            assert_ok!(manifest);
        }

    }

}

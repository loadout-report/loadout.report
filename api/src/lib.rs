#![feature(type_alias_impl_trait)]
pub mod routes;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Debug)]
pub enum ApiError {
    Unknown,
}
impl warp::reject::Reject for ApiError {}

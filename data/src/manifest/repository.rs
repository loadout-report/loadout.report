
enum RepositoryErr {

}

pub trait Repository<T> {
    fn list() -> Vec<T>;
    fn get(id: &str) -> Option<T>;
}

use model::InventoryItem;

pub mod repository;
pub mod model;

pub trait Manifest {
    fn items() -> Box<dyn repository::Repository<InventoryItem>>;
}

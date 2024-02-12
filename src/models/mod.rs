pub mod waiters;
pub mod orders;
pub mod products;
pub mod tables;
pub mod categories;

pub trait CollectionName {
    fn collection_name() -> &'static str;
}
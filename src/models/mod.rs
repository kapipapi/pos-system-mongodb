pub mod waiters;
pub mod orders;
pub mod products;

pub trait CollectionName {
    fn collection_name() -> &'static str;
}
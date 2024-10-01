use std::any::type_name;

pub mod config;
pub mod crypting;
pub mod files;
pub mod network;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

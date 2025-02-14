use std::any::type_name;

pub mod cfg;
pub mod check;
pub mod net;

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

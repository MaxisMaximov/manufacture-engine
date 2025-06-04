use super::storage::gmStorage;
use std::any::Any;

pub trait Component: Any + Sized{
    type STORAGE: gmStorage<Self>;
    const ID: &'static str;
}
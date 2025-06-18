use super::storage::Storage;
use std::any::Any;

/// # Component trait
/// A trait identifying Components within the engine
/// 
/// `Storage` is anything implementing `Storage` trait
/// 
/// `ID` is what the component will be identified by in the World
/// 
/// ## WARNING
/// Make sure your Component ID does not collide with other IDs from other plugins
pub trait Component: Sized + 'static{
    type STORAGE: Storage<Self>;
    const ID: &'static str;
}
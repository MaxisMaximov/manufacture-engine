use std::ops::{Deref, DerefMut};

use super::comp::Component;

/// # Component Storage trait
/// Specifies some basic functions for the storage to do
pub trait Storage<T: Component>{
    /// Create a new specified Storage for this component
    fn new() -> Self;
    /// Insert a Component for an entity into this storage
    fn insert(&mut self, Index: usize, Comp: T);
    /// Remove the specified Entity's Component from this storage
    fn remove(&mut self, Index: usize);
    /// Get a reference to the specified Entity's Component from this storage
    fn get(&self, Index: &usize) -> Option<&T>;
    /// Get a mutable reference to the specified ENtity's Component from this storage
    fn get_mut(&mut self, Index: &usize) -> Option<&mut T>;
}

/// # Storage trait Container
/// Wraps a Component's `STORAGE` to safely store it within the world
/// 
/// It is required as compound generics *(`T: Trait<U>`)* aren't supported yet
/// 
/// To get the underlying `STORAGE`, use a dereference
pub struct StorageContainer<T: Component>{
    inner: T::STORAGE
}
impl<T: Component> Deref for StorageContainer<T>{
    type Target = T::STORAGE;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
} 
impl<T: Component> DerefMut for StorageContainer<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl<T: Component> StorageContainer<T>{
    pub fn new() -> Self{
        Self{
            inner: T::STORAGE::new()
        }
    }

    pub fn comp_id(&self) -> &'static str{
        T::ID
    }
}

/// # Storage Container Wrapper trait
/// A dyn-compatible wrapper for StorageContainer for the World to store with
/// 
/// Provides ability to remove a component of the specified entity for easier cleanup,  
/// as well as Downcast methods to get the underlying Containers
pub trait StorageWrapper{
    /// Remove a specified Entity's component from this storage
    fn remove(&mut self, Index: usize);
    /// Get the underlying Container's Component ID
    fn comp_id(&self) -> &'static str;
}

impl<T: Component> StorageWrapper for StorageContainer<T>{
    fn remove(&mut self, Index: usize){
        Storage::remove(&mut self.inner, Index);
    }

    fn comp_id(&self) -> &'static str {
        T::ID
    }
}

impl dyn StorageWrapper{
    /// Downcast to a reference of a StorageContainer of the `T` component type
    /// 
    /// Returns None if the ID of the `T` component does not match the underlying Container's Component ID
    pub fn downcast_ref<T: Component>(&self) -> Option<&StorageContainer<T>>{
        if T::ID == self.comp_id(){
            // SAFETY: We check if the Component IDs match on the line above
            Some(unsafe {
                &*(self as *const dyn StorageWrapper as *const StorageContainer<T>)
            })
        }else{
            None
        }
    }

    /// Downcast to a mutable reference of a StorageContainer of the `T` component type
    /// 
    /// Returns None if the ID of the `T` component does not match the underlying Container's Component ID
    pub fn downcast_mut<T: Component>(&mut self) -> Option<&mut StorageContainer<T>>{
        if T::ID == self.comp_id(){
            Some(unsafe {
                &mut *(self as *mut dyn StorageWrapper as *mut StorageContainer<T>)
            })
        }else{
            None
        }
    }
}
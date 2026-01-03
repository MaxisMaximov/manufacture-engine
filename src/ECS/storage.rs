use std::ops::{Deref, DerefMut};


use super::comp::Component;
use super::entity::Token;

/// # Component Storage trait
/// Specifies some basic functions for the storage to do
pub trait Storage<T: Component>{
    /// Create a new specified Storage for this Component
    fn new() -> Self;

    /// Insert a Component for the specified Entity into this Storage
    fn insert(&mut self, id: usize, comp: T);
    /// Insert the Component for the Entity referenced by the Token into this Storage
    /// 
    /// It's recommended to ensure the Token is valid beforehand
    fn insert_with_token(&mut self, token: &Token, comp: T){
        if !token.valid(){
            return
        }
        self.insert(token.id(), comp);
    }

    /// Remove the specified Entity's Component from this Storage
    fn remove(&mut self, id: &usize);
    /// Remove the Component from the Entity referenced by the Token from this Storage
    /// 
    /// It's recommended to ensure the Token is valid beforehand
    fn remove_with_token(&mut self, id: &Token){
        if !id.valid(){
            return
        }
        self.remove(&id.id());
    }

    /// Get a reference to the specified Entity's Component from this storage
    fn get(&self, id: &usize) -> Option<&T>;
    /// Get a reference to the Component from this storage of the Entity refereced by the Token
    /// 
    /// It's recommended to ensure the Token is valid beforehand
    fn get_from_token(&self, token: &Token) -> Option<&T>{
        if !token.valid(){
            return None
        }
        self.get(&token.id())
    }
    
    /// Get a mutable reference to the specified Entity's Component from this storage
    fn get_mut(&mut self, id: &usize) -> Option<&mut T>;
    /// Get a mutable reference to the Component from this storage of the Entity refereced by the Token
    /// 
    /// It's recommended to ensure the Token is valid beforehand
    fn get_from_token_mut(&mut self, token: &Token) -> Option<&mut T>{
        if !token.valid(){
            return None
        }
        self.get_mut(&token.id())
    }
}

/// # Storage trait Container
/// Wraps a Component's `STORAGE` to safely store it within the World
/// 
/// It is required as compound generics *(`U: Trait_A, T: Trait<U>`)* aren't supported yet
/// 
/// To get the underlying `STORAGE`, use a dereference
pub struct StorageContainer<T: Component>{
    inner: T::STORAGE
}
impl<T: Component> StorageContainer<T>{
    /// Create a new wrapper for a Component's Storage
    pub fn new() -> Self{
        Self{
            inner: T::STORAGE::new()
        }
    }
    /// Get the underlying Storage's Component ID
    pub fn comp_id(&self) -> &'static str{
        T::ID
    }
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

/// # Storage Container Wrapper trait
/// A dyn-compatible wrapper for StorageContainer for the World to store with
/// 
/// Provides ability to remove a Component of the specified entity for easier cleanup,  
/// as well as Downcast methods to get the underlying Containers
pub trait StorageWrapper{
    /// Remove a specified Entity's Component from this storage
    fn remove(&mut self, id: usize);
    /// Get the underlying Container's Component ID
    fn comp_id(&self) -> &'static str;
}

impl<T: Component> StorageWrapper for StorageContainer<T>{
    fn remove(&mut self, id: usize){
        self.inner.remove(&id);
    }

    fn comp_id(&self) -> &'static str {
        T::ID
    }
}

impl dyn StorageWrapper{
    /// Downcast to a reference of a StorageContainer of the `T` Component type
    /// 
    /// Returns None if the ID of the `T` Component does not match the underlying Container's Component ID
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

    /// Downcast to a mutable reference of a StorageContainer of the `T` Component type
    /// 
    /// Returns None if the ID of the `T` Component does not match the underlying Container's Component ID
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
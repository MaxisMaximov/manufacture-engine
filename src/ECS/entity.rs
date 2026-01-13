use std::collections::HashSet;

use super::comp::Component;
use super::storage::Storage;
use super::world::World;

type Hash = u32;

/// # Entity struct
/// Identifies a single Entity within the World
/// 
/// Stores it's own ID and Hash for collision checks, 
/// as well as what Components it has on the given frame
pub struct Entity{
    id: usize,
    hash: Hash
}
impl Entity{
    /// Create a new Entity with given ID
    pub(crate) fn new(id: usize) -> Self{
        Self{
            id,
            hash: rand::random()
        }
    }
    /// Get a Token for this Entity
    pub fn get_token(&self) -> Token{
        Token{
            id: self.id,
            hash: self.hash,
            valid: true,
        }
    }
    /// Read this Entity's ID
    pub fn id(&self) -> usize{
        self.id
    }
    /// Read this Entity's Hash
    pub fn hash(&self) -> Hash{
        self.hash
    }
}

/// # Entity Token
/// A "reference" to a specific Entity within the World
/// 
/// Holds the Entity's ID, Hash, and whether it's a valid Token
/// 
/// Tokens whose Entities no longer exist are invalid  
/// This is checked through the Hash value
#[derive(Clone, Copy)]
pub struct Token{
    id: usize,
    hash: Hash,
    valid: bool
}
impl Token{
    /// Read the tracked Entity's ID
    pub fn id(&self) -> usize{
        self.id
    }
    /// Read the tracked Entity's Hash
    pub fn hash(&self) -> Hash{
        self.hash
    }
    /// Read if the Token is valid
    pub fn valid(&self) -> bool{
        self.valid
    }
    /// Check if the Token is still valid within the World
    /// 
    /// Updates it's own `valid` flag and returns it.  
    /// If the IDs don't match, it doesn't do anything
    pub fn validate(&mut self, entity: &Entity) -> bool{
        if self.id == entity.id(){
            self.valid = self.hash == entity.hash();
        }
        self.valid
    }
}

/// # Entity Builder
/// A safe and easy way to contruct a new Entity in the World
#[must_use]
pub struct EntityBuilder<'a>{
    pub(crate) entity: Token,
    pub(crate) world_ref: &'a mut World,
    pub(crate) components: HashSet<&'static str>
}
impl<'a> EntityBuilder<'a>{
    /// Add a specified Component to the current Entity
    pub fn with<T: Component>(mut self, comp: T) -> Self{
        self.world_ref.fetch_mut::<T>().insert(self.entity.id(), comp);
        self.components.insert(T::ID);
        self
    }
    /// Get the list of Components added to the Entity
    pub fn components(&self) -> &HashSet<&'static str>{
        &self.components
    }
    /// Get the ID of the entity currently being built
    pub fn id(&self) -> usize{
        self.entity.id()
    }
    /// Get the Token for the entity currently being built
    pub fn get_token(&self) -> Token{
        self.entity
    }
    /// "Finish" the building process
    pub fn finish(self){}
}
use std::cell::{RefCell, Ref, RefMut};
use std::collections::{HashMap, BTreeMap, BTreeSet};

use super::events::*;
use super::resource::*;
use super::comp::*;
use super::storage::*;
use super::fetch::*;
use super::entity::*;
use super::commands::CommandWrapper;

/// # ECS World
/// Stores all the data within ECS:
/// - Entities
/// - Components
/// - Resources
/// - Events
/// - Triggers
/// - Commands
/// 
/// Provides methods for registering, removing and accessing the data
pub struct World{
    entities: BTreeMap<usize, Entity>,
    next_free: BTreeSet<usize>,
    components: HashMap<&'static str, RefCell<Box<dyn StorageWrapper>>>,
    resources: HashMap<&'static str, RefCell<Box<dyn ResourceWrapper>>>,
    events: EventBufferMap,
    triggers: RefCell<Vec<&'static str>>,
    commands: RefCell<Vec<Box<dyn CommandWrapper>>>
}
impl World{
    /// Create a new, empty World
    pub fn new() -> Self{
        Self{
            entities: BTreeMap::new(),
            next_free: BTreeSet::new(),
            components: HashMap::new(),
            resources: HashMap::new(),
            events: EventBufferMap::new(),
            triggers: RefCell::new(Vec::new()),
            commands: RefCell::new(Vec::new())
        }
    }

    /// Get a reference to `T` component storage
    pub fn fetch<'a, T>(&'a self) -> Fetch<'a, T> where T: Component{
        // Check if we have such component in the first place
        if !self.components.contains_key(T::ID){
            // There's no way to signify missing components yet, so we panic for now
            panic!("ERROR: Tried to fetch an unregistered component: {}", T::ID)
        }

        Ref::map(
            // Unwrap: We have a check for an invalid Component earlier
            self.components.get(T::ID).unwrap().borrow(), 
            |idkfa| &**idkfa.downcast_ref::<T>().unwrap())
    }
    /// Get a mutable reference to `T` component storage
    pub fn fetch_mut<'a, T>(&'a self) -> FetchMut<'a, T> where T: Component{
        // Same as above
        if !self.components.contains_key(T::ID){
            panic!("ERROR: Tried to fetch an unregistered component: {}", T::ID)
        }

        RefMut::map(
            self.components.get(T::ID).unwrap().borrow_mut(), 
            |idkfa| &mut **idkfa.downcast_mut::<T>().unwrap())
    }

    /// Get a reference to `T` resource
    pub fn fetch_res<'a, T>(&'a self) -> FetchRes<'a, T> where T: Resource{
        // Check if we have such Resource registered already
        if !self.resources.contains_key(T::ID){
            // Same as with Component fetch
            panic!("ERROR: Tried to fetch an unregistered resource: {}", T::ID)
        }

        Ref::map(
            self.resources.get(T::ID).unwrap().borrow(), 
            |idkfa| idkfa.downcast_ref::<T>().unwrap())
    }
    /// Get a reference to `T` resource
    pub fn fetch_res_mut<'a, T>(&'a self) -> FetchResMut<'a, T> where T: Resource{
        // Same as above
        if !self.resources.contains_key(T::ID){
            panic!("ERROR: Tried to fetch an unregistered resource: {}", T::ID)
        }

        RefMut::map(
            self.resources.get(T::ID).unwrap().borrow_mut(), 
            |idkfa| idkfa.downcast_mut::<T>().unwrap())
    }

    /// Get a reader for `T` event
    /// 
    /// The reader accesses the events sent in the previous frame
    pub fn get_event_reader<'a, T>(&'a self) -> EventReader<'a, T> where T: Event{
        self.events.get_reader()
    }
    /// Get a writer for `T` event
    /// 
    /// The writer sends events for the current frame
    pub fn get_event_writer<'a, T>(&'a self) -> EventWriter<'a, T> where T: Event{
        // SAFETY: Same as above
        self.events.get_writer()
    }

    /// Get writer for System Triggers
    pub fn get_trigger_writer(&self) -> TriggerWriter{
        self.triggers.borrow_mut()
    }

    /// Register `T` component in this World
    pub fn register_comp<T>(&mut self) where T: Component{
        if self.components.contains_key(T::ID){
            panic!("ERROR: Attempted to override an existing component: {}", T::ID)
        }

        self.components.insert(
            T::ID, 
            RefCell::new(Box::new(StorageContainer::<T>::new())));
    }
    /// Remove the `T` component from this World
    /// 
    /// Every Entity with this component will have that component dropped
    pub fn deregister_comp<T>(&mut self) where T: Component{
        self.components.remove(T::ID);
        for entity in self.entities.values_mut(){
            entity.components.remove(T::ID);
        }
    }

    /// Register a `T` resource in this World
    pub fn register_res<T>(&mut self) where T: Resource{
        if self.resources.contains_key(T::ID){
            panic!("ERROR: Attempted to override an existing resource: {}", T::ID)
        }

        self.resources.insert(T::ID, RefCell::new(Box::new(T::new())));
    }
    /// Remove the `T` resource from this World
    pub fn deregister_res<T>(&mut self) where T: Resource{
        self.resources.remove(T::ID);
    }

    /// Register a `T` event in this World
    pub fn register_event<T>(&mut self) where T: Event{
        self.events.register::<T>();
    }
    /// Remove the `T` event from this world
    /// 
    /// The respective Read and Write queues will get removed from EventMap
    pub fn deregister_event<T>(&mut self) where T: Event{
        self.events.deregister::<T>();
    }

    /// Spawn a new entity
    /// 
    /// Returns a Builder that monitors the construction of the Entity
    pub fn spawn(&mut self) -> EntityBuilder{
        EntityBuilder{
            entity: {
                let next_id = self.next_free.pop_first().unwrap_or(self.entities.len());
                self.entities.insert(next_id, Entity::new(next_id));
                next_id
            },
            world_ref: self,
        }
    }
    /// Despawn the given Entity
    /// 
    /// This will drop all of the Entity's components
    pub fn despawn(&mut self, Id: usize) -> Result<(), ()>{
        match self.entities.remove(&Id){
            Some(_) => {
                for storage in self.components.values_mut(){
                    storage.borrow_mut().as_mut().remove(Id);
                }
                return Ok(())
            }
            None => {Err(())}
        }
    }
    
    /// Get Writer for the Command Queue
    pub fn get_command_writer<'a>(&'a self) -> CommandWriter<'a>{
        RefMut::map(
            self.commands.borrow_mut(), 
            |idkfa| idkfa)
    }

    /// Swap buffers of EventMap
    pub fn swap_event_buffers(&mut self){
        self.events.swap_buffers();
    }

    /// Take the full Trigger queue
    /// 
    /// This will initialize a new queue in it's place
    pub fn take_triggers(&mut self) -> Vec<&'static str>{
        self.triggers.take()
    }
    /// Take the full Command queue
    /// 
    /// This will initialize a new queue in it's place
    pub fn take_commands(&mut self) -> Vec<Box<dyn CommandWrapper>>{
        self.commands.take()
    }

    /// Validate an Entity Token
    /// 
    /// Returns False if the Entity no longer exists or the Hashes are not the same
    pub fn validate_token(&self, Token: &mut Token) -> bool{
        match self.entities.get(&Token.id()){
            Some(entity) => entity.hash() == Token.hash(),
            None => false,
        }
    }

    /// Get quick info about a given Entity
    pub fn get_entity(&self, Id: &usize) -> Option<&Entity>{
        self.entities.get(Id)
    }
    /// Get quick info about a given entity to change it
    pub fn get_entity_mut(&mut self, Id: &usize) -> Option<&mut Entity>{
        self.entities.get_mut(Id)
    }

    /// Get the entities within the World
    pub fn get_entities(&self) -> &BTreeMap<usize, Entity>{
        &self.entities
    }
}
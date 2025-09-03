use std::cell::{RefCell, Ref, RefMut, UnsafeCell};
use std::collections::{HashMap, BTreeMap, BTreeSet};

use super::events::*;
use super::resource::*;
use super::comp::*;
use super::storage::*;
use super::fetch::*;
use super::entity::*;
use super::commands::CommandWrapper;

pub struct World{
    entities: BTreeMap<usize, Entity>,
    next_free: BTreeSet<usize>,
    components: HashMap<&'static str, RefCell<Box<dyn StorageWrapper>>>,
    resources: HashMap<&'static str, RefCell<Box<dyn ResourceWrapper>>>,
    events: UnsafeCell<EventMap>,
    triggers: RefCell<Vec<&'static str>>,
    commands: RefCell<Vec<Box<dyn CommandWrapper>>>
}
impl World{

    pub fn new() -> Self{
        Self{
            entities: BTreeMap::new(),
            next_free: BTreeSet::new(),
            components: HashMap::new(),
            resources: HashMap::new(),
            events: UnsafeCell::new(EventMap::new()),
            triggers: RefCell::new(Vec::new()),
            commands: RefCell::new(Vec::new())
        }
    }

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
    pub fn fetch_mut<'a, T>(&'a self) -> FetchMut<'a, T> where T: Component{
        // Same as above
        if !self.components.contains_key(T::ID){
            panic!("ERROR: Tried to fetch an unregistered component: {}", T::ID)
        }

        RefMut::map(
            self.components.get(T::ID).unwrap().borrow_mut(), 
            |idkfa| &mut **idkfa.downcast_mut::<T>().unwrap())
    }

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
    pub fn fetch_res_mut<'a, T>(&'a self) -> FetchResMut<'a, T> where T: Resource{
        // Same as above
        if !self.resources.contains_key(T::ID){
            panic!("ERROR: Tried to fetch an unregistered resource: {}", T::ID)
        }

        RefMut::map(
            self.resources.get(T::ID).unwrap().borrow_mut(), 
            |idkfa| idkfa.downcast_mut::<T>().unwrap())
    }

    pub fn get_event_reader<'a, T>(&'a self) -> EventReader<'a, T> where T: Event{
        // SAFETY: Right now the entirety of the engine is single-threaded
        // So we don't have to worry about two systems on sepparate threads
        // colliding when they send/receive a new event
        //
        // This is because `EventMap` initializes a new queue for an event 
        // if the queue for the current frame doesn't yet exist
        // THAT, requires a mutable borrow
        //
        // TODO: Remove the Unsafe
        unsafe{self.events.get().as_mut().unwrap().get_reader()}
    }
    pub fn get_event_writer<'a, T>(&'a self) -> EventWriter<'a, T> where T: Event{
        // SAFETY: Same as above
        unsafe{self.events.get().as_mut().unwrap().get_writer()}
    }

    pub fn get_trigger_writer(&self) -> RefMut<'_, Vec<&'static str>>{
        self.triggers.borrow_mut()
    }

    pub fn register_comp<T>(&mut self) where T: Component{
        if self.components.contains_key(T::ID){
            panic!("ERROR: Attempted to override an existing component: {}", T::ID)
        }

        self.components.insert(
            T::ID, 
            RefCell::new(Box::new(StorageContainer::<T>::new())));
    }
    pub fn deregister_comp<T>(&mut self) where T: Component{
        self.components.remove(T::ID);
        for entity in self.entities.values_mut(){
            entity.components.remove(T::ID);
        }
    }

    pub fn register_res<T>(&mut self) where T: Resource{
        if self.resources.contains_key(T::ID){
            panic!("ERROR: Attempted to override an existing resource: {}", T::ID)
        }

        self.resources.insert(T::ID, RefCell::new(Box::new(T::new())));
    }
    pub fn deregister_ref<T>(&mut self) where T: Resource{
        self.resources.remove(T::ID);
    }

    pub fn register_event<T>(&mut self) where T: Event{
        self.events.get_mut().register::<T>();
    }
    pub fn deregister_event<T>(&mut self) where T: Event{
        self.events.get_mut().deregister::<T>();
    }

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
    
    pub fn get_command_writer<'a>(&'a self) -> CommandWriter<'a>{
            RefMut::map(
                self.commands.borrow_mut(), 
                |idkfa| idkfa)
    }

    pub fn exec_commands(&mut self){

        loop{
            let idkfa = self.commands.borrow_mut().pop();
            match idkfa{
                Some(mut command) => command.execute(self),
                None => break,
            }
        }
    }

    pub fn end_tick(&mut self){
        self.exec_commands();
        self.events.get_mut().swap_buffers();
    }

    pub fn validate_token(&self, Token: &mut Token) -> bool{
        match self.entities.get(&Token.id()){
            Some(entity) => entity.hash() == Token.hash(),
            None => false,
        }
    }

    pub fn get_entity(&self, Id: &usize) -> Option<&Entity>{
        self.entities.get(Id)
    }
    pub fn get_entity_mut(&mut self, Id: &usize) -> Option<&mut Entity>{
        self.entities.get_mut(Id)
    }
}
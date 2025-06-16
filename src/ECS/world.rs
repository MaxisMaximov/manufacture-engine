use std::{cell::{Ref, RefMut, UnsafeCell}, collections::BTreeSet};

use super::*;

use events::*;
use resource::*;
use comp::*;
use builders::gmObjBuilder;
use storage::*;
use fetch::*;
use entity::*;
use commands::gmCommand;

pub struct gmWorld{
    gmObjs: BTreeMap<usize, Entity>,
    next_free: BTreeSet<usize>,
    components: HashMap<&'static str, RefCell<Box<dyn StorageWrapper>>>,
    resources: HashMap<&'static str, RefCell<Box<dyn ResourceWrapper>>>,
    events: UnsafeCell<EventMap>,
    commands: RefCell<Vec<Box<dyn gmCommand>>>
}
impl gmWorld{

    pub fn new() -> Self{
        Self{
            gmObjs: BTreeMap::new(),
            next_free: BTreeSet::new(),
            components: HashMap::new(),
            resources: HashMap::new(),
            events: UnsafeCell::new(EventMap::new()),
            commands: RefCell::new(Vec::new())
        }
    }

    pub fn fetch<'a, T>(&'a self) -> Fetch<'a, T> where T: Component + 'static{
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
    pub fn fetchMut<'a, T>(&'a self) -> FetchMut<'a, T> where T: Component + 'static{
        // Same as above
        if !self.components.contains_key(T::ID){
            panic!("ERROR: Tried to fetch an unregistered component: {}", T::ID)
        }

        RefMut::map(
            self.components.get(T::ID).unwrap().borrow_mut(), 
            |idkfa| &mut **idkfa.downcast_mut::<T>().unwrap())
    }

    pub fn fetchRes<'a, T>(&'a self) -> FetchRes<'a, T> where T: Resource + 'static{
        // Check if we have such Resource registered already
        if !self.resources.contains_key(T::ID){
            // Same as with Component fetch
            panic!("ERROR: Tried to fetch an unregistered resource: {}", T::ID)
        }

        Ref::map(
            self.resources.get(T::ID).unwrap().borrow(), 
            |idkfa| idkfa.downcast_ref::<T>().unwrap())
    }
    pub fn fetchResMut<'a, T>(&'a self) -> FetchResMut<'a, T> where T: Resource + 'static{
        // Same as above
        if !self.resources.contains_key(T::ID){
            panic!("ERROR: Tried to fetch an unregistered resource: {}", T::ID)
        }

        RefMut::map(
            self.resources.get(T::ID).unwrap().borrow_mut(), 
            |idkfa| idkfa.downcast_mut::<T>().unwrap())
    }

    pub fn fetchEventReader<'a, T>(&'a self) -> EventReader<'a, T> where T: Event + 'static{
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
    pub fn fetchEventWriter<'a, T>(&'a self) -> EventWriter<'a, T> where T: Event + 'static{
        // SAFETY: Same as above
        unsafe{self.events.get().as_mut().unwrap().get_writer()}
    }

    pub fn registerComp<T>(&mut self) where T: Component + 'static{
        if !self.components.contains_key(T::ID){
            panic!("ERROR: Attempted to override an existing component: {}", T::ID)
        }

        self.components.insert(
            T::ID, 
            RefCell::new(Box::new(StorageContainer::<T>::new())));
    }
    pub fn unRegisterComp<T>(&mut self) where T: Component + 'static{
        self.components.remove(T::ID);
    }

    pub fn registerRes<T>(&mut self) where T: Resource + 'static{
        if !self.resources.contains_key(T::ID){
            panic!("ERROR: Attempted to override an existing resource: {}", T::ID)
        }

        self.resources.insert(T::ID, RefCell::new(Box::new(T::new())));
    }
    pub fn unRegisterRes<T>(&mut self) where T: Resource + 'static{
        self.resources.remove(T::ID);
    }

    pub fn registerEvent<T>(&mut self) where T: Event + 'static{
        self.events.get_mut().register::<T>();
    }
    pub fn unRegisterEvent<T>(&mut self) where T: Event + 'static{
        self.events.get_mut().deregister::<T>();
    }

    pub fn createGmObj(&mut self) -> gmObjBuilder{
        gmObjBuilder::new(
             {
                let next_id = self.next_free.pop_first().unwrap_or(self.gmObjs.len());
                self.gmObjs.insert(next_id, Entity::new(next_id));
                next_id
            },
            self,
        )
    }
    pub fn deleteGmObj(&mut self, IN_id: usize) -> Result<(), ()>{
        match self.gmObjs.remove(&IN_id){
            Some(_) => {
                for COMP in self.components.values_mut(){
                    COMP.borrow_mut().as_mut().remove(IN_id);
                }
                return Ok(())
            }
            None => {Err(())}
        }
    }
    
    pub fn fetchCommandWriter<'a>(&'a self) -> CommandWriter<'a>{
            RefMut::map(
                self.commands.borrow_mut(), 
                |idkfa| idkfa)
    }

    pub fn commandsExec(&mut self){

        loop{
            let idkfa = self.commands.borrow_mut().pop();
            match idkfa{
                Some(COMMAND) => COMMAND.execute(self),
                None => break,
            }
        }
    }

    pub fn endTick(&mut self){
        self.commandsExec();
        self.events.get_mut().swap_buffers();
    }

    pub fn validateToken(&self, Token: &mut entity::Token) -> bool{
        match self.gmObjs.get(&Token.id()){
            Some(GMOBJ) => GMOBJ.hash == Token.hash(),
            None => false,
        }
    }
}
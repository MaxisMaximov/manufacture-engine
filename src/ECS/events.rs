use std::{cell::{Ref, RefMut}, collections::{HashSet, VecDeque}};

use super::*;
use fetch::*;

pub trait gmEvent{
    fn EVENT_ID() -> &'static str where Self:Sized;
}

/// # Event Map
/// A Double Hashmap Buffer queue of Events
/// 
/// Maintains a Registry of Events to prevent illegal overrides and reading/writing non-existent Events
/// 
/// Buffers switch at the end of every Tick, clearing the previously Read-Only buffer
pub struct EventMap{
    registry: HashSet<&'static str>,
    active_buffer: HashMap<&'static str, RefCell<Box<dyn Any>>>,
    alt_buffer: HashMap<&'static str, RefCell<Box<dyn Any>>>,
}
impl EventMap{
    /// Create a new, empty EventMap
    pub fn new() -> Self{
        Self{
            registry: HashSet::new(),
            active_buffer: HashMap::new(),
            alt_buffer: HashMap::new(),
        }
    }

    /// Register an event
    pub fn register<T: gmEvent>(&mut self){
        if self.registry.contains(T::EVENT_ID()){
            // Events CANNOT share IDs because systems expect a specific type
            // This ain't OOP, we can't replace one struct with another and expect it to go the same
            panic!("ERROR: Conflicting Event IDs: {}", T::EVENT_ID())
        }
        self.registry.insert(T::EVENT_ID());
    }
    /// Deregister an event
    /// 
    /// This also clears the respective Event's Queues from both buffers
    pub fn deregister<T: gmEvent>(&mut self){
        self.registry.remove(T::EVENT_ID());
        // Remove those events from the Map as they're no longer valid
        self.active_buffer.remove(T::EVENT_ID());
        self.alt_buffer.remove(T::EVENT_ID());
    }

    pub(super) fn swap_buffers(&mut self){
        // Clear active buffer to (kinda) free up memory
        self.active_buffer.clear();
        std::mem::swap(&mut self.active_buffer, &mut self.alt_buffer);
    }
    /// Get a Reader for an Event
    /// 
    /// Panics if the requested Event is not registered
    pub fn get_reader<'a, T: gmEvent + 'static>(&'a mut self) -> EventReader<'a, T>{
        // Check if the Event is valid
        if !self.registry.contains(T::EVENT_ID()){
            panic!("ERROR: Attempted to fetch unregistered event: {}", T::EVENT_ID())
        }

        // If we don't have this key in buffer yet, initialize it
        // This is a hacky workaround to return a "valid" `EventReader`
        // even if the queue for that event is empty
        if !self.alt_buffer.contains_key(T::EVENT_ID()){
            self.alt_buffer.insert(T::EVENT_ID(), RefCell::new(Box::new(VecDeque::<T>::new())));
        }

        // We have checks for valid ID and a backup Queue, so we can safely unwrap
        let queue = self.alt_buffer.get(T::EVENT_ID()).unwrap();

        EventReader::new(
            Ref::map(
                queue.borrow(), 
                |x| x.downcast_ref::<VecDeque<T>>().unwrap())
        )
    }
    /// Get a Writer for an Event
    /// 
    /// Panics if the requested Event is not registered
    pub fn get_writer<'a, T: gmEvent + 'static>(&'a mut self) -> EventWriter<'a, T>{
        // Check if the Event is valid
        if !self.registry.contains(T::EVENT_ID()){
            panic!("ERROR: Attempted to fetch unregistered event: {}", T::EVENT_ID())
        }

        // If there's a valid event but no queue, set up a new one
        if !self.active_buffer.contains_key(T::EVENT_ID()){
            self.active_buffer.insert(T::EVENT_ID(), RefCell::new(Box::new(VecDeque::<T>::new())));
        }

        // We have checks for valid ID and a backup Queue, so we can safely unwrap
        let queue = self.active_buffer.get(T::EVENT_ID()).unwrap();

        EventWriter::new(
            RefMut::map(
                queue.borrow_mut(),
                |x| x.downcast_mut::<VecDeque<T>>().unwrap())
        )
    }
}
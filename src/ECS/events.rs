use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet, VecDeque};
use std::any::Any;

use super::fetch::{EventReader, EventWriter};

/// # Event trait
/// Defines an event that systems can send and receive
/// 
/// ## WARNING
/// Make sure the Event ID does not collide with Events from other plugins
pub trait Event: 'static{
    const ID: &'static str;
}

/// # Event trait Wrapper
/// A wrapper trait for Events to safely store them within the Event Map
/// 
/// Provides ID method for identifying the underlying Event
pub trait EventWrapper{
    fn id(&self) -> &'static str;
}
impl<T: Event> EventWrapper for T{
    fn id(&self) -> &'static str {
        T::ID
    }
}

/// # Event Map
/// A Double Hashmap Buffer queue of Events
/// 
/// Maintains a Registry of Events to prevent illegal overrides and reading/writing non-existent Events
/// 
/// Buffers switch at the end of every Tick, clearing the previously Read-Only buffer
pub struct EventBufferMap{
    registry: HashSet<&'static str>,
    active_buffer: HashMap<&'static str, RefCell<Box<dyn Any>>>,
    alt_buffer: HashMap<&'static str, RefCell<Box<dyn Any>>>,
}
impl EventBufferMap{
    /// Create a new, empty EventMap
    pub fn new() -> Self{
        Self{
            registry: HashSet::new(),
            active_buffer: HashMap::new(),
            alt_buffer: HashMap::new(),
        }
    }

    /// Register an event
    pub fn register<T: Event>(&mut self){
        if self.registry.contains(T::ID){
            // Events CANNOT share IDs because systems expect a specific type
            // This ain't OOP, we can't replace one struct with another and expect it to go the same
            panic!("ERROR: Conflicting Event IDs: {}", T::ID)
        }
        self.registry.insert(T::ID);
    }
    /// Deregister an event
    /// 
    /// This also clears the respective Event's Queues from both buffers
    pub fn deregister<T: Event>(&mut self){
        self.registry.remove(T::ID);
        // Remove those events from the Map as they're no longer valid
        self.active_buffer.remove(T::ID);
        self.alt_buffer.remove(T::ID);
    }

    /// Switch buffers and clear the previous one
    pub(super) fn swap_buffers(&mut self){
        // Clear active buffer to (kinda) free up memory
        self.active_buffer.clear();
        std::mem::swap(&mut self.active_buffer, &mut self.alt_buffer);
    }
    /// Get a Reader for an Event
    /// 
    /// Panics if the requested Event is not registered
    pub fn get_reader<'a, T: Event + 'static>(&'a mut self) -> EventReader<'a, T>{
        // Check if the Event is valid
        if !self.registry.contains(T::ID){
            panic!("ERROR: Attempted to fetch unregistered event: {}", T::ID)
        }

        // If we don't have this key in buffer yet, initialize it
        // This is a hacky workaround to return a "valid" `EventReader`
        // even if the queue for that event is empty
        if !self.alt_buffer.contains_key(T::ID){
            self.alt_buffer.insert(T::ID, RefCell::new(Box::new(VecDeque::<T>::new())));
        }

        // We have checks for valid ID and a backup Queue, so we can safely unwrap
        let queue = self.alt_buffer.get(T::ID).unwrap();

        Ref::map(
            queue.borrow(), 
            |x| x.downcast_ref::<VecDeque<T>>().unwrap())
    }
    /// Get a Writer for an Event
    /// 
    /// Panics if the requested Event is not registered
    pub fn get_writer<'a, T: Event + 'static>(&'a mut self) -> EventWriter<'a, T>{
        // Check if the Event is valid
        if !self.registry.contains(T::ID){
            panic!("ERROR: Attempted to fetch unregistered event: {}", T::ID)
        }

        // If there's a valid event but no queue, set up a new one
        if !self.active_buffer.contains_key(T::ID){
            self.active_buffer.insert(T::ID, RefCell::new(Box::new(VecDeque::<T>::new())));
        }

        // We have checks for valid ID and a backup Queue, so we can safely unwrap
        let queue = self.active_buffer.get(T::ID).unwrap();

        RefMut::map(
            queue.borrow_mut(),
            |x| x.downcast_mut::<VecDeque<T>>().unwrap())
    }
}
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};

use super::fetch::{EventReader, EventWriter};

/// # Event trait
/// Defines an Event that Systems can send and receive
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
    read_buffer: HashMap<&'static str, RefCell<Box<dyn EventQueue>>>,
    write_buffer: HashMap<&'static str, RefCell<Box<dyn EventQueue>>>,
}
impl EventBufferMap{
    /// Create a new, empty EventMap
    pub fn new() -> Self{
        Self{
            registry: HashSet::new(),
            read_buffer: HashMap::new(),
            write_buffer: HashMap::new(),
        }
    }

    /// Register an Event
    pub fn register<T: Event>(&mut self){
        if self.registry.contains(T::ID){
            // Events CANNOT share IDs because Systems expect a specific type
            // This ain't OOP, we can't replace one struct with another and expect it to go the same
            panic!("ERROR: Conflicting Event IDs: {}", T::ID)
        }
        self.registry.insert(T::ID);
        self.read_buffer.insert(T::ID, RefCell::new(Box::new(Vec::<T>::new())));
        self.write_buffer.insert(T::ID, RefCell::new(Box::new(Vec::<T>::new())));
    }
    /// Deregister an Event
    /// 
    /// This also clears the respective Event's Queues from both buffers
    pub fn deregister<T: Event>(&mut self){
        self.registry.remove(T::ID);
        // Remove those events from the Map as they're no longer valid
        self.read_buffer.remove(T::ID);
        self.write_buffer.remove(T::ID);
    }

    /// Switch buffers and clear the previous one
    pub(crate) fn swap_buffers(&mut self){
        // Clear active buffer to (kinda) free up memory
        for queue in self.read_buffer.values_mut(){
            queue.borrow_mut().clear();
        }
        std::mem::swap(&mut self.read_buffer, &mut self.write_buffer);
    }
    /// Get a Reader for an Event
    /// 
    /// Panics if the requested Event is not registered
    pub fn get_reader<'a, T: Event + 'static>(&'a self) -> EventReader<'a, T>{
        // Check if the Event is valid
        if !self.registry.contains(T::ID){
            panic!("ERROR: Attempted to fetch unregistered Event: {}", T::ID)
        }

        // We have checks for valid ID and a backup Queue, so we can safely unwrap
        let queue = self.write_buffer.get(T::ID).unwrap();

        EventReader(
            Ref::map(
                queue.borrow(), 
                |x| x.downcast_ref::<T>())
            )
    }
    /// Get a Writer for an Event
    /// 
    /// Panics if the requested Event is not registered
    pub fn get_writer<'a, T: Event + 'static>(&'a self) -> EventWriter<'a, T>{
        // Check if the Event is valid
        if !self.registry.contains(T::ID){
            panic!("ERROR: Attempted to fetch unregistered Event: {}", T::ID)
        }

        // We have checks for valid ID and a backup Queue, so we can safely unwrap
        let read_queue = self.read_buffer.get(T::ID).unwrap();
        let write_queue = self.write_buffer.get(T::ID).unwrap();

        EventWriter{
            read: Ref::map(
                read_queue.borrow(), 
                |x| x.downcast_ref::<T>()),
            write: RefMut::map(
                write_queue.borrow_mut(),
                |x| x.downcast_mut::<T>())
        }
    }
    /// Get a list of events currently in the Read Buffer
    /// 
    /// Called "active" as they're the ones being read in the current frame
    pub fn get_active_events(&self) -> Box<[&'static str]>{
        // Bit of a mess, but it works
        self.read_buffer.iter()
            .map_while(
                |(id, queue)|
                Some(*id).filter(|_| queue.borrow().is_empty())
            )
            .collect()
    }
    /// Get the Event registry 
    pub fn get_registry(&self) -> &HashSet<&'static str>{
        &self.registry
    }
}

/// # Event queue trait
/// A tiny rudimentary trait to remove the usage of `UnsafeCell` from World
/// 
/// It is essentially `Any` trait with an added `.clear()` method to flush the queue
trait EventQueue{
    /// Clear the underlying Queue
    fn clear(&mut self);
    /// Check if there are any events in this Queue
    fn is_empty(&self) -> bool;
}
impl<E: Event> EventQueue for Vec<E>{
    fn clear(&mut self) {
        self.clear();
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
impl dyn EventQueue{
    /// Downcast to a reference of an Event `T` queue
    fn downcast_ref<T: Event>(&self) -> &Vec<T>{
        unsafe{&*(self as *const dyn EventQueue as *const Vec<T>)}
    }
    /// Downcast to a mutable reference of an Event `T` queue
    fn downcast_mut<T: Event>(&mut self) -> &mut Vec<T>{
        unsafe{&mut *(self as *mut dyn EventQueue as *mut Vec<T>)}
    }
}

/// **System Level Event**
/// 
/// Announces to the Dispatcher that the app is to be shut down
/// 
/// Note: It doesn't shut down immediatelly, the Dispatcher reads the System Level Events only once it has run Postprocessors
/// 
/// TODO: Error Codes themselves
pub struct ExitApp(pub i32);
impl Event for ExitApp{
    const ID: &'static str = "_APP_EXIT";
}
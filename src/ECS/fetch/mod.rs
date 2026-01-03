#![allow(type_alias_bounds)]
use std::cell::{RefMut, Ref};

use super::comp::Component;
use super::events::Event;
use super::resource::Resource;
use super::commands::{Command, CommandWrapper};

pub mod query;
pub mod request;

pub type Fetch<'a, C: Component> = Ref<'a, C::STORAGE>;
pub type FetchMut<'a, C: Component> = RefMut<'a, C::STORAGE>;

pub type FetchRes<'a, R: Resource> = Ref<'a, R>;
pub type FetchResMut<'a, R: Resource> = RefMut<'a, R>;

/// # Event Reader
/// Lets you read events that have been sent on the previous frame
pub struct EventReader<'a, E: Event>(pub(super) Ref<'a, Vec<E>>);
impl<E: Event> EventReader<'_, E>{
    /// Iterate over events sent on the previous frame
    pub fn iter(&self) -> impl Iterator<Item = &E>{
        self.0.iter()
    }
    /// Get the number events that were sent on the previous frame
    pub fn event_count(&self) -> usize{
        self.0.len()
    }
}
/// # Event Writer
/// Lets you read events that have been sent on the previous frame, as well as send events for next frame
pub struct EventWriter<'a, E: Event>{
    pub(super) read: Ref<'a, Vec<E>>,
    pub(super) write: RefMut<'a, Vec<E>>
}
impl<E: Event> EventWriter<'_, E>{
    /// Iterate over events sent on the current frame
    pub fn current_iter(&self) -> impl Iterator<Item = &E>{
        self.write.iter()
    }
    /// Iterate over events sent on the previous frame
    pub fn prev_iter(&self) -> impl Iterator<Item = &E>{
        self.read.iter()
    }
    /// Get the number of events that are present on the current frame
    pub fn current_event_count(&self) -> usize{
        self.write.len()
    }
    /// Get the number events that were sent on the previous frame
    pub fn prev_event_count(&self) -> usize{
        self.read.len()
    }
    // The only function exclusive to Writer
    /// Send an Event
    pub fn send(&mut self, event: E){
        self.write.push(event);
    }
}

pub struct CommandWriter<'a>(pub(super) RefMut<'a, Vec<Box<dyn CommandWrapper>>>);
impl CommandWriter<'_>{
    /// Get the number of Commands that are currently in the queue
    pub fn command_count(&self) -> usize{
        self.0.len()
    }
    /// Send a Command
    pub fn send<C: Command>(&mut self, command: C){
        self.0.push(Box::new(command));
    }
}
pub struct TriggerWriter<'a>(pub(super) RefMut<'a, Vec<&'static str>>);
impl TriggerWriter<'_>{
    /// Get the numebr of Triggers that are currently in the queue
    pub fn trigger_count(&self) -> usize{
        self.0.len()
    }
    /// Send a Trigger
    pub fn send(&mut self, trigger: &'static str){
        self.0.push(trigger);
    }
}

///////////////////////////////////////////////////////////////////////////////
// Reexports
///////////////////////////////////////////////////////////////////////////////

pub use query::*;
pub use request::*;
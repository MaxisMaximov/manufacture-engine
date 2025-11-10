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

pub struct EventReader<'a, E: Event>(pub(super) Ref<'a, Vec<E>>);
impl<E: Event> EventReader<'_, E>{
    /// Iterate over events sent on the previous frame
    pub fn read_iter(&self) -> impl Iterator<Item = &E>{
        self.0.iter()
    }
    /// Get the number events that were sent on the previous frame
    pub fn event_count(&self) -> usize{
        self.0.len()
    }
}
pub struct EventWriter<'a, E: Event>(pub(super) RefMut<'a, Vec<E>>);
impl<E: Event> EventWriter<'_, E>{
    /// Iterate over events sent on the current frame
    /// 
    /// TODO: Add support for previous frame events
    pub fn iter(&self) -> impl Iterator<Item = &E>{
        self.0.iter()
    }
    /// Get the number of events that are present on the current frame
    /// 
    /// TODO: Add support for previous frame events
    pub fn event_count(&self) -> usize{
        self.0.len()
    }
    // The only function exclusive to Writer
    /// Send an event
    pub fn send(&mut self, Event: E){
        self.0.push(Event);
    }
}

pub struct CommandWriter<'a>(pub(super) RefMut<'a, Vec<Box<dyn CommandWrapper>>>);
impl CommandWriter<'_>{
    /// Get the number of Commands that are currently in the queue
    pub fn command_count(&self) -> usize{
        self.0.len()
    }
    /// Send a Command
    pub fn send<C: Command>(&mut self, Command: C){
        self.0.push(Box::new(Command));
    }
}
pub struct TriggerWriter<'a>(pub(super) RefMut<'a, Vec<&'static str>>);
impl TriggerWriter<'_>{
    /// Get the numebr of Triggers that are currently in the queue
    pub fn trigger_count(&self) -> usize{
        self.0.len()
    }
    /// Send a Trigger
    pub fn send(&mut self, Trigger: &'static str){
        self.0.push(Trigger);
    }
}

///////////////////////////////////////////////////////////////////////////////
// Reexports
///////////////////////////////////////////////////////////////////////////////

pub use query::*;
pub use request::*;
use super::world::World;
use super::dispatcher::{RunOrder, SystemType};
use super::fetch::*;

/// # System trait
/// Defines a System that will be run on the World
///
/// `Data` is the Data the System wants from the World, like Components Query, Resources, Event Readers/Writers and Command Writer
/// 
/// `ID` is what the System will be identified by for future overrides
/// 
/// `OVERRIDE` marks this system as an override of a system with same ID. 
/// Note: If multiple systems are marked as overrides of the same system, only the last loaded plugin gets the go. Every system is not an override by default. 
/// TODO: Priority system for overrides? Multiple overrides at the same time?
/// 
/// `DEPENDS` are the Systems that must be registered for this System, the System has no dependencies by default
/// 
/// `RUNORD` specifies what Systems should this System be run before/after, the System has no Run Orders be default
/// 
/// `TYPE` defines where the System should be put within the Execution loop, it is `SystemType::Logic` by default
/// 
/// ## WARNING
/// Make sure your System's ID does not collide with IDs of Systems from other plugins
pub trait System: 'static{
    type Data: RequestData;
    const ID: &'static str;
    const OVERRIDE: bool = false;
    const DEPENDS: &'static [&'static str] = &[];
    const RUNORD: &'static [RunOrder] = &[];
    const TYPE: SystemType = SystemType::Logic;

    /// Create a new instance of this System
    fn new() -> Self;
    /// Run the System
    fn execute(&mut self, Data: Request<'_, Self::Data>);
}

/// # System trait Wrapper
/// A wrapper trait for Systems to safely store and dispatch them in the Dispatcher
/// 
/// Provides methods for accessing the specifics of the underlying System
pub trait SystemWrapper{
    /// Get the underlying System's ID
    fn id(&self) -> &'static str;
    /// Get the underlying System's dependencies
    fn depends(&self) -> &'static [&'static str];
    /// Get a list of run order conditions of the underlying System
    fn run_order(&self) -> &'static [RunOrder];
    /// Get the type of the underlying System
    fn sys_type(&self) -> SystemType;
    /// Run the underlying System with specified World
    fn execute<'a>(&mut self, World: &'a mut World);
    
}

impl<T: System> SystemWrapper for T{
    fn id(&self) -> &'static str {
        T::ID
    }   
    fn depends(&self) -> &'static [&'static str] {
        T::DEPENDS
    }
    fn run_order(&self) -> &'static [RunOrder] {
        T::RUNORD
    }
    fn sys_type(&self) -> SystemType {
        T::TYPE
    }
    fn execute<'a>(&mut self, World: &'a mut World) {
        self.execute(Request::fetch(World));
    }
}
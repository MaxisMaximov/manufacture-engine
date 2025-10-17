use crate::ECS::fetch::query::QueryFilter;

use super::world::World;
use super::dispatcher::{RunOrder, SystemType};
use super::fetch::query::{QueryData, Query};
use super::fetch::request::{Request, RequestData};

/// # System trait
/// Defines a System that will be run on the World
/// 
/// `QUERY` is the query for components the System wants from the Word
/// 
/// `FILTERS` are filters for Query to use when fetching data
/// 
/// `REQUEST` are the System resources the System wants from the World, like Resources, Event Readers/Writers and Command Writer
/// 
/// `ID` is what the System will be identified by for future overrides
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
    type QUERY: QueryData;
    type FILTERS: QueryFilter;
    type REQUEST: RequestData;
    const ID: &'static str;
    const DEPENDS: &'static [&'static str] = &[];
    const RUNORD: &'static [RunOrder] = &[];
    const TYPE: SystemType = SystemType::Logic;

    /// Create a new instance of this System
    fn new() -> Self;
    /// Run the System
    fn execute(&mut self, Query: Query<'_, Self::QUERY, Self::FILTERS>, Request: Request<'_, Self::REQUEST>);
}

/// # System trait Wrapper
/// A wrapper trait for Systems to safely dispatch them in the Dispatcher
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
        self.execute(Query::fetch(World), Request::fetch(World));
    }
}
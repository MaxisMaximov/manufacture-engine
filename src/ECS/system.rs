use super::world::World;
use super::fetch::query::{QueryData, Query};
use super::fetch::request::{Request, RequestData};

/// # System trait
/// Defines a System that will be run on the World
/// 
/// `QUERY` is the query for components the System wants from the Word
/// 
/// `REQUEST` are the System resources the System wants from the World, like Resources, Event Readers/Writers and Command Writer
/// 
/// `ID` is what the System will be identified by for future overrides
/// 
/// `DEPENDS` are the other Systems that must be run before the System can be
/// 
/// ## WARNING
/// Make sure your System's ID does not collide with Systems fro other plugins
pub trait System: 'static{
    type QUERY: QueryData;
    type REQUEST: RequestData;
    const ID: &'static str;
    const DEPENDS: &'static [&'static str];

    /// Create a new instance of this System
    fn new() -> Self;
    /// Run the System
    fn execute(&mut self, Query: Query<'_, Self::QUERY>, Request: Request<'_, Self::REQUEST>);
}

/// # System trait Wrapper
/// A wrapper trait for Systems to safely dispatch them in the Dispatcher
/// 
/// Provides ID method for identifying the underlying System, Depends method for getting the it's dependencies
/// and Execute method for running it
pub trait SystemWrapper{
    /// Get the underlying System's ID
    fn id(&self) -> &'static str;
    /// Get the underlying System's dependencies
    fn depends(&self) -> &'static [&'static str];
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
    fn execute<'a>(&mut self, World: &'a mut World) {
        self.execute(Query::fetch(World), Request::fetch(World));
    }
}
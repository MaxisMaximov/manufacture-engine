use super::world::World;
use super::fetch::{QueryData, Query};

/// # System trait
/// Defines a System that will be run on the World
/// 
/// `QUERY` is the components the system wants from the Word
/// 
/// `ID` is what the system will be identified by for future overrides
/// 
/// `DEPENDS` are the other Systems that must be run before the System can be
/// 
/// ## WARNING
/// Make sure your System ID does not collide with Systems fro other plugins
pub trait System: 'static{
    type QUERY: QueryData;
    const ID: &'static str;
    const DEPENDS: &'static [&'static str];

    /// Create a new instance of this system
    fn new() -> Self;
    /// Run the system
    fn execute(&mut self, Data: Query<'_, Self::QUERY>);
}

/// # System trait Wrapper
/// A wrapper trait for Systems to safely dispatch them in the Dispatcher
/// 
/// Provides ID method for identifying the underlying System, Depends method for getting the it's dependencies
/// and Execute method for running it
pub trait SystemWrapper{
    /// Get the underlying System's ID
    fn id(&self) -> &'static str;
    /// Get the underlying system's dependencies
    fn depends(&self) -> &'static [&'static str];
    /// Run the underlying system with specified World
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
        self.execute(Query::fetch(World));
    }
}
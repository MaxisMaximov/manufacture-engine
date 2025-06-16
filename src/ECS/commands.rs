use std::any::Any;
use super::world::gmWorld;

/// # Command trait
/// Defines a command that does an operation on the whole World
/// 
/// Typically used to add/remove components from entities, 
/// spawn/despawn entities, register new resources/components etc.
/// 
/// ID is completely optional for debug purposes
pub trait Command: Any{
    const ID: &'static str = "idkfa";
    /// Execute the Command on specified World
    fn execute(&mut self, World: &mut gmWorld);
}

/// # Command trait Wrapper
/// A wrapper trait for Commands to safely store them within the World
/// 
/// Provides ID method for identifying the command (if such ID was specified) 
/// and execute method for executing the command on the specified World
pub trait CommandWrapper{
    fn id(&self) -> &'static str;
    fn execute(&mut self, World: &mut gmWorld);
}

impl<T: Command> CommandWrapper for T{
    /// Get the underlying Command's ID
    fn id(&self) -> &'static str {
        T::ID
    }

    /// Execute the Command on the specified World
    fn execute(&mut self, World: &mut gmWorld) {
        Command::execute(self, World);
    }
}
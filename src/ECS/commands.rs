use super::world::World;

/// # Command trait
/// Defines a command that does an operation on the whole World
/// 
/// Typically used to add/remove Components from entities, 
/// spawn/despawn entities, register new resources/Components etc.
/// 
/// ID is completely optional for debug purposes
pub trait Command: 'static{
    const ID: &'static str = "idkfa";
    /// Execute the Command on specified World
    fn execute(&mut self, world: &mut World);
}

/// # Command trait Wrapper
/// A wrapper trait for Commands to safely store them within the World
/// 
/// Provides ID method for identifying the command (if such ID was specified) 
/// and execute method for executing the command on the specified World
pub trait CommandWrapper{
    /// Get the underlying Command's ID
    fn id(&self) -> &'static str;
    /// Execute the Command on the specified World
    fn execute(&mut self, world: &mut World);
}

impl<T: Command> CommandWrapper for T{
    fn id(&self) -> &'static str {
        T::ID
    }

    fn execute(&mut self, world: &mut World) {
        Command::execute(self, world);
    }
}
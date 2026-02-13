use super::world::World;
use super::dispatcher::{RunOrder, SystemType};
use super::fetch::*;

/// # System trait
/// Defines a System that will be run on the World
///
/// `Data` is the Data the System wants from the World, like Components Query, Resources, Event Readers/Writers, etc.
/// 
/// `ID` is what the System is identified by in the Dispatcher and in Overrides
/// 
/// `OVERRIDE` marks this System as an override of a System with same ID. 
/// Note: If multiple Systems are marked as overrides of the same System, only the last loaded plugin gets the go. Every System is not an override by default. 
/// TODO: Priority System for overrides? Multiple overrides at the same time?
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
    type Data<'a>: RequestData;
    const ID: &'static str;
    const OVERRIDE: bool = false;
    const DEPENDS: &'static [&'static str] = &[];
    const RUNORD: &'static [RunOrder] = &[];
    const TYPE: SystemType = SystemType::Logic;

    /// Create a new instance of this System
    fn new() -> Self;
    /// Run the System
    fn execute(&mut self, data: Request<'_, Self::Data<'_>>);
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
    fn execute<'a>(&mut self, world: &'a mut World);
    
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
    fn execute<'a>(&mut self, world: &'a mut World) {
        self.execute(Request::fetch(world));
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    mod test_comp{
        use super::*;
        use crate::ECS::comp::Component;
        use crate::ECS::storage::test::HashMapStorage;
        use crate::prelude::Storage;

        struct idkfa(u8);
        struct iddqd(u8);
        impl Component for idkfa{
            type STORAGE = HashMapStorage<Self>;
        
            const ID: &'static str = "idkfa";
        }
        impl Component for iddqd{
            type STORAGE = HashMapStorage<Self>;
        
            const ID: &'static str = "iddqd";
        }

        struct Get;
        struct Iter;
        /// PREREQUISTES:
        /// IDs 1 and 2 have only idkfa and iddqd component respectively
        struct AddRemove;
        
        impl System for Get{
            type Data<'a> = Query<(&'a idkfa, &'a mut iddqd), ()>;
        
            const ID: &'static str = "_test_Get";
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {
                let get = data.get(&0);
                assert!(matches!(get, Some((&idkfa(5), &iddqd(10)))));

                let get_mut = data.get_mut(&0);
                assert!(matches!(get_mut, Some((&idkfa(5), &mut iddqd(10)))))
            }
        }
        impl System for Iter{
            type Data<'a> = Query<(&'a idkfa, &'a mut iddqd), ()>;
        
            const ID: &'static str = "_test_Iter";
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {

                for (kfa, dqd) in data.iter(){
                    assert!(matches!(kfa, idkfa(5)));
                    assert!(matches!(dqd, iddqd(10)))
                }

                for (kfa, dqd) in data.iter_mut(){
                    dqd.0 = 20;
                    assert!(matches!(kfa, idkfa(5)));
                    assert!(matches!(dqd, iddqd(20)))
                }
            }
        }
        impl System for AddRemove{
            type Data<'a> = Query<(&'a mut idkfa, &'a mut iddqd), ()>;
        
            const ID: &'static str = "_test_AddRemove";
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {

                data.1.insert(1, iddqd(20)); // Add iddqd to 1
                data.0.insert(2, idkfa(10)); // Add idkfa to 2

                assert!(data.1.get(&1).is_some());
                assert!(data.0.get(&2).is_some());
                
                data.0.remove(&1); // Remove idkfa from 1
                data.1.remove(&2); // Remove iddqd from 2
                
                assert!(data.0.get(&1).is_none());
                assert!(data.1.get(&2).is_none());
            }
        }

        #[test]
        fn test(){
            let mut world = World::new();
            world.register_comp::<idkfa>();
            world.register_comp::<iddqd>();

            world.spawn().with(idkfa(5)).with(iddqd(10)).finish();
            world.spawn().with(idkfa(5)).finish();
            world.spawn().with(iddqd(10)).finish();

            let mut test_get = Get::new();
            let mut test_iter = Iter::new();
            let mut test_addremove = AddRemove::new();

            SystemWrapper::execute(&mut test_get, &mut world);
            SystemWrapper::execute(&mut test_iter, &mut world);
            SystemWrapper::execute(&mut test_addremove, &mut world);
        }
    }
    mod test_resource{}
    mod test_events{}
    mod test_meta{}
}
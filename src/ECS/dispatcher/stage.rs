use std::collections::HashMap;
// Why is this a valid syntax
use super::super::world::*;
use super::{System, SystemWrapper};

pub struct gmDispatchStage{
    systems: HashMap<&'static str, ()>,
    inner: Vec<Box<dyn SystemWrapper>>
}
impl gmDispatchStage{
    pub fn new() -> Self{
        Self{
            systems: HashMap::new(),
            inner: Vec::new()
        }
    }

    pub fn withSys<T>(mut self) -> Self where T: System + 'static{
        self.addSys::<T>();
        self
    }

    pub fn addSys<T>(&mut self) where T: System + 'static{
        // The Dispatcher does the check if exists for us already so no need to check it
        self.systems.insert(T::ID, ());
        self.inner.push(Box::new(T::new()));
    }

    pub fn checkSys<T>(&self) -> bool where T: System{
        self.checkSysID(T::ID)
    }
    
    pub fn checkSysID(&self, IN_id: &'static str) -> bool{
        match self.systems.get(IN_id){
            Some(_) => true,
            None => false,
        }
    }

    pub fn dispatch(&mut self, IN_world: &mut gmWorld){
        for SYS in self.inner.iter_mut(){
            SYS.execute(IN_world);
        }
    }
}
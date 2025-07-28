use std::collections::{HashMap, HashSet};

use super::system::*;
use super::world::World;

type Stage = Vec<Box<dyn SystemWrapper>>;

pub struct Dispatcher{
    registry: HashSet<&'static str>,
    systems: Vec<Stage>
}
impl Dispatcher{
    pub fn dispatch(&mut self, World: &mut World){
        for stage in self.systems.iter_mut(){
            for system in stage.iter_mut(){
                system.execute(World);
            }
        }
    }
}

#[must_use]
pub struct DispatcherBuilder{
    registry: HashMap<&'static str, Box<dyn SystemWrapper>>
}
impl DispatcherBuilder{
    pub fn new() -> Self{
        Self{
            registry: HashMap::new(),
        }
    }
    pub fn add<S: System>(&mut self){
        unimplemented!()
    }
    pub fn build(mut self) -> Dispatcher{
        Dispatcher{
            registry: todo!(),
            systems: todo!()
        }
    }
}

pub enum RunOrder{
    Before(&'static str),
    After(&'static str),
}
impl RunOrder{
    pub fn value(&self) -> &'static str{
        match *self{
            RunOrder::Before(val) => val,
            RunOrder::After(val) => val,
        }
    }
}

pub enum SystemType{
    Preprocessor,
    Normal,
    Postprocessor
}
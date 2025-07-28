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
    registry: HashSet<&'static str>,
    dep_graph: Vec<Vec<Box<dyn SystemWrapper>>>
}
impl DispatcherBuilder{
    pub fn new() -> Self{
        Self{
            registry: HashSet::new(),
            dep_graph: Vec::new()
        }
    }
    pub fn add<S: System>(&mut self){
        if self.registry.contains(S::ID){
            panic!("ERROR: System {} already exists", S::ID)
        }
        self.registry.insert(S::ID);
        self.dep_graph[0].push(Box::new(S::new()));
    }
    pub fn build(mut self) -> Dispatcher{
        // VERIFY DEPS
        todo!();
        // BUILD DEP GRAPH
        todo!();
        // FINALIZE STAGES
        todo!();
        
        Dispatcher{
            registry: self.registry,
            systems: self.dep_graph
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
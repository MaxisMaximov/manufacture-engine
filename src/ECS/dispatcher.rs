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
    dep_graph: Vec<HashMap<&'static str, Box<dyn SystemWrapper>>>
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
        self.dep_graph[0].insert(S::ID, Box::new(S::new()));
    }
    pub fn build(mut self) -> Dispatcher{
        // Verify dependencies of each system
        for system in self.dep_graph[0].values(){
            for dep in system.depends(){
                if !self.registry.contains(dep){
                    panic!("ERROR: System {}'s dependency system {} does not exist", system.id(), dep)
                }
            }
        }

        // Build dependency 'graph' and resolve system order
        // Welcome to indentation hell
        // Population: Graph Building
        let mut shifts = HashSet::new();
        for layer_id in 0..{
            let layer = self.dep_graph.get(layer_id).unwrap();
            // Iterate over layer's systems to see which we should shift
            for system in layer.values(){
                for order_dep in system.run_order(){
                    match order_dep{
                        // If we need this system to run before, we shift the other system to later
                        RunOrder::Before(id) => {
                            if layer.contains_key(id){
                                shifts.insert(id.clone());
                            }
                        },
                        // Equivalent of the other system running before this one
                        // So we simply shift this one down
                        RunOrder::After(id) => {
                            if layer.contains_key(id){
                                shifts.insert(system.id());
                            }
                        },
                    }
                }
            }
            // No shifts happened, we're done with our graph
            if shifts.is_empty() {
                break;
            }
            // Push a new layer for the shifts..
            self.dep_graph.push(HashMap::new());
            // ..and now move all the systems from current layer to next layer
            // Also clears the shifts set for next layer
            for system_id in shifts.drain(){
                let system = self.dep_graph[layer_id].remove(system_id).unwrap();
                self.dep_graph[layer_id + 1].insert(system.id(), system);
            }
        }

        // FINALIZE STAGES
        todo!();
        
        Dispatcher{
            registry: unimplemented!(),
            systems: unimplemented!()
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
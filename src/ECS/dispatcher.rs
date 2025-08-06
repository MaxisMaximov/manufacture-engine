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
    systems: HashMap<&'static str, Box<dyn SystemWrapper>>,
    dep_graph: Vec<HashSet<&'static str>>
}
impl DispatcherBuilder{
    pub fn new() -> Self{
        Self{
            registry: HashSet::new(),
            systems: HashMap::new(),
            dep_graph: Vec::new()
        }
    }
    pub fn add<S: System>(&mut self){

        if self.systems.contains_key(S::ID){
            panic!("ERROR: System {} already exists", S::ID)
        }

        self.registry.insert(S::ID);
        self.systems.insert(S::ID, Box::new(S::new()));
        self.dep_graph[0].insert(S::ID); // We will resolve everythin in the Build step
    }
    // Verify dependencies of each system
    fn verify_deps(&mut self){
        for system in self.systems.values(){
            for dep in system.depends(){
                if !self.systems.contains_key(dep){
                    panic!("ERROR: System {}'s dependency system {} does not exist", system.id(), dep)
                }
            }
        }
    }
    // Build dependency 'graph' and resolve system order
    fn build_run_order_graph(&mut self){
        // Welcome to indentation hell
        // Population: Graph Building

        // Here to prevent unnecessary reallocation
        let mut shifts = HashSet::new();

        // Essentially iterate until everything is resolved
        for layer_id in 0..{

            // Unwrap: We only push new layers when there were shifts on previous layers
            // If there were none, we break out of the loop
            let layer = self.dep_graph.get(layer_id).unwrap();

            // Iterate over layer's systems to see which we should shift
            for system_id in layer.iter(){
                
                for order_dep in self.systems.get(system_id).unwrap().run_order(){

                    match order_dep{
                        // If we need this system to run before, we shift the other system to later
                        RunOrder::Before(id) => {
                            if layer.contains(id){
                                shifts.insert(id.clone());
                            }
                        },
                        // Equivalent of the other system having `Before(this_system)`
                        // So we shift *this* one to later instead
                        RunOrder::After(id) => {
                            if layer.contains(id){
                                shifts.insert(system_id);
                            }
                        },
                    }
                }
            }

            // No shifts happened, we're done with our graph
            if shifts.is_empty() {
                break;
            }

            // This should not happen unless there's a circular dependency between the systems
            if shifts.len() == layer.len(){
                panic!("ERROR: There are circular run orders between {} systems:\n{:#?}\nPlease resolve them", layer.len(), layer)
            }

            // This is here to ensure the layer reference gets dropped
            // The compiler doesn't complain that we're pushing to the graph while having
            // a part of it borrowed in the later step, no idea why, usually it yells at me for that
            drop(layer);

            // Push a new layer and move all the shifted systems from current layer to next layer
            self.dep_graph.push(HashSet::new());
            
            for system_id in shifts.drain(){ // Clear the shifts while we're at it
                self.dep_graph[layer_id].remove(system_id);
                self.dep_graph[layer_id + 1].insert(system_id);
            }
        }
    }
    // Convert layers to stages & split them accordingly
    fn build_stages(&mut self) -> Dispatcher{
        // Init thingies that will be used in the Dispatcher
        let mut registry = HashSet::new();
        let mut stages: Vec<Stage> = Vec::new();

        // Now convert the 'graph' into stages 
        for mut layer in self.dep_graph.drain(0..){

            let mut stage = Vec::new();

            // Convert layer to stages
            for system_id in layer.drain(){

                // Take the system Box out of the registry and 
                // push it to the Dispatcher registry and the stage
                let system = self.systems.remove(system_id).unwrap();

                registry.insert(system_id);
                stage.push(system);

                // Stage is full, push it outta here and init a new one for next iteration
                if stage.len() == 5{
                    stages.push(stage);
                    stage = Vec::new()
                }
            }
            // Push the incomplete stage just in case
            // We can't have it for next iteration as systems may get their run order jumbled up
            if !stage.is_empty(){
                stages.push(stage);
            }
        };
        
        Dispatcher{
            registry,
            systems: stages,
        }
    }
    pub fn build(mut self) -> Dispatcher{

        // First time splitting something into sepparate functions
        // But it's for the sake of readibility here
        self.verify_deps();
        self.build_run_order_graph();
        self.build_stages()
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
    Logic,
    Postprocessor
}
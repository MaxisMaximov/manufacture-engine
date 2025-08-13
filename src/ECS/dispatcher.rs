use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use super::system::*;
use super::world::World;

type Stage = Vec<Box<dyn SystemWrapper>>;

pub struct Dispatcher{
    registry: HashSet<&'static str>,
    preproc: Vec<Stage>,
    logic: Vec<Stage>,
    postproc: Vec<Stage>
}
impl Dispatcher{
    pub fn dispatch(&mut self, World: &mut World){
        let mut previous_tick = Instant::now();
        loop{
            // -- PREPROCESSORS --
            for stage in self.preproc.iter_mut(){
                for system in stage.iter_mut(){
                    system.execute(World);
                }
            }
            // -- LOGIC LOOP --
            if previous_tick.elapsed().as_millis() < 50{
                for stage in self.logic.iter_mut(){
                    for system in stage.iter_mut(){
                        system.execute(World);
                    }
                }
            }
            // -- POSTPROCESSORS --
            for stage in self.postproc.iter_mut(){
                for system in stage.iter_mut(){
                    system.execute(World);
                }
            }
            previous_tick = Instant::now();
        }
    }
}

#[must_use]
pub struct DispatcherBuilder{
    systems: HashMap<&'static str, Box<dyn SystemWrapper>>
}
impl DispatcherBuilder{
    pub fn new() -> Self{
        Self{
            systems: HashMap::new()
        }
    }
    pub fn add<S: System>(&mut self){

        if self.systems.contains_key(S::ID){
            panic!("ERROR: System {} already exists", S::ID)
        }

        self.systems.insert(S::ID, Box::new(S::new()));
    }
    // Create the registry with all the specifics about each system
    // TODO: Make it actually compile the specifics
    // lol
    fn compile_registry(&self) -> HashSet<&'static str>{
        let mut registry = HashSet::new();
        self.systems.keys().map(|system| registry.insert(*system));
        registry
    }
    // Verify dependencies of each system
    fn verify_deps(&self){
        for system in self.systems.values(){
            for dep in system.depends(){
                if !self.systems.contains_key(dep){
                    panic!("ERROR: System {}'s dependency system {} does not exist", system.id(), dep)
                }
            }
        }
    }
    // Build dependency 'graph' and resolve system order
    fn build_run_order_graph(&self, SystemSet: HashSet<&'static str>) -> Vec<HashSet<&'static str>>{
        // Welcome to indentation hell
        // Population: Graph Building

        let mut dep_graph = Vec::from([SystemSet]);
        // Here to prevent unnecessary reallocation
        let mut shifts = HashSet::new();

        // Essentially iterate until everything is resolved
        for layer_id in 0..{

            // Unwrap: We only push new layers when there were shifts on previous layers
            // If there were none, we break out of the loop
            let layer = dep_graph.get(layer_id).unwrap();

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
            dep_graph.push(HashSet::new());
            
            for system_id in shifts.drain(){ // Clear the shifts while we're at it
                dep_graph[layer_id].remove(system_id);
                dep_graph[layer_id + 1].insert(system_id);
            }
        };

        dep_graph
    }
    // Convert layers to stages & split them accordingly
    fn build_stages(&mut self, mut Graph: Vec<HashSet<&'static str>>) -> Vec<Stage>{

        let mut stages: Vec<Stage> = Vec::new();

        // Now convert the 'graph' into stages 
        for mut layer in Graph.drain(0..){

            let mut stage = Vec::new();

            // Convert layer to stages
            for system_id in layer.drain(){

                // Take the system Box out of the registry and 
                // push it to the Dispatcher registry and the stage
                let system = self.systems.remove(system_id).unwrap();

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
        
        stages
    }
    pub fn build(mut self) -> Dispatcher{

        self.verify_deps();

        let registry = self.compile_registry();

        let mut preproc = HashSet::new();
        let mut logic = HashSet::new();
        let mut postproc = HashSet::new();

        for system in self.systems.values(){
            match system.sys_type(){
                SystemType::Preprocessor => preproc.insert(system.id()),
                SystemType::Logic => logic.insert(system.id()),
                SystemType::Postprocessor => postproc.insert(system.id()),
            };
        }

        let preproc_graph = self.build_run_order_graph(preproc);
        let logic_graph = self.build_run_order_graph(logic);
        let postproc_graph = self.build_run_order_graph(postproc);
        
        let preproc = self.build_stages(preproc_graph);
        let logic = self.build_stages(logic_graph);
        let postproc = self.build_stages(postproc_graph);

        Dispatcher{
            registry,
            preproc,
            logic,
            postproc,
        }
    }
}

struct RunOrderGraph{
    graph: Vec<HashMap<&'static str, &'static [RunOrder]>>
}
impl RunOrderGraph{
    fn new() -> Self{
        Self{
            graph: Vec::new(),
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
    Logic,
    Postprocessor
}
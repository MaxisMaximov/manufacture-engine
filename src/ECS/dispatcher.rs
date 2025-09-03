use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use super::system::*;
use super::world::World;

const MAX_SYS_PER_STAGE: usize = 5;
const TICKS_PER_SECOND: u64 = 20;

const TICKRATE: Duration = Duration::from_millis(1000/TICKS_PER_SECOND);

type Stage = Vec<Box<dyn SystemWrapper>>;

/// # System Dispatcher
/// Handles the execution of the Systems within the app
/// 
/// Has 2 loops:
/// - Staller Loop -- Runs every frame
/// - Logic Loop - Runs at most N times per second, specified by the Tickrate
pub struct Dispatcher{
    registry: HashMap<&'static str, SystemInfo>,
    preproc: Vec<Stage>,
    singlefires: HashMap<&'static str, Box<dyn SystemWrapper>>,
    logic: Vec<Stage>,
    postproc: Vec<Stage>
}
impl Dispatcher{
    /// Start building a new Dispatcher
    pub fn new() -> DispatcherBuilder{
        DispatcherBuilder::new()
    }
    /// Dispatch the systems
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
            if previous_tick.elapsed() >= TICKRATE{
                // -- Logic systems --
                for stage in self.logic.iter_mut(){
                    for system in stage.iter_mut(){
                        system.execute(World);
                    }
                }
                // -- Singlefires --
                for trigger in World.take_triggers(){
                    self.singlefires.get_mut(trigger).unwrap().execute(World);
                }
                // -- Event Responders --

                // -- Commands --
                for mut command in World.take_commands(){
                    command.execute(World);
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

/// # Dispatcher Builder
/// Handles the building of the Dispatcher without letting anything disrupt
/// 
/// Make sure to use `.build()` once you're done
#[must_use]
pub struct DispatcherBuilder{
    registry: HashMap<&'static str, SystemInfo>,
    preproc: StagesBuilder,
    logic: StagesBuilder,
    singlefires: HashMap<&'static str, Box<dyn SystemWrapper>>,
    postproc: StagesBuilder,
}
impl DispatcherBuilder{
    /// Start building a new Dispatcher
    pub fn new() -> Self{
        Self{
            registry: HashMap::new(),
            preproc: StagesBuilder::new(),
            logic: StagesBuilder::new(),
            singlefires: HashMap::new(),
            postproc: StagesBuilder::new(),            
        }
    }
    /// Add a system to the Dispatcher
    pub fn add<S: System>(&mut self){

        if self.registry.contains_key(S::ID){
            panic!("ERROR: System {} already exists", S::ID)
        }

        self.registry.insert(S::ID, SystemInfo::new::<S>());

        match S::TYPE{
            SystemType::Preprocessor => self.preproc.add::<S>(),
            SystemType::Logic => self.logic.add::<S>(),
            SystemType::Singlefire => {
                self.singlefires.insert(S::ID, Box::new(S::new()));
            },
            SystemType::Postprocessor => self.postproc.add::<S>(),
        }
    }
    /// Verify dependencies of each system
    fn verify_deps(&self){
        for system in self.registry.values(){
            for dep in system.depends.iter(){
                if !self.registry.contains_key(dep){
                    panic!("ERROR: System {}'s dependency system {} does not exist", system.id, dep)
                }
            }
        }
    }
    /// Build the Dispatcher
    pub fn build(self) -> Dispatcher{

        self.verify_deps();

        Dispatcher{
            registry: self.registry,
            preproc: self.preproc.build(),
            singlefires: self.singlefires,
            logic: self.logic.build(),
            postproc: self.postproc.build(),
        }
    }
}

/// # System Information
/// A collection of data for the Dispatcher's Registry to keep track of
/// 
/// Is typically only used as Metadata for debug purposes
struct SystemInfo{
    id: &'static str,
    depends: &'static [&'static str],
    run_ord: &'static [RunOrder],
    sys_type: SystemType
}
impl SystemInfo{
    fn new<S: System>() -> Self{
        Self{
            id: S::ID,
            depends: S::DEPENDS,
            run_ord: S::RUNORD,
            sys_type: S::TYPE,
        }
    }
}

/// # Stages Builder
/// Builds a stage graph for Dispatcher to execute using provided Systems
#[must_use]
struct StagesBuilder{
    systems: HashMap<&'static str, Box<dyn SystemWrapper>>,
}
impl StagesBuilder{
    /// Start building a new collection of Stages
    fn new() -> Self{
        Self{
            systems: HashMap::new(),
        }
    }
    /// Add a System to this builder
    fn add<S: System>(&mut self){
        self.systems.insert(S::ID, Box::new(S::new()));
    }
    /// Build the graph
    fn build_run_order_graph(&self) -> Vec<Vec<&'static str>>{
        // Welcome to indentation hell
        // Population: Graph Building

        // Here to prevent unnecessary reallocation
        let mut shifts = HashSet::new();

        // Prepare the graph
        // Yeah, it's kinda a mess
        let mut graph: Vec<HashMap<&'static str, &'static [RunOrder]>> 
            = Vec::from([
                    self.systems.values()
                                .map(|system|
                                    (system.id(), system.run_order())).collect()
                ]);


        // Essentially iterate until everything is resolved
        for layer_id in 0..{

            // Unwrap: We only push new layers when there were shifts on previous layers
            // If there were none, we would break out of the loop
            let layer = graph.get(layer_id).unwrap();

            // Iterate over layer's systems to see which we should shift
            for (system_id, order_deps) in layer.iter(){
                
                for order_dep in order_deps.iter(){

                    match order_dep{
                        // If we need this system to run before, we shift the other system to later
                        RunOrder::Before(id) => {
                            if layer.contains_key(id){
                                shifts.insert(*id);
                            }
                        },
                        // Equivalent of the other system having `Before(this_system)`
                        // So we shift *this* one to later instead
                        RunOrder::After(id) => {
                            if layer.contains_key(id){
                                shifts.insert(*system_id);
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
                panic!("ERROR: There are circular run orders between {} systems:\n{:#?}\nPlease resolve them", layer.len(), layer.keys())
            }

            // This is here to ensure the layer reference gets dropped
            // The compiler doesn't complain that we're pushing to the graph while having
            // a part of it borrowed in the later step, no idea why, usually it yells at me for that
            drop(layer);

            // Push a new layer and move all the shifted systems from current layer to next layer
            graph.push(HashMap::new());
            
            for system_id in shifts.drain(){ // Clear the shifts while we're at it
                let orders = graph[layer_id].remove(system_id).unwrap();
                graph[layer_id + 1].insert(system_id, orders);
            }
        };

        // Now convert it into a graph without the extra data
        let mut final_graph = Vec::new();

        for mut layer in graph.drain(0..){
            final_graph.push(Vec::new());
            for (id, _) in layer.drain(){
                final_graph.last_mut().unwrap().push(id);
            }
        }

        final_graph
    }
    /// Build the Stages for DIspatcher to use
    fn build(mut self) -> Vec<Stage>{

        let mut stages = Vec::new();

        let graph = self.build_run_order_graph();

        // We don't need to use `.iter()` as the final graph will not be used for anything else, we also own it
        for layer in graph{
            stages.push(Vec::new());
            for system_id in layer{
                // Don't like that I have to use so many unwraps
                stages.last_mut()
                    .unwrap()
                    .push(
                        self.systems.remove(system_id)
                        .unwrap()
                    );

                if stages.last().unwrap().len() == MAX_SYS_PER_STAGE{
                    stages.push(Vec:: new());
                }
            }
        }

        stages
    }
}

/// # Run Order enum
/// Specifies when a System should be run
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

/// # System Type
/// Specifies where the System should be within the Execution Loop
/// 
/// `Preprocessor` Systems are ran at the beggining of every frame  
/// They are typically used to update Resources
/// 
/// `Logic` Systems are ran at most N times per second specified by the Tickrate  
/// These systems run the actual logic of the game
/// 
/// `Postprocessor` Systems are ran at the end of every frame  
/// They are typically output Systems like Audio and Rendering
pub enum SystemType{
    Preprocessor,
    Logic,
    Singlefire,
    Postprocessor
}
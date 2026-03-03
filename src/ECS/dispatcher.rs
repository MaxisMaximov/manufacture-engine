use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use super::system::*;
use super::world::World;
use super::resource::DeltaT;

const MAX_SYS_PER_STAGE: usize = 5;
const TICKS_PER_SECOND: u64 = 20; // Default: 20, subject to change
// Additional value to push the Tickrate that's *just* short of next loop
const TICKRATE_EPS: Duration = Duration::from_millis(5);

// DO NOT TOUCH
const TICKRATE: Duration = Duration::from_millis(1000/TICKS_PER_SECOND);

type Stage = Vec<Box<dyn SystemWrapper>>;

/// # System Dispatcher
/// Handles the execution of the Systems within the app
/// 
/// Has 2 loops:
/// - Staller Loop -- Runs every frame
/// - Logic Loop - Runs inside Staller Loop at most N times per second, specified by the Tickrate
/// 
/// TODO: Make Tickrate adjustable at runtime
pub struct Dispatcher{
    _registry: HashMap<&'static str, SystemInfo>,
    preproc: Vec<Stage>,
    singlefires: HashMap<&'static str, Box<dyn SystemWrapper>>,
    logic: Vec<Stage>,
    event_responders: HashMap<&'static str, Vec<Box<dyn SystemWrapper>>>,
    postproc: Vec<Stage>
}
impl Dispatcher{
    /// Start building a new Dispatcher
    pub fn new() -> DispatcherBuilder{
        DispatcherBuilder::new()
    }
    /// Dispatch the Systems
    pub fn dispatch(&mut self, world: &mut World) -> Box<[i32]>{
        
        let mut last_frame = Instant::now();
        let mut last_tick = Instant::now();

        loop{
            // Update Frame Delta
            world.fetch_res_mut::<DeltaT>().set_delta_frame( last_frame.elapsed().as_millis());

            // -- PREPROCESSORS --
            for stage in self.preproc.iter_mut(){
                for system in stage.iter_mut(){
                    system.execute(world);
                }
            }

            // -- LOGIC LOOP --
            if last_tick.elapsed() >= TICKRATE + TICKRATE_EPS{
                // Update Logic Delta
                world.fetch_res_mut::<DeltaT>().set_delta_logic(last_tick.elapsed().as_millis());

                // -- Logic Systems --
                for stage in self.logic.iter_mut(){
                    for system in stage.iter_mut(){
                        system.execute(world);
                    }
                }
                // -- Singlefires --
                for trigger in world.take_triggers(){
                    // We have triggers for systems, but may not have any systems *to* trigger
                    if let Some(system) = self.singlefires.get_mut(trigger){
                        system.execute(world);
                    }
                }
                // -- Event Responders --
                for event in world.get_events().get_active_events(){
                    // We have events, but may not have any responders for them
                    if let Some(responders) = self.event_responders.get_mut(event){
                        for system in responders.iter_mut(){
                            system.execute(world);
                        }

                    }
                }
                // -- Commands --
                for mut command in world.take_commands(){
                    command.execute(world);
                }

                // Update last Logic Tick
                last_tick = Instant::now();

                world.fetch_res_mut::<DeltaT>().incr_logic_frame();
            }

            // -- POSTPROCESSORS --
            for stage in self.postproc.iter_mut(){
                for system in stage.iter_mut(){
                    system.execute(world);
                }
            }
            
            // Check system-level events
            {
                use super::events;
                // Borrow for an extended period of time
                let events = world.get_events();

                // App exit
                let event = events.get_reader::<events::ExitApp>();
                if event.event_count() > 0{
                    let error_codes = event.iter().map(|event| event.0).collect::<Box<[i32]>>();

                    eprintln!("{} requests for shutdown have been sent with following error codes: {:?}", event.event_count(), error_codes);

                    return error_codes
                }
            }

            // Clear Events
            world.swap_event_buffers();

            // Update last Frame Tick
            last_frame = Instant::now();

            world.fetch_res_mut::<DeltaT>().incr_frame();
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
    event_responders: HashMap<&'static str, Vec<Box<dyn SystemWrapper>>>,
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
            event_responders: HashMap::new(),
            postproc: StagesBuilder::new(),            
        }
    }
    /// Add a System to the Dispatcher
    pub fn add<S: System>(&mut self){
        // The System has the same ID but is not an override, we can't have it here
        if self.registry.contains_key(S::ID) && !S::OVERRIDE{
            panic!("ERROR: Conflicting system IDs {}\nDid you mean to override the System?", S::ID)
        }
        // Also acts as an auto override for the registry, neat
        self.registry.insert(S::ID, SystemInfo::new::<S>());

        match S::TYPE{
            SystemType::Preprocessor => self.preproc.add::<S>(),
            SystemType::Logic => self.logic.add::<S>(),
            SystemType::Singlefire => {
                self.singlefires.insert(S::ID, Box::new(S::new()));
            },
            SystemType::EventResponder(event_id) => {
                self.event_responders.entry(event_id).or_insert(Vec::new()).push(Box::new(S::new()));
            }
            SystemType::Postprocessor => self.postproc.add::<S>(),
        }
    }
    /// Verify dependencies of each System
    fn verify_deps(&self){
        for system in self.registry.values(){
            for dep in system.depends.iter(){
                if !self.registry.contains_key(dep){
                    panic!("ERROR: System {}'s dependency System {} does not exist", system.id, dep)
                }
            }
        }
    }
    /// Build the Dispatcher
    pub fn build(self) -> Dispatcher{

        self.verify_deps();

        Dispatcher{
            _registry: self.registry,
            preproc: self.preproc.build(),
            singlefires: self.singlefires,
            logic: self.logic.build(),
            event_responders: self.event_responders,
            postproc: self.postproc.build(),
        }
    }
}

/// # System Information
/// A collection of data for the Dispatcher's Registry to keep track of
/// 
/// Is typically only used as Metadata for debug purposes
pub struct SystemInfo{
    pub id: &'static str,
    pub depends: &'static [&'static str],
    pub run_ord: &'static [RunOrder],
    pub sys_type: SystemType,
    pub overr: bool
}
impl SystemInfo{
    fn new<S: System>() -> Self{
        Self{
            id: S::ID,
            depends: S::DEPENDS,
            run_ord: S::RUNORD,
            sys_type: S::TYPE,
            overr: S::OVERRIDE
        }
    }
}

/// # Stages Builder
/// Builds a stage graph for Dispatcher to execute using provided Systems
#[must_use]
struct StagesBuilder{
    systems: HashMap<&'static str, Box<dyn SystemWrapper>>
}
impl StagesBuilder{
    /// Start building a new collection of Stages
    fn new() -> Self{
        Self{
            systems: HashMap::new()
        }
    }
    /// Add a System to this builder
    fn add<S: System>(&mut self){
        // Realized there's no need to put overrides into a sepparate queue, we haven't started the build process yet, so we can safely override them right here
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
                                        (system.id(), system.run_order())
                                ).collect()
                ]);

        // Essentially iterate until everything is resolved
        for layer_id in 0..{

            // Unwrap: We only push new layers when there were shifts on previous layers
            // If there were none, we would break out of the loop
            let layer = graph.get(layer_id).unwrap();

            // Iterate over layer's Systems to see which we should shift
            for (system_id, order_deps) in layer.iter(){
                
                for order_dep in order_deps.iter(){

                    match order_dep{
                        // If we need this System to run before, we shift the other System to later
                        RunOrder::Before(id) => {
                            if layer.contains_key(id){
                                shifts.insert(*id);
                            }
                        },
                        // Equivalent of the other System having `Before(this_system)`
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

            // This should not happen unless there's a circular dependency between the Systems
            if shifts.len() == layer.len(){
                panic!("ERROR: There are circular run orders between {} Systems:\n{:#?}\nPlease resolve them", layer.len(), layer.keys())
            }

            // Push a new layer and move all the shifted Systems from current layer to next layer
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
    /// Build the Stages for Dispatcher to use
    fn build(mut self) -> Vec<Stage>{

        let mut stages = Vec::new();

        let graph = self.build_run_order_graph();

        // We don't need to use `.iter()` as the final graph will not be used for anything else, we also own it anyway
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
/// Systems work within 2 loops:
/// 1. Staller
///     - Preprocessors
///      2. Logic
///         - Logic
///         - Singlefires
///         - Event Responders
///         - Commands
///     - Postprocessors
/// 
/// Staller Systems are ran every frame, Logic Systems are ran *at most* `N` times per second, specified by the Tickrate
/// 
/// `Preprocessor` Systems are ran at the beggining of every frame  
/// They are typically used to update Resources
/// 
/// `Logic` Systems are the main Update Systems that run the game logic, this is the default System type
/// 
/// `Singlefire` Systems are ran at the will of other Systems  
/// To execute a System like this, another System needs to send a Trigger it through TriggerWriter  
/// You cannot daisy-chain Singlefires within same Tick, any Singlefires triggered by current Tick Singlefires will only be executed on the next Tick
/// 
/// `EventResponder` Systems are ran when their respective Event is present within the Read Buffer, signified by the ID  
/// They're effectively Logic Systems that listen for Events only and declutter the main Logic loop
/// 
/// `Postprocessor` Systems are ran at the end of every frame  
/// They are typically output Systems like Audio, Outgoing Network and Rendering
#[derive(Default)]
pub enum SystemType{
    Preprocessor,
    #[default]
    Logic,
    Singlefire,
    EventResponder(&'static str),
    Postprocessor
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::ECS::fetch::Request;
    mod loops{
        use super::*;
        use crate::ECS::resource::DeltaT;
        use crate::ECS::fetch::WriteEvent;
        use crate::ECS::events::ExitApp;

        struct Staller;
        struct Logic;

        impl System for Staller{
            type Data<'a> = (&'a DeltaT, WriteEvent<ExitApp>);
            const ID: &'static str = "_Staller";
            const TYPE: SystemType = SystemType::Preprocessor;
        
            fn new() -> Self {
                Self
            }
            fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {
                assert!(data.0.frame() >= data.0.logic_frame());
                if data.0.logic_frame() == 20{
                    eprintln!("Frames: {}, Logic frames: {}", data.0.frame(), data.0.logic_frame());
                    data.1.send(ExitApp(0));
                }
            }
        }
        impl System for Logic{
            type Data<'a> = &'a DeltaT;
            const ID: &'static str = "_Logic";
            const TYPE: SystemType = SystemType::Logic;
        
            fn new() -> Self {
                Self
            }
            fn execute(&mut self, data: Request<'_, Self::Data<'_>>) {
                assert!(data.frame() >= data.logic_frame());
            }
        }

        #[test]
        fn test(){
            let mut world = World::new();

            let mut builder = Dispatcher::new();
            builder.add::<Staller>();
            builder.add::<Logic>();

            let mut dispatcher = builder.build();
            dispatcher.dispatch(&mut world);
        }
    }
    mod deps{
        use super::*;

        struct BackDep;
        struct Dependee;
        struct ForwDep;

        impl System for BackDep{
            type Data<'a> = ();
            const ID: &'static str = "BackDep";
            const TYPE: SystemType = SystemType::Preprocessor;
        
            fn new() -> Self {
                Self
            }
            fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
                
            }
        }
        impl System for Dependee{
            type Data<'a> = ();
            const ID: &'static str = "Dependee";
            const DEPENDS: &'static [&'static str] = &[BackDep::ID, ForwDep::ID];

            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
                
            }
        }
        impl System for ForwDep{
            type Data<'a> = ();
            const ID: &'static str = "ForwDep";
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
                
            }
        }

        #[test]
        fn test_deps(){
            let mut builder = Dispatcher::new();
            builder.add::<Dependee>();
            builder.add::<BackDep>();
            builder.add::<ForwDep>();

            let _dispatcher = builder.build();
        }
        #[test]
        #[should_panic]
        fn test_missing(){
            let mut builder = Dispatcher::new();
            builder.add::<Dependee>();

            let _dispatcher = builder.build();
        }
    }
    mod run_ord{
        use super::*;
        use crate::ECS::resource::Resource;
        use crate::ECS::events::ExitApp;
        use crate::ECS::fetch::WriteEvent;

        struct SysA;
        struct SysB;
        struct SysC;

        struct Runs{
            a: bool,
            b: bool,
            c: bool
        }

        impl Resource for Runs{
            const ID: &'static str = "Runs";
        
            fn new() -> Self {
                Self{
                    a: false,
                    b: false,
                    c: false,
                }
            }
        }

        impl System for SysA{
            type Data<'a> = &'a mut Runs;
            const ID: &'static str = "SysA";
            const RUNORD: &'static [RunOrder] = &[RunOrder::Before(SysC::ID)];
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {
                data.a = true;

                assert!(data.a);
                assert!(!data.b);
                assert!(!data.c);
            }
        }

        impl System for SysB{
            type Data<'a> = &'a mut Runs;
            const ID: &'static str = "SysB";
            const RUNORD: &'static [RunOrder] = &[RunOrder::After(SysA::ID)];
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {
                data.b = true;

                assert!(data.a);
                assert!(data.b);
                assert!(!data.c);
            }
        }

        impl System for SysC{
            type Data<'a> = (&'a mut Runs, WriteEvent<ExitApp>);
            const ID: &'static str = "SysC";
            const RUNORD: &'static [RunOrder] = &[RunOrder::After(SysB::ID)];

            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {
                data.0.c = true;

                assert!(data.0.a);
                assert!(data.0.b);
                assert!(data.0.c);

                data.1.send(ExitApp(0));
            }
        }

        #[test]
        fn test(){
            let mut world = World::new();
            world.register_res::<Runs>();

            let mut builder = Dispatcher::new();
            builder.add::<SysA>();
            builder.add::<SysB>();
            builder.add::<SysC>();

            let mut dispatcher = builder.build();

            dispatcher.dispatch(&mut world);
        }
    }
    mod collision{
        use super::*;

        struct Sys;
        struct Collision;

        impl System for Sys{
            type Data<'a> = ();
            const ID: &'static str = "System";
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
                
            }
        }
        impl System for Collision{
            type Data<'a> = ();
            const ID: &'static str = "System";
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
                
            }
        }

        #[test]
        #[should_panic]
        fn test(){
            let mut builder = Dispatcher::new();
            builder.add::<Sys>();
            builder.add::<Collision>();
        }
    }
    mod overrides{
        use super::*;
        use crate::ECS::events::ExitApp;
        use crate::ECS::fetch::WriteEvent;

        struct Sys;
        struct Override;

        impl System for Sys{
            type Data<'a> = ();
            const ID: &'static str = "System";
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, _data: Request<'_, Self::Data<'_>>) {
                panic!("Did not override!")
            }
        }
        impl System for Override{
            type Data<'a> = WriteEvent<ExitApp>;
            const ID: &'static str = "System";
            const OVERRIDE: bool = true;
        
            fn new() -> Self {
                Self
            }
        
            fn execute(&mut self, mut data: Request<'_, Self::Data<'_>>) {
                data.send(ExitApp(0));
            }
        }

        #[test]
        fn test(){
            let mut world = World::new();

            let mut builder = Dispatcher::new();
            builder.add::<Sys>();
            builder.add::<Override>();

            let mut dispatcher = builder.build();
            dispatcher.dispatch(&mut world);
        }
    }
}
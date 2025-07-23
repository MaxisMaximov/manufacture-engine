use std::collections::{HashMap, HashSet};

use super::system::*;
use super::world::World;

mod stage;
use stage::*;

pub struct gmDispatcher{
    systems: HashMap<&'static str, usize>,
    stages: Vec<gmDispatchStage>
}
impl gmDispatcher{
    pub fn new() -> Self{
        Self{
            systems: HashMap::new(),
            // Singular item to avoid checks for empty vec later
            stages: Vec::from([gmDispatchStage::new()])
        }
    }

    pub fn withSys<T>(mut self) -> Self where T: System + 'static{
        self.addSys::<T>();
        self
    }

    pub fn addSys<T>(&mut self) where T: System + 'static{
        // Check if the system is registered already
        if self.systems.contains_key(T::ID){
            panic!("ERROR: Attempted to override an existing system: {}", T::ID)
        }

        let mut w_nextStage: usize = 0;
        
        'CHECKSTAGE:{
            // Exit early if there's no dependencies
            if T::DEPENDS.is_empty(){
                break 'CHECKSTAGE;
            }

            for DEPEND in T::DEPENDS.iter(){
                // Check if such dependency exists
                let STAGEID = self.systems.get(DEPEND).expect(&format!("ERROR: Dependency {} for system {} Is not registered", DEPEND, T::ID));
                if *STAGEID > w_nextStage{
                    w_nextStage = *STAGEID + 1
                }
            }
        }

        // Check if the desired stage exists
        if let Some(STAGE) = self.stages.get_mut(w_nextStage){
            STAGE.addSys::<T>();
            return
        }
        // If not, add a new stage with the system
        self.addStage(gmDispatchStage::new().withSys::<T>());

    }

    pub fn addStage(&mut self, IN_stage: gmDispatchStage){
        self.stages.push(IN_stage);
    }

    pub fn dispatch(&mut self, IN_world: &mut World){
        for STAGE in self.stages.iter_mut(){
            STAGE.dispatch(IN_world);
        }
        IN_world.end_tick();
    }
}

pub struct Dispatcher{
    registry: HashMap<&'static str, usize>,
    stages: Vec<Vec<Box<dyn SystemWrapper>>>
}
impl Dispatcher{
    pub fn new() -> Self{
        Self{
            registry: HashMap::new(),
            stages: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, World: &mut World){
        for stage in self.stages.iter_mut(){
            for system in stage.iter_mut(){
                system.execute(World);
            }
        }
    }
}

#[must_use]
pub struct DispatcherBuilder{
    registry: HashMap<&'static str, usize>,
    stages: Vec<Vec<Box<dyn SystemWrapper>>>
}
impl DispatcherBuilder{
    pub fn new() -> Self{
        Self{
            registry: HashMap::new(),
            stages: Vec::new(),
        }
    }

    pub fn with<S: System>(mut self) -> Self{
        self.add::<S>();
        self
    }

    pub fn add<S: System>(&mut self){
        // First check if we already registered the system
        if self.registry.contains_key(S::ID){
            panic!("ERROR: System {} already exists\nIf you wish to override this system, use `.overrides()` instead", S::ID);
        }

        // Check what stage the system should ideally be in
        let mut ideal_stage = 0;
        for dep in S::DEPENDS{
            if let Some(dep_stage) = self.registry.get(dep){
                // We found the dependency, check if it's later than the ideal stage
                ideal_stage = std::cmp::max(ideal_stage, *dep_stage)
            }else{
                match S::DEPRESOLVE{
                    DependResolve::Null => {},
                    DependResolve::RemoveSelf => return,
                    DependResolve::Panic => 
                        panic!("ERROR: System {}'s dependency {} does not exist", S::ID, dep)
                }
            }
        }

        // Find a suitable stage starting from Ideal one
        // Ideal stage is the earliest stage the system can be in
        for pos_stage in ideal_stage..{
            if let Some(stage) = self.stages.get_mut(pos_stage){
                // If the stage still has room in it, push the system
                if !stage.len() < 5{
                    stage.push(Box::new(S::new()));
                    self.registry.insert(S::ID, pos_stage);
                    break;
                }
            // If we've gone over all stages and found no suitable stage, make a new one
            }else{
                self.stages.push(Vec::new());
                self.stages.last_mut().unwrap().push(Box::new(S::new()));
                self.registry.insert(S::ID, pos_stage);
            }
        }
    }

    fn with_override<S: System>(mut self) -> Self{
        self.overrides::<S>();
        self
    }

    fn overrides<S: System>(&mut self){
        if let Some(stage_id) = self.registry.remove(S::ID) {
            let stage = self.stages.get_mut(stage_id).unwrap();
            stage.retain(|system| system.id() != S::ID);
        }else{
            panic!("ERROR: Attempted to override non-existing system: {}", S::ID)
        };
    }

    fn build(self) -> Dispatcher{
        Dispatcher{
            registry: self.registry,
            stages: self.stages,
        }
    }
}
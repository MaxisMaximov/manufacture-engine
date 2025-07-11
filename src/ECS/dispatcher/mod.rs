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
    pub fn with<S: System>(mut self) -> Self{
        if self.registry.contains_key(S::ID){
            panic!("ERROR: System {} already exists\nOverrides are not yet supported, if that's what you wanted to do", S::ID);
        }

        // Check what stage the system would ideally be in
        let mut ideal_stage = 0;
        for dep in S::DEPENDS{
            if !self.registry.contains_key(dep){
                panic!("ERROR: System {}'s dependency {} does not exist", S::ID, dep)
            }
            // The latest stage a dependency is in is the ideal stage
            ideal_stage = ideal_stage.max(*self.registry.get(dep).unwrap());
        }
        
        // Keep looping until we find a compatible stage
        loop{
            match self.stages.get_mut(ideal_stage){
                Some(stage) => {
                    // Check if the stage is at it's limits
                    if !stage.len() == 5{
                        stage.push(Box::new(S::new()));
                        break;
                    }else{
                        ideal_stage += 1
                    }
                },
                None => {
                    let mut stage: Vec<Box<dyn SystemWrapper>> = Vec::new();
                    stage.push(Box::new(S::new()));
                    self.stages.push(stage);
                    break;
                }
            }
        }

        self
    }
    pub fn dispatch(&mut self, World: &mut World){
        for stage in self.stages.iter_mut(){
            for system in stage.iter_mut(){
                system.execute(World);
            }
        }
    }
}
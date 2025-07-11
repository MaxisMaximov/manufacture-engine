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
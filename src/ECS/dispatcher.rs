use std::collections::HashMap;

use super::system::*;
use super::world::World;

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

        // Check if system's dependencies exist
        for dep in S::DEPENDS{
            if !self.registry.contains_key(dep){
                panic!("ERROR: System {}'s dependency system {} does not exist", S::ID, dep)
            }
        }
        
        // Check what stage the system should ideally be in
        let mut min_stage = 0;
        let mut max_stage = usize::MAX;
        for cond in S::RUNORD.iter(){
            match cond{
                RunOrder::Before(sys) => {
                    if let Some(stage) = self.registry.get(sys){
                        max_stage = std::cmp::min(min_stage, *stage) - 1
                    }
                },
                RunOrder::After(sys) => {
                    if let Some(stage) = self.registry.get(sys){
                        min_stage = std::cmp::max(min_stage, *stage)
                    }
                },
            }
        }

        if min_stage > max_stage{
            panic!("ERROR: Minimum possible stage for system {} is later than the maximum possible stage", S::ID)
        }

        // Find a suitable stage for the system
        // Ideal stage is the earliest stage the system can be in
        for final_stage in min_stage..max_stage{
            if let Some(stage) = self.stages.get_mut(final_stage){
                // If the stage still has room in it, push the system
                if !stage.len() < 5{
                    stage.push(Box::new(S::new()));
                    self.registry.insert(S::ID, final_stage);
                    break;
                }
            // If we've rached the end of stages and found no suitable stage, make a new one
            }else{
                self.stages.push(Vec::new());
                self.stages.last_mut().unwrap().push(Box::new(S::new()));
                self.registry.insert(S::ID, final_stage);
            }
        }

        // If it is still not registered, there must've been no available stage in range
        if !self.registry.contains_key(S::ID){
            // As such, we insert a new stage inbetween the possible stages
            // Since `insert` pushes all elements to the right, we can just insert a new stage at `max_stage`'s position`
            self.stages.insert(max_stage, Vec::new());
            self.stages.get_mut(max_stage).unwrap().push(Box::new(S::new()));
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
            self.add::<S>();
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
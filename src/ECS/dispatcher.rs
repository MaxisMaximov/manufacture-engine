use std::collections::HashMap;

use super::system::*;
use super::world::World;

pub struct Dispatcher{
    registry: HashMap<&'static str, usize>,
    preproc: StageManager,
    stages: StageManager,
    postproc: StageManager
}
impl Dispatcher{
    pub fn new() -> Self{
        Self{
            registry: HashMap::new(),
            preproc: StageManager::new(),
            stages: StageManager::new(),
            postproc: StageManager::new(),
        }
    }

    pub fn dispatch(&mut self, World: &mut World){
        self.preproc.dispatch(World);
        self.stages.dispatch(World);
        self.postproc.dispatch(World);
    }
}

#[must_use]
pub struct DispatcherBuilder{
    registry: HashMap<&'static str, usize>, // ID, Stage
    preproc: StageManagerBuilder,
    stages: StageManagerBuilder,
    postproc: StageManagerBuilder
}
impl DispatcherBuilder{
    pub fn new() -> Self{
        Self{
            registry: HashMap::new(),
            preproc: StageManagerBuilder::new(),
            stages: StageManagerBuilder::new(),
            postproc: StageManagerBuilder::new(),
        }
    }

    pub fn with<S: System>(mut self) -> Self{
        self.add::<S>();
        self
    }

    pub fn add<S: System>(&mut self){
        // First check if we already registered the system
        if self.registry.contains_key(S::ID){
            panic!("ERROR: System {} already exists\nOverrides are unavailable for now", S::ID);
        }

        // Check if system's dependencies exist
        for dep in S::DEPENDS{
            if !self.registry.contains_key(dep){
                panic!("ERROR: System {}'s dependency system {} does not exist", S::ID, dep)
            }
        }

        // See where it should go
        let final_stage = match S::TYPE{
            SystemType::Preprocessor => self.preproc.add::<S>(),
            SystemType::Normal => self.stages.add::<S>(),
            SystemType::Postprocessor => self.postproc.add::<S>(),
        };
        self.registry.insert(S::ID, final_stage);
    }

    fn build(self) -> Dispatcher{
        Dispatcher{
            registry: self.registry,
            preproc: self.preproc.build(),
            stages: self.stages.build(),
            postproc: self.postproc.build(),
        }
    }
}

struct StageManager{
    registry: HashMap<&'static str, usize>,
    stages: Vec<Vec<Box<dyn SystemWrapper>>>
}
impl StageManager{
    pub fn new() -> Self{
        Self{
            registry: HashMap::new(),
            stages: Vec::new()
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

struct StageManagerBuilder{
    registry: HashMap<&'static str, usize>,
    stages: Vec<Vec<Box<dyn SystemWrapper>>>
}
impl StageManagerBuilder{
    pub fn new() -> Self{
        Self{
            registry: HashMap::new(),
            stages: Vec::new(),
        }
    }

    pub fn add<S: System>(&mut self) -> usize{
        // We don't need to do any checks as the DispatcherBuilder already does that for us
        
        // Check what stage the system should ideally be in
        let mut min_stage = 0;
        let mut max_stage = usize::MAX;
        for cond in S::RUNORD.iter(){
            match cond{
                RunOrder::Before(sys) => {
                    if let Some(stage) = self.registry.get(sys){
                        // We can't be in the same stage as the system in `Before`, so we go a stage back
                        max_stage = std::cmp::min(min_stage, *stage - 1)
                    }
                },
                RunOrder::After(sys) => {
                    if let Some(stage) = self.registry.get(sys){
                        min_stage = std::cmp::max(min_stage, *stage)
                    }
                },
            }
        }

        // Something went wrong with the order
        if min_stage > max_stage{
            panic!("ERROR: Earliest possible stage for system {} is later than the latest possible stage", S::ID)
        }

        // Find a suitable stage for the system
        for final_stage in min_stage..max_stage{
            match self.stages.get_mut(final_stage) {
                Some(stage) => {
                    // Stage is full and we can't push anything else into it
                    if stage.len() >= 5{
                        continue
                    }

                    stage.push(Box::new(S::new()));
                    self.registry.insert(S::ID, final_stage);
                    return final_stage
                }
                // We've reached the end of the stages but found no suitable one
                None => {
                    self.stages.push(Vec::new());
                    // We can safely index it as we just pushed at `final_stage`
                    self.stages[final_stage].push(Box::new(S::new()));
                    self.registry.insert(S::ID, final_stage);
                    return final_stage;
                }
            }
        }

        // If we got here, there must've been no available stage in range
        // As such, we insert a new stage inbetween the possible stages
        // Since `insert` pushes all elements to the right, we can just 
        // insert a new stage at `max_stage`'s position
        self.stages.insert(max_stage, Vec::new());
        self.stages[max_stage].push(Box::new(S::new()));
        return max_stage

    }

    pub fn build(self) -> StageManager{
        StageManager{
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
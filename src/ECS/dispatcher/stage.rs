use super::*;

use world::*;

pub struct gmDispatchStage{
    systems: HashMap<&'static str, ()>,
    inner: Vec<Box<dyn gmSysRun>>
}
impl gmDispatchStage{
    pub fn new() -> Self{
        Self{
            systems: HashMap::new(),
            inner: Vec::new()
        }
    }

    pub fn withSys<T>(mut self) -> Self where T: gmSystem + 'static{
        self.addSys::<T>();
        self
    }

    pub fn addSys<T>(&mut self) where T: gmSystem + 'static{
        // The Dispatcher does the check if exists for us already so no need to check it
        self.systems.insert(T::SYS_ID(), ());
        self.inner.push(Box::new(T::new()));
    }

    pub fn checkSys<T>(&self) -> bool where T: gmSystem{
        self.checkSysID(T::SYS_ID())
    }
    
    pub fn checkSysID(&self, IN_id: &'static str) -> bool{
        match self.systems.get(IN_id){
            Some(_) => true,
            None => false,
        }
    }

    pub fn dispatch(&mut self, IN_world: &mut gmWorld){
        for SYS in self.inner.iter_mut(){
            SYS.executeNow(IN_world);
        }
    }
}
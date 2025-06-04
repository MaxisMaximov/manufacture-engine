use super::*;

use comp::*;
use storage::*;
use world::*;

#[must_use]
pub struct gmObjBuilder<'a>{
    gmObjID: usize,
    worldRef: &'a mut gmWorld
}
impl<'a> gmObjBuilder<'a>{
    pub fn new(IN_id: usize, IN_world: &'a mut gmWorld) -> Self{
        Self{
            gmObjID: IN_id,
            worldRef: IN_world,
        }
    }

    pub fn addComp<T>(self, IN_comp: T) -> Self where T:Component{
        self.worldRef.fetchMut::<T>().insert(self.gmObjID, IN_comp);
        self
    }

    pub fn finish(self){}
}
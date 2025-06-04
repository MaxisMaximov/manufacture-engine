use std::marker::PhantomData;

use super::*;
use comp::Component;
use prefab::gmPrefab;
use storage::gmStorage;
use world::gmWorld;

pub trait gmCommand: Any{
    fn CMD_ID(&self) -> &'static str;
    fn execute(&self, IN_world: &mut gmWorld);
}

pub struct cmd_SpawnGmObj{}
impl gmCommand for cmd_SpawnGmObj{
    fn CMD_ID(&self) -> &'static str {
        "cmd_SpawnGmObj"
    }

    fn execute(&self, IN_world: &mut gmWorld) {
        let _ = IN_world.createGmObj();
    }
}

pub struct cmd_DespawnGmObj{
    pub id: usize
}
impl gmCommand for cmd_DespawnGmObj{
    fn CMD_ID(&self) -> &'static str {
        "cmd_DespawnGmObj"
    }

    fn execute(&self, IN_world: &mut gmWorld) {
        let _ = IN_world.deleteGmObj(self.id);
    }
}

pub struct cmd_addComp<T: Component>{
    pub gmObj: usize,
    pub comp: T
}
impl<T: Component + Clone> gmCommand for cmd_addComp<T>{
    fn CMD_ID(&self) -> &'static str {
        "cmd_addComp"
    }

    fn execute(&self, IN_world: &mut gmWorld) {
        IN_world.fetchMut::<T>().insert(self.gmObj.clone(), self.comp.to_owned());
    }
}

pub struct cmd_removeComp<T: Component>{
    pub gmObj: usize,
    pub _phantom: PhantomData<T>
}
impl<T: Component> gmCommand for cmd_removeComp<T>{
    fn CMD_ID(&self) -> &'static str {
        "cmd"
    }

    fn execute(&self, IN_world: &mut gmWorld) {
        IN_world.fetchMut::<T>().remove(&self.gmObj);
    }
}

pub struct cmd_spawnPrefab<T: gmPrefab>{
    pub prefab: T
}
impl<T: gmPrefab> gmCommand for cmd_spawnPrefab<T>{
    fn CMD_ID(&self) -> &'static str {
        "cmd_SpawnPrefab"
    }

    fn execute(&self, IN_world: &mut gmWorld) {
        self.prefab.spawn(IN_world.createGmObj());
    }
}
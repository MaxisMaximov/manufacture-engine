use super::*;

use world::*;

pub trait gmSystem<'a>{
    type sysData: gmSystemData<'a>;
    type QUERY: fetch::QueryData;
    const sysDepends: &'static [&'static str];
    fn new() -> Self;
    fn SYS_ID() -> &'static str;
    fn execute(&mut self, IN_data: Self::sysData);
}

pub trait gmSysRun<'a>{
    fn executeNow(&mut self, IN_world: &'a mut gmWorld);
}
impl<'a, T> gmSysRun<'a> for T where T:gmSystem<'a>{
    fn executeNow(&mut self, IN_world: &'a mut gmWorld) {
        self.execute(T::sysData::fetch(IN_world));
    }
}

pub trait gmSystemData<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self;
}
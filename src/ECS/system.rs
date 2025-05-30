use super::*;

use world::*;

pub trait gmSystem{
    #[cfg(not(query_data))]
    type sysData<'a>: gmSystemData<'a>;
    #[cfg(query_data)]
    type QUERY: fetch::QueryData;

    const sysDepends: &'static [&'static str];

    fn new() -> Self;
    fn SYS_ID() -> &'static str;

    #[cfg(not(query_data))]
    fn execute<'a>(&mut self, IN_data: Self::sysData<'a>);
    #[cfg(query_data)]
    fn execute<'a>(&mut self, IN_data: Self::QUERY);
}

pub trait gmSysRun{
    fn executeNow<'a>(&mut self, IN_world: &'a mut gmWorld);
}
impl<T> gmSysRun for T where T:gmSystem{
    fn executeNow<'a>(&mut self, IN_world: &'a mut gmWorld) {
        #[cfg(not(query_data))]
        self.execute(T::sysData::fetch(IN_world));

        #[cfg(query_data)]
        self.execute(T::QUERY::fetch(IN_world))
    }
}

pub trait gmSystemData<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self;
}
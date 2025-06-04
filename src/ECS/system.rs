use super::*;

use world::*;
use fetch::*;

pub trait gmSystem{
    #[cfg(not(query_data))]
    type sysData: gmSystemData;
    #[cfg(query_data)]
    type QUERY: QueryData;

    fn new() -> Self;
    fn SYS_ID() -> &'static str;
    fn SYS_DEPENDS() -> &'static [&'static str];

    

    #[cfg(not(query_data))]
    fn execute(&mut self, IN_data: Self::sysData);
    #[cfg(query_data)]
    fn execute<'a>(&mut self, IN_data: Query<'_, Self::QUERY>);
}

pub trait gmSystemWrapper{
    fn execute<'a>(&mut self, IN_world: &'a mut gmWorld);
}
impl<T: gmSystem> gmSystemWrapper for T{
    
    fn execute<'a>(&mut self, IN_world: &'a mut gmWorld) {
        #[cfg(not(query_data))]
        self.execute(T::sysData::fetch(IN_world));

        #[cfg(query_data)]
        self.execute(Query::fetch(IN_world))
    }
}
}

pub trait gmSystemData<'a>{
    fn fetch(IN_world: &'a mut gmWorld) -> Self;
}
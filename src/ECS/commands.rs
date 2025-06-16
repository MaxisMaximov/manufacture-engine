use std::any::Any;
use super::world::gmWorld;

pub trait Command: Any{
    const ID: &'static str;
    fn execute(&mut self, World: &mut gmWorld);
}

pub trait CommandWrapper{
    fn id(&self) -> &'static str;
    fn execute(&mut self, World: &mut gmWorld);
}

impl<T: Command> CommandWrapper for T{
    fn id(&self) -> &'static str {
        T::ID
    }

    fn execute(&mut self, World: &mut gmWorld) {
        Command::execute(self, World);
    }
}

impl dyn CommandWrapper{
    pub fn downcast_ref<T: Command>(&self) -> Option<&T>{
        if T::ID == self.id(){
            // SAFETY: We have a check for matching IDs beforehand
            Some(unsafe{
                &*(self as *const dyn CommandWrapper as *const T)
            })
        }else{
            None
        }
    }
    pub fn downcast_mut<T: Command>(&mut self) -> Option<&mut T>{
        if T::ID == self.id(){
            // SAFETY: We have a check for matching IDs beforehand
            Some(unsafe {
                &mut *(self as *mut dyn CommandWrapper as *mut T)
            })
        }else{
            None
        }
    }
}
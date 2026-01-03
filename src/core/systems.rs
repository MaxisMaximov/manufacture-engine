use super::*;
use resources::*;

/// # Command Line Input getter
/// Acquires the current pressed key from the Command Line
/// 
/// Note: Some terminals may put `Press` and `Hold` events at the same time
/// 
/// TODO: Fix the double input issue
pub struct CMDInputHandler;
impl System for CMDInputHandler{
    type Data = &'static mut CMDInput;

    const ID: &'static str = "CMDInput";

    const TYPE: SystemType = SystemType::Preprocessor;

    fn new() -> Self {
        Self
    }

    fn execute(&mut self, mut data: Request<'_, Self::Data>) {
        use crossterm::event::{Event, read, poll};
        if poll(std::time::Duration::from_millis(0)).unwrap(){
            if let Event::Key(key) = read().unwrap(){
                // Triple Deref, whoops
                data.set(key)
            }
        }else{
            data.reset();
        }
    }
}
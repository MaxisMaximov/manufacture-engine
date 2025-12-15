use std::ops::{Deref, DerefMut};

use super::*;

/// # Delta Time Resources
/// Tracks milliseconds elapsed since last frame and last Logic run. Use provided `delta_frame` and `delta_logic` methods to get the time
/// 
/// Only the Dispatcher is allowed modify the inner values, **DO NOT SET IT MANUALLY**
pub struct DeltaT{
    delta_frame: u128,
    delta_logic: u128,
    frame: u64,
    logic_frame: u64
}
impl DeltaT{
    /// Get milliseconds elapsed since last frame
    pub fn delta_frame(&self) -> u128{
        self.delta_frame
    }
    /// Get milliseconds elapsed since last time Logic ran
    pub fn delta_logic(&self) -> u128{
        self.delta_logic
    }
    /// Get the number of frames elapsed since the app started
    pub fn frame(&self) -> u64{
        self.frame
    }
    /// Get the number of logic loops that were ran since the app started
    pub fn logic_frame(&self) -> u64{
        self.logic_frame
    }
    /// ## DO NOT USE THIS
    /// Only the Dispatcher is allowed to modify the Delta
    /// 
    /// Set the new Delta Frame value
    pub fn set_delta_frame(&mut self, Delta: u128){
        self.delta_frame = Delta
    }
    /// ## DO NOT USE THIS
    /// Only the Dispatcher is allowed to modify the Delta
    /// 
    /// Set the new Delta Logic value
    pub fn set_delta_logic(&mut self, Delta: u128){
        self.delta_logic = Delta
    }
    /// ## DO NOT USE THIS
    /// Only the Dispatcher is allowed to increment Frame count
    /// 
    /// Increment Staller loop count
    pub fn incr_frame(&mut self){
        self.frame += 1
    }
    /// ## DO NOT USE THIS
    /// Only the Dispatcher is allowed to increment Frame count
    /// 
    /// Increment Logic loop count
    pub fn incr_logic_frame(&mut self){
        self.logic_frame += 1
    }
}
impl Resource for DeltaT{
    const ID: &'static str = "DeltaT";

    fn new() -> Self {
        Self{
            delta_frame: 0,
            delta_logic: 0,
            frame: 0,
            logic_frame: 0
        }
    }
}

pub use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
/// # User input -- CMD
/// Stores the input provided by the Command Line
/// 
/// See `crossterm`'s `KeyEvent` for more
/// 
/// TODO: Remove dependency on Crossterm
pub struct CMDInput{
    key: KeyEvent
}
impl CMDInput{
    /// Get the current key
    pub fn get(&self) -> KeyEvent {
        self.key
    }
    /// Set the current key
    pub fn set(&mut self, key: KeyEvent){
        self.key = key
    }
    /// Set key back to Null
    pub fn reset(&mut self){
        self.key = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)
    }
}
impl Resource for CMDInput{
    const ID: &'static str = "CMDInputData";

    fn new() -> Self {
        Self{
            key: KeyEvent::new(KeyCode::Null, KeyModifiers::NONE),
        }
    }
}
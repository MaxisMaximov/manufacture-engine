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
    pub fn frame(&self) -> u64{
        self.frame
    }
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
    pub fn incr_frame(&mut self){
        self.frame += 1
    }
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
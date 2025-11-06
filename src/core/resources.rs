use super::ECS;

use ECS::resource::Resource;

/// # Delta Time Resources
/// Tracks milliseconds elapsed since last frame and last Logic run. Use provided `delta_frame` and `delta_logic` methods to get the time
/// 
/// Only the Dispatcher is allowed modify the inner values, **DO NOT SET IT MANUALLY**
pub struct DeltaT{
    delta_frame: u128,
    delta_logic: u128
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
    /// Set the new Delta Frame value
    /// 
    /// ## DO NOT USE THIS
    /// Only the DIspatcher is allowed to modify the Delta
    pub fn set_delta_frame(&mut self, Delta: u128){
        self.delta_frame = Delta
    }
    /// Set the new Delta Logic value
    /// 
    /// ## DO NOT USE THIS
    /// Only the Dispatcher is allowed to modify the Delta
    pub fn set_delta_logic(&mut self, Delta: u128){
        self.delta_logic = Delta
    }
}
impl Resource for DeltaT{
    const ID: &'static str = "DeltaT";

    fn new() -> Self {
        Self{
            delta_frame: 0,
            delta_logic: 0
        }
    }
}
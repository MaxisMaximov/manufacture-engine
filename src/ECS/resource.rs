/// # System Resource trait
/// Defines a Resource that can be shared between systems
/// 
/// Basically an equivalent of Unity/Unreal's singletons
/// 
/// ## WARNING
/// Make sure your Resource ID does not collide with other IDs from other plugins
pub trait Resource: 'static{
    const ID: &'static str;
    /// Create a new instance of this Resource
    fn new() -> Self;
}

/// # Resource trait Wrapper
/// A wrapper trait for Resources to safely store them in the World
/// 
/// Provides ID method for identifying and Downcast methods to get the underlying Resource
pub(crate) trait ResourceWrapper{
    /// Get the underlying Resource's ID
    fn id(&self) -> &'static str;
}

impl<T: Resource> ResourceWrapper for T{
    fn id(&self) -> &'static str {
        T::ID
    }
}

impl dyn ResourceWrapper{
    /// Downcast to a reference of `T` resource
    ///
    /// Returns None if the ID of the `T` resource does not match the underlying Resource's ID
    pub fn downcast_ref<T: Resource>(&self) -> Option<&T>{
        if T::ID == self.id(){
            // SAFETY: We have a check for matching IDs beforehand
            Some(unsafe {
                &*(self as *const dyn ResourceWrapper as *const T)
            })
        }else{
            None
        }
    }
    /// Downcast to a mutable reference of `T` resource
    ///
    /// Returns None if the ID of the `T` resource does not match the underlying Resource's ID
    pub fn downcast_mut<T: Resource>(&mut self) -> Option<&mut T>{
        if T::ID == self.id(){
            // SAFETY: We have a check for matching IDs beforehand
            Some(unsafe {
                &mut *(self as *mut dyn ResourceWrapper as *mut T)
            })
        }else{
            None
        }
    }
}

/// # Delta Time Resources
/// Tracks milliseconds elapsed since last frame and last Logic run. Use provided `delta_frame` and `delta_logic` methods to get the time
pub struct DeltaT{
    delta_frame: u128,
    delta_frame_f32: f32,
    delta_logic: u128,
    delta_logic_f32: f32,
    frame: u64,
    logic_frame: u64,
    app_start: std::time::Instant
}
impl DeltaT{
    /// Get microseconds elapsed since last frame
    pub fn delta_frame(&self) -> u128{
        self.delta_frame
    }
    /// Get microseconds elapsed since last Logic frame
    pub fn delta_logic(&self) -> u128{
        self.delta_logic
    }
    pub fn delta_frame_f32(&self) -> f32{
        self.delta_frame_f32
    }
    pub fn delta_logic_f32(&self) -> f32{
        self.delta_logic_f32
    }
    /// Get the number of frames elapsed since the app started
    pub fn frame(&self) -> u64{
        self.frame
    }
    /// Get the number of logic loops that were ran since the app started
    pub fn logic_frame(&self) -> u64{
        self.logic_frame
    }
    pub fn delta_app_start(&self) -> u128{
        self.app_start.elapsed().as_micros()
    }
    pub fn delta_app_start_f32(&self) -> f32{
        self.app_start.elapsed().as_secs_f32()
    }
    /// Set the new Delta Frame values
    pub(crate) fn set_delta_frame(&mut self, delta_u128: u128, delta_f32: f32){
        self.delta_frame = delta_u128;
        self.delta_frame_f32 = delta_f32;
    }
    /// Set the new Delta Logic values
    pub(crate) fn set_delta_logic(&mut self, delta_u128: u128, delta_f32: f32){
        self.delta_logic = delta_u128;
        self.delta_logic_f32 = delta_f32;
    }
    /// Increment Staller loop count
    pub(crate) fn incr_frame(&mut self){
        self.frame += 1
    }
    /// Increment Logic loop count
    pub(crate) fn incr_logic_frame(&mut self){
        self.logic_frame += 1
    }
}
impl Resource for DeltaT{
    const ID: &'static str = "DeltaT";

    fn new() -> Self {
        Self{
            delta_frame: 0,
            delta_frame_f32: 0.0,
            delta_logic: 0,
            delta_logic_f32: 0.0,
            frame: 0,
            logic_frame: 0,
            app_start: std::time::Instant::now()
        }
    }
}
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
pub trait ResourceWrapper{
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
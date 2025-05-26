use super::*;

use comp::gmComp;

pub trait gmStorage<T: gmComp>: Any{
    fn new() -> Self;
    fn get(&self, IN_id: &usize) -> Option<&T>;
    fn get_mut(&mut self, IN_id: &usize) -> Option<&mut T>;
    fn insert(&mut self, IN_id: usize, IN_item: T);
    fn remove(&mut self, IN_id: &usize) -> Option<T>;
    fn iter(&self) -> impl Iterator;
    fn iter_mut(&mut self) -> impl Iterator;
}
pub trait gmStorageDrop: Any{
    fn drop(&mut self, IN_id: &usize);
}

impl dyn gmStorageDrop{
    pub fn downcast<T: gmComp>(&self) -> &gmStorageContainer<T>{
        unsafe {&*(self as *const dyn gmStorageDrop as *const gmStorageContainer<T>)}
    }
    pub fn downcast_mut<T: gmComp>(&mut self) -> &mut gmStorageContainer<T>{
        unsafe {&mut *(self as *mut dyn gmStorageDrop as *mut gmStorageContainer<T>)}
    }
}

// Necessary abstraction for `gmStorageDrop` to be used
pub struct gmStorageContainer<T:gmComp>{
    pub inner: T::COMP_STORAGE
}
impl<T: gmComp + 'static> gmStorageDrop for gmStorageContainer<T>{
    fn drop(&mut self, IN_id: &usize) {
        self.inner.remove(IN_id);
    }
}


pub struct denseVecStorage<T>{
    pub proxyMap: HashMap<usize, usize>,
    pub inner: Vec<denseVecEntry<T>>
}
impl<T: gmComp + 'static> gmStorage<T> for denseVecStorage<T>{

    fn new() -> Self {
        Self{
            proxyMap: HashMap::new(),
            inner: Vec::new()
        }
    }

    fn get(&self, IN_id: &usize) -> Option<&T> {
        match self.proxyMap.get(&IN_id){
            Some(ID) => {
                Some(&self.inner.get(*ID).unwrap().val)
            },
            None => None,
        }
    }

    fn get_mut(&mut self, IN_id: &usize) -> Option<&mut T> {
        match self.proxyMap.get(&IN_id){
            Some(ID) => {
                Some(&mut self.inner.get_mut(*ID).unwrap().val)
            },
            None => None,
        }
    }

    fn insert(&mut self, IN_id: usize, IN_item: T) {
        if self.proxyMap.contains_key(&IN_id){return}

        self.proxyMap.insert(IN_id, self.inner.len()); // Vec length is always the next free index
        self.inner.push(
            denseVecEntry{
                id: IN_id,
                val: IN_item,
            }
        );
    }

    fn remove(&mut self, IN_id: &usize) -> Option<T> {
        
        if let Some(INDEX) = self.proxyMap.remove(&IN_id){

            if INDEX == self.inner.len() - 1{
                return Some(self.inner.pop().unwrap().val)
            }

            *self.proxyMap.get_mut(&self.inner.last().unwrap().id).unwrap() = INDEX;

            return Some(self.inner.swap_remove(INDEX).val);
        }
        None

    }

    fn iter(&self) -> impl Iterator {
        self.inner.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator {
        self.inner.iter_mut()
    }
}
pub struct denseVecEntry<T>{
    pub id: usize,
    pub val: T
}
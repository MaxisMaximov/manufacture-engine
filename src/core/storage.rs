use std::collections::HashMap;

use super::ECS;

use ECS::storage::Storage;
use ECS::comp::Component;

/// # Vec Storage
/// The simplest component storage possible
/// 
/// Uses a single Vec under the hood and iterates over it for operations
/// 
/// It's generally recommended to use it only when a given component has *very* little use
pub struct VecStorage<C: Component>{
    inner: Vec<(usize, C)>
}
impl<C: Component> Storage<C> for VecStorage<C>{
    fn new() -> Self {
        Self{
            inner: Vec::new(),
        }
    }

    fn insert(&mut self, Index: usize, Comp: C) {
        if self.inner.iter().find(|(id, _)|*id == Index).is_none(){
            self.inner.push((Index, Comp));
        }
    }
    fn remove(&mut self, Index: &usize) {
        if let Some(id) = self.inner.iter().position(|(id, _)| id == Index){
            self.inner.remove(id);
        }
    }

    fn get(&self, Index: &usize) -> Option<&C> {
        self.inner.iter().find(|(id, _)| id == Index).map(|(_, comp)| comp)
    }
    fn get_mut(&mut self, Index: &usize) -> Option<&mut C> {
        self.inner.iter_mut().find(|(id, _)| id == Index).map(|(_, comp)| comp)
    }
}

/// # HashMap Storage
/// 
/// Essentially a wrapper over a HashMap
/// 
/// It's generally recommended to use it for components that are sparsely used across entities
pub struct HashMapStorage<C: Component>{
    inner: HashMap<usize, C>
}
impl<C: Component> Storage<C> for HashMapStorage<C>{
    fn new() -> Self {
        Self{
            inner: HashMap::new(),
        }
    }

    fn insert(&mut self, Index: usize, Comp: C) {
        self.inner.insert(Index, Comp);
    }
    fn remove(&mut self, Index: &usize) {
        self.inner.remove(Index);
    }

    fn get(&self, Index: &usize) -> Option<&C> {
        self.inner.get(Index)
    }
    fn get_mut(&mut self, Index: &usize) -> Option<&mut C> {
        self.inner.get_mut(Index)
    }
}
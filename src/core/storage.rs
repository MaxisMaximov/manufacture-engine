use std::collections::{BTreeMap, HashMap};

use super::*;

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

/// # BTreeMap Storage
/// 
/// Essentially a wrapper over BTreeMap
/// 
/// It's generally recommended to use this for components that will be on nearly all entities
pub struct BTreeMapStorage<C: Component>{
    inner: BTreeMap<usize, C>
}
impl<C: Component> Storage<C> for BTreeMapStorage<C>{
    fn new() -> Self {
        Self{
            inner: BTreeMap::new(),
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

/// # DenseVecStorage
/// A Vec Storage that uses a Hashmap as fast index proxy
/// 
/// Best of HashMap and Vec Storages, with the density of Vec and fast access time of HashMap
/// 
/// It's generally recommended to use this for sparsely populated, but heavy components
pub struct DenseVecStorage<C: Component>{
    proxy: HashMap<usize, usize>,
    inner: Vec<(usize, C)>
}
impl<C: Component> Storage<C> for DenseVecStorage<C>{
    fn new() -> Self {
        Self{
            proxy: HashMap::new(),
            inner: Vec::new(),
        }
    }

    fn insert(&mut self, Index: usize, Comp: C) {
        if self.proxy.contains_key(&Index){
            return
        }

        self.proxy.insert(Index, self.inner.len());
        self.inner.push((Index, Comp));
    }
    fn remove(&mut self, Index: &usize) {
        if let Some(inner_index) = self.proxy.remove(Index){
            self.inner.swap_remove(inner_index);
            // It was the only element in the Storage, we return early
            if self.inner.is_empty(){
                return
            }
            // Now we update the proxy map value that linked to the last element
            let to_update = self.inner[*Index].0;
            *self.proxy.get_mut(&to_update).unwrap() = *Index;
        }
    }

    fn get(&self, Index: &usize) -> Option<&C> {
        let index = self.proxy.get(Index)?;
        // Bit of a mess here
        self.inner.get(*index).map(|(_, comp)| comp)
    }
    fn get_mut(&mut self, Index: &usize) -> Option<&mut C> {
        let index = self.proxy.get(Index)?;
        self.inner.get_mut(*index).map(|(_, comp)| comp)
    }
}
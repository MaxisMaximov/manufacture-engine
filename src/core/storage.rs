use std::collections::{BTreeMap, HashMap};

use super::*;

/// # Vec Storage
/// The simplest Component storage possible
/// 
/// Uses a single Vec under the hood and iterates over it for operations
/// 
/// It's generally recommended to use it only when a given Component has *very* little use
pub struct VecStorage<C: Component>{
    inner: Vec<(usize, C)>
}
impl<C: Component> Storage<C> for VecStorage<C>{
    fn new() -> Self {
        Self{
            inner: Vec::new(),
        }
    }

    fn insert(&mut self, id: usize, comp: C) {
        if self.inner.iter().find(|(index, _)|*index == id).is_none(){
            self.inner.push((id, comp));
        }
    }
    fn remove(&mut self, id: &usize) {
        if let Some(index) = self.inner.iter().position(|(index, _)| index == id){
            self.inner.remove(index);
        }
    }

    fn get(&self, id: &usize) -> Option<&C> {
        self.inner.iter().find(|(index, _)| index == id).map(|(_, comp)| comp)
    }
    fn get_mut(&mut self, id: &usize) -> Option<&mut C> {
        self.inner.iter_mut().find(|(index, _)| index == id).map(|(_, comp)| comp)
    }
}

/// # HashMap Storage
/// 
/// Essentially a wrapper over a HashMap
/// 
/// It's generally recommended to use it for Components that are sparsely used across entities
pub struct HashMapStorage<C: Component>{
    inner: HashMap<usize, C>
}
impl<C: Component> Storage<C> for HashMapStorage<C>{
    fn new() -> Self {
        Self{
            inner: HashMap::new(),
        }
    }

    fn insert(&mut self, id: usize, comp: C) {
        self.inner.insert(id, comp);
    }
    fn remove(&mut self, id: &usize) {
        self.inner.remove(id);
    }

    fn get(&self, id: &usize) -> Option<&C> {
        self.inner.get(id)
    }
    fn get_mut(&mut self, id: &usize) -> Option<&mut C> {
        self.inner.get_mut(id)
    }
}

/// # BTreeMap Storage
/// 
/// Essentially a wrapper over BTreeMap
/// 
/// It's generally recommended to use this for Components that will be on nearly all entities
pub struct BTreeMapStorage<C: Component>{
    inner: BTreeMap<usize, C>
}
impl<C: Component> Storage<C> for BTreeMapStorage<C>{
    fn new() -> Self {
        Self{
            inner: BTreeMap::new(),
        }
    }

    fn insert(&mut self, id: usize, comp: C) {
        self.inner.insert(id, comp);
    }
    fn remove(&mut self, id: &usize) {
        self.inner.remove(id);
    }

    fn get(&self, id: &usize) -> Option<&C> {
        self.inner.get(id)
    }
    fn get_mut(&mut self, id: &usize) -> Option<&mut C> {
        self.inner.get_mut(id)
    }
}

/// # DenseVecStorage
/// A Vec Storage that uses a Hashmap as fast key proxy
/// 
/// Best of HashMap and Vec Storages, with the density of Vec and fast access time of HashMap
/// 
/// It's generally recommended to use this for sparsely populated, but heavy Components
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

    fn insert(&mut self, id: usize, comp: C) {
        if self.proxy.contains_key(&id){
            return
        }

        self.proxy.insert(id, self.inner.len());
        self.inner.push((id, comp));
    }
    fn remove(&mut self, id: &usize) {
        if let Some(index) = self.proxy.remove(id){
            self.inner.swap_remove(index);
            // It was the only element in the Storage, we return early
            if self.inner.is_empty(){
                return
            }
            // Now we update the proxy map value that linked to the last element
            let to_update = self.inner[*id].0;
            *self.proxy.get_mut(&to_update).unwrap() = *id;
        }
    }

    fn get(&self, id: &usize) -> Option<&C> {
        let index = self.proxy.get(id)?;
        // Bit of a mess here
        self.inner.get(*index).map(|(_, comp)| comp)
    }
    fn get_mut(&mut self, id: &usize) -> Option<&mut C> {
        let index = self.proxy.get(id)?;
        self.inner.get_mut(*index).map(|(_, comp)| comp)
    }
}
use std::cell::{RefMut, Ref};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use super::world::gmWorld;
use super::*;

use comp::Component;
use events::gmEvent;

pub struct Fetch<'a, T>{
    data: Ref<'a, T>,
}
impl<'a, T> Fetch<'a, T>{
    pub fn new(IN_data: Ref<'a, T>) -> Self{
        Self{
            data: IN_data
        }
    }
}
impl<'a, T> Deref for Fetch<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
pub struct FetchMut<'a, T>{
    data: RefMut<'a, T>
}
impl<'a, T> FetchMut<'a, T>{
    pub fn new(IN_data: RefMut<'a, T>) -> Self{
        Self{
            data: IN_data
        }
    }
}
impl<'a, T> Deref for FetchMut<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, T> DerefMut for FetchMut<'a, T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

// So I'm thinking
// This thing is basically a wrapper, over a wrapper (Fetch/Mut), over another wrapper (Ref/Mut)
// And they all 3 implement Derefs, however Ref/Mut use it for safety stuff, Fetch/Mut and StorageRef only use it to give direct access to the storage/resource
// So in the end they both don't provide anything useful other than type clarity to what's a component and what's a resource fetch
// Also I'm not sure if Deref³ is a good idea for performance
pub struct StorageRef<'a, T: Component, D>{
    data: D,
    _phantom: PhantomData<&'a T>
} 
impl<'a, T: Component, D> StorageRef<'a, T, D>{
    pub fn new(IN_data: D) -> Self{
        Self{
            data: IN_data,
            _phantom: PhantomData,
        }
    }
}
impl<'a, T: Component, D> Deref for StorageRef<'a, T, D>{
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, T: Component, D> DerefMut for StorageRef<'a, T, D>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(type_alias_bounds)]
pub type EventReader<'a, T: gmEvent> = Fetch<'a, VecDeque<T>>;
#[allow(type_alias_bounds)]
pub type EventWriter<'a, T: gmEvent> = FetchMut<'a, VecDeque<T>>;

#[allow(type_alias_bounds)]
pub type ReadStorage<'a, T: Component> = StorageRef<'a, T, Fetch<'a, T::STORAGE>>;
#[allow(type_alias_bounds)]
pub type WriteStorage<'a, T: Component> = StorageRef<'a, T, FetchMut<'a, T::STORAGE>>;

/// # Query fetch trait
/// Required for `Query` to know what to fetch from the World
/// 
/// It is implemented by default on `&` and `&mut` Component references, as well as Tuples up to 4 elements
/// 
/// The return type `Item` is typically the type the trait gets implemented on
pub trait QueryData{
    type Item<'b>;

    /// Fetch the data from the world
    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a>;
}

/// # World Query
/// Struct that queries the World and fetches the specified `QueryData`
pub struct Query<'a, C: QueryData>{
    data: C::Item<'a>
}
impl<'a, D: QueryData> Query<'a, D>{
    pub fn fetch(World: &'a gmWorld) -> Self{
        Self{
            data: D::fetch(World)
        }
    }
}
impl<'a, C:QueryData> Deref for Query<'a, C>{
    type Target = C::Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, C: QueryData> DerefMut for Query<'a, C>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T:Component> QueryData for &T{
    type Item<'b> = ReadStorage<'b, T>;

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        World.fetch()
    }
}

impl<T: Component> QueryData for &mut T{
    type Item<'b> = WriteStorage<'b, T>;

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        World.fetchMut()
    }
}


impl QueryData for (){
    type Item<'b> = ();

    fn fetch<'a>(_World: &'a gmWorld) -> Self::Item<'a>{}
}
impl<A: QueryData, B: QueryData> QueryData for (A, B){
    type Item<'b> = (A::Item<'b>, B::Item<'b>);

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World))
    }
}
impl<A: QueryData, B: QueryData, C: QueryData> QueryData for (A, B, C){
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>);

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World))
    }
}
impl<A: QueryData, B: QueryData, C: QueryData, D: QueryData> QueryData for (A, B, C, D){
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>, D::Item<'b>);

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World), D::fetch(World))
    }
}
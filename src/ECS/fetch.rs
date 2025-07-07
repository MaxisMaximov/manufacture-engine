#![allow(type_alias_bounds)]
use std::cell::{RefMut, Ref};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};


use super::world::World;

use super::comp::Component;
use super::events::Event;
use super::resource::Resource;
use super::commands::CommandWrapper;

pub type Fetch<'a, C: Component> = Ref<'a, C::STORAGE>;
pub type FetchMut<'a, C: Component> = RefMut<'a, C::STORAGE>;

pub type FetchRes<'a, R: Resource> = Ref<'a, R>;
pub type FetchResMut<'a, R: Resource> = RefMut<'a, R>;

pub type EventReader<'a, E: Event> = Ref<'a, VecDeque<E>>;
pub type EventWriter<'a, E: Event> = RefMut<'a, VecDeque<E>>;

pub type CommandWriter<'a> = RefMut<'a, Vec<Box<dyn CommandWrapper>>>;

/// # Query fetch trait
/// Required for `Query` to know what to fetch from the World
/// 
/// It is implemented by default on `&` and `&mut` Component references, as well as Tuples up to 4 elements
/// 
/// The return type `Item` is typically the type the trait gets implemented on
pub trait QueryData{
    type Item<'b>;

    /// Fetch the data from the world
    fn fetch<'a>(World: &'a World) -> Self::Item<'a>;
}

/// # World Query
/// Struct that queries the World and fetches the specified `QueryData`
pub struct Query<'a, D: QueryData>{
    data: D::Item<'a>
}
impl<'a, D: QueryData> Query<'a, D>{
    pub fn fetch(World: &'a World) -> Self{
        Self{
            data: D::fetch(World)
        }
    }
}
impl<'a, D:QueryData> Deref for Query<'a, D>{
    type Target = D::Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, D: QueryData> DerefMut for Query<'a, D>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}


impl<T:Component> QueryData for &T{
    type Item<'b> = Fetch<'b, T>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch::<T>()
    }
}
impl<T: Component> QueryData for &mut T{
    type Item<'b> = FetchMut<'b, T>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_mut::<T>()
    }
}


impl QueryData for (){
    type Item<'b> = ();

    fn fetch<'a>(_World: &'a World) -> Self::Item<'a>{}
}
impl<A, B> QueryData for (A, B)
where 
    A: QueryData, 
    B: QueryData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World))
    }
}
impl<A, B, C> QueryData for (A, B, C)
where 
    A: QueryData,
    B: QueryData,
    C: QueryData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World))
    }
}
impl<A, B, C, D> QueryData for (A, B, C, D)
where 
    A: QueryData,
    B: QueryData,
    C: QueryData,
    D: QueryData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>, D::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World), D::fetch(World))
    }
}

/// # Request fetch trait
/// Required for `Request` to know what system resources to fetch from the World
/// 
/// It is implemented by default on `&` and `&mut` Resource references, 
/// Event Readers and Writers, the Command Writer, as well as Tuples up to 4 elements
/// 
/// The return type `Item` is typically the type the trait gets implemented on
pub trait RequestData{
    type Item<'b>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a>;
}

/// # System resource Request
/// Struct that requests desired system resources from the World
pub struct Request<'a, D: RequestData>{
    data: D::Item<'a>
}
impl<'a, D: RequestData> Request<'a, D>{
    pub fn fetch(World: &'a World) -> Self{
        Self{
            data: D::fetch(World),
        }
    }
}
impl<'a, D: RequestData> Deref for Request<'a, D>{
    type Target = D::Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, D: RequestData> DerefMut for Request<'a, D>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Resource> RequestData for &T{
    type Item<'b> = FetchRes<'b, T>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_res()
    }
}
impl<T: Resource> RequestData for &mut T{
    type Item<'b> = FetchResMut<'b, T>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_res_mut()
    }
}

impl<E: Event> RequestData for EventReader<'_, E>{
    type Item<'b> = EventReader<'b, E>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.get_event_reader()
    }
}
impl<E: Event> RequestData for EventWriter<'_, E>{
    type Item<'b> = EventWriter<'b, E>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.get_event_writer()
    }
}

impl RequestData for (){
    type Item<'b> = ();

    fn fetch<'a>(_World: &'a World) -> Self::Item<'a>{}
}
impl<A, B> RequestData for (A, B)
where 
    A: RequestData, 
    B: RequestData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World))
    }
}
impl<A, B, C> RequestData for (A, B, C)
where 
    A: RequestData,
    B: RequestData,
    C: RequestData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World))
    }
}
impl<A, B, C, D> RequestData for (A, B, C, D)
where 
    A: RequestData,
    B: RequestData,
    C: RequestData,
    D: RequestData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>, D::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World), D::fetch(World))
    }
}
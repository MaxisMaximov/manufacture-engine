use std::ops::{Deref, DerefMut};

use crate::ECS;
use ECS::world::World;
use ECS::resource::Resource;
use ECS::events::Event;
use super::{FetchRes, FetchResMut};
use super::{EventReader, EventWriter};
use super::{CommandWriter, TriggerWriter};

/// # Request fetch trait
/// Required for `Request` to know what system resources to fetch from the World
/// 
/// It is implemented by default on `&` and `&mut` Resource references, 
/// Event Readers and Writers, the Command and Trigger Writers, as well as Tuples up to 4 elements
/// 
/// The return type `Item` is typically the type the trait gets implemented on
pub trait RequestData{
    type Item<'b>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a>;
}

/// # System resource Request
/// Struct that requests desired system resources from the World  
/// Such as Resources, Event Readers/Writers and Trigger and Command Writers
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

///////////////////////////////////////////////////////////////////////////////
// Resources
///////////////////////////////////////////////////////////////////////////////

impl<R: Resource> RequestData for &R{
    type Item<'b> = FetchRes<'b, R>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_res()
    }
}
impl<R: Resource> RequestData for &mut R{
    type Item<'b> = FetchResMut<'b, R>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_res_mut()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Events
///////////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////////
// Writers
//////////////////////////////////////////////////////////////////////////////////////////

impl RequestData for CommandWriter<'_>{
    type Item<'b> = CommandWriter<'b>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.get_command_writer()
    }
}
impl RequestData for TriggerWriter<'_>{
    type Item<'b> = TriggerWriter<'b>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.get_trigger_writer()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tuples
///////////////////////////////////////////////////////////////////////////////

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
impl<A, B, C, D, E> RequestData for (A, B, C, D, E)
where 
    A: RequestData,
    B: RequestData,
    C: RequestData,
    D: RequestData,
    E: RequestData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>, D::Item<'b>, E::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World), D::fetch(World), E::fetch(World))
    }
}
impl<A, B, C, D, E, F> RequestData for (A, B, C, D, E, F)
where 
    A: RequestData,
    B: RequestData,
    C: RequestData,
    D: RequestData,
    E: RequestData,
    F: RequestData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>, D::Item<'b>, E::Item<'b>, F::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World), D::fetch(World), E::fetch(World), F::fetch(World))
    }
}
impl<A, B, C, D, E, F, G> RequestData for (A, B, C, D, E, F, G)
where 
    A: RequestData,
    B: RequestData,
    C: RequestData,
    D: RequestData,
    E: RequestData,
    F: RequestData,
    G: RequestData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>, D::Item<'b>, E::Item<'b>, F::Item<'b>, G::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World), D::fetch(World), E::fetch(World), F::fetch(World), G::fetch(World))
    }
}
impl<A, B, C, D, E, F, G, H> RequestData for (A, B, C, D, E, F, G, H)
where 
    A: RequestData,
    B: RequestData,
    C: RequestData,
    D: RequestData,
    E: RequestData,
    F: RequestData,
    G: RequestData,
    H: RequestData
{
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>, D::Item<'b>, E::Item<'b>, F::Item<'b>, G::Item<'b>, H::Item<'b>);

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World), D::fetch(World), E::fetch(World), F::fetch(World), G::fetch(World), H::fetch(World))
    }
}
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

macro_rules! request_impl {
    ($($x:ident), *) => {
        #[allow(non_snake_case)]
        impl<$($x: RequestData), *> RequestData for ($($x), *){
            type Item<'b> = ($($x::Item<'b>), *);

            fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
                ($($x::fetch(World)), *)
            }
        }
    }
}

request_impl!(A, B);
request_impl!(A, B, C);
request_impl!(A, B, C, D);
request_impl!(A, B, C, D, E);
request_impl!(A, B, C, D, E, F);
request_impl!(A, B, C, D, E, F, G);
request_impl!(A, B, C, D, E, F, G, H);
request_impl!(A, B, C, D, E, F, G, H, I);
request_impl!(A, B, C, D, E, F, G, H, I, J);
request_impl!(A, B, C, D, E, F, G, H, I, J, K);
request_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
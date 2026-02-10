use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::ECS;
use crate::ECS::fetch::{WorldQuery, QueryData, QueryFilter};
use ECS::world::World;
use ECS::resource::Resource;
use ECS::events::Event;
use super::{FetchRes, FetchResMut};
use super::{EventReader, EventWriter};
use super::{CommandWriter, TriggerWriter};

/// # Request fetch trait
/// Required for `Request` to know what System resources to fetch from the World
/// 
/// It is implemented by default on `&` and `&mut` Resource references, 
/// Event Readers and Writers, the Command and Trigger Writers, as well as Tuples up to 4 elements
/// 
/// The return type `Item` is typically the type the trait gets implemented on
pub trait RequestData{
    type Item<'b>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a>;
}

/// # System resource Request
/// Struct that requests desired System resources from the World  
/// Such as Resources, Event Readers/Writers and Trigger and Command Writers
pub struct Request<'a, D: RequestData>{
    data: D::Item<'a>
}
impl<'a, D: RequestData> Request<'a, D>{
    pub fn fetch(world: &'a World) -> Self{
        Self{
            data: D::fetch(world),
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

/// # Query Request
/// An identifier for `WorldQuery` to make data acquisition easier
/// 
/// **Below documentation for `WorldQuery`**
/// 
/// Struct that queries the World and fetches the specified `QueryData`, usually Components
/// 
/// You can specify filters for the Query to use when getting Entities, such as `With` and `Without`.  
/// Any type implementing `QueryFilter` can be used
/// 
/// To get a specific Entity's set of Components, use `get`, `get_mut`, and their Token variations.  
/// Token variations of getters are preferred over normal getters
/// 
/// To iterate over all entities with all queried Components, use `iter` and `iter_mut`
/// 
/// To access the underlying Storages directly, use a dereference `*`.  
/// Note that Filters will not apply if you do this
/// 
/// Query automatically validates Tokens in Getter functions, they can also be  
/// manually validated via `validate_token`
pub struct Query<D: QueryData, F: QueryFilter>(PhantomData<(D, F)>);
impl <D: QueryData, F: QueryFilter> RequestData for Query<D, F>{
    type Item<'b> = WorldQuery<'b, D, F>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        WorldQuery::fetch(world)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Resources
///////////////////////////////////////////////////////////////////////////////

impl<R: Resource> RequestData for &R{
    type Item<'b> = FetchRes<'b, R>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch_res()
    }
}
impl<R: Resource> RequestData for &mut R{
    type Item<'b> = FetchResMut<'b, R>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch_res_mut()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Events
///////////////////////////////////////////////////////////////////////////////

/// # Event Reader Request
/// An identifier for `EventReader` to make Event queue acquisition for reading events easier
pub struct ReadEvent<E: Event>(PhantomData<E>);

impl<E: Event> RequestData for ReadEvent<E>{
    type Item<'b> = EventReader<'b, E>;
    
    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.get_event_reader()
    }
}

/// # Event Writer Request
/// An identifier for `EventWriter` to make Event queue acquisition for reading and sending events easier
pub struct WriteEvent<E: Event>(PhantomData<E>);
impl<E: Event> RequestData for WriteEvent<E>{
    type Item<'b> = EventWriter<'b, E>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.get_event_writer()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Writers
///////////////////////////////////////////////////////////////////////////////

/// # Commands Request
/// An identifier for `CommandWriter` to make command queue acquisition easier
pub struct Commands;
/// # Triggers Request
/// An identifier for `TriggerWriter` to make trigger queue acquisition easier
pub struct Triggers;

impl RequestData for Commands{
    type Item<'b> = CommandWriter<'b>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.get_command_writer()
    }
}
impl RequestData for Triggers{
    type Item<'b> = TriggerWriter<'b>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.get_trigger_writer()
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

#[cfg(test)]
mod tests{
    use super::*;
    mod test_resource{
        use super::*;

        struct idkfa(u8);
        struct iddqd(u8);
        impl Resource for idkfa{
            const ID: &'static str = "idkfa";
        
            fn new() -> Self {
                Self(5)
            }
        }
        impl Resource for iddqd{
            const ID: &'static str = "iddqd";
        
            fn new() -> Self {
                Self(5)
            }
        }

        #[test]
        fn test(){
            let mut world = World::new();
            world.register_res::<idkfa>();
            world.register_res::<iddqd>();

            let mut request: Request<'_, (&idkfa, &mut iddqd)> = Request::fetch(&world);
            request.1.0 = 10;

            assert!(request.0.0 == 5);
            assert!(request.1.0 == 10);
        }
    }
    mod test_query{
        use super::*;
        use crate::ECS::comp::Component;
        use crate::ECS::storage::test::HashMapStorage;

        struct idkfa(u8);
        struct iddqd(u8);
        impl Component for idkfa{
            type STORAGE = HashMapStorage<Self>;
        
            const ID: &'static str = "idkfa";
        }
        impl Component for iddqd{
            type STORAGE = HashMapStorage<Self>;
        
            const ID: &'static str = "iddqd";
        }

        #[test]
        fn test(){
            let mut world = World::new();
            world.register_comp::<idkfa>();
            world.register_comp::<iddqd>();

            world.spawn().with(idkfa(5)).with(iddqd(10)).finish();

            let mut request: Request<'_, Query<(&idkfa, &mut iddqd), ()>> = Request::fetch(&world);

            request.get_mut(&0).unwrap().1.0 += 5;

            assert!(matches!(request.get_mut(&0).unwrap(), (&idkfa(5), &mut iddqd(15))))
            
        }
        #[test]
        fn test_overlap_safe(){
            let mut world = World::new();
            world.register_comp::<idkfa>();
            world.register_comp::<iddqd>();

            world.spawn().with(idkfa(5)).with(iddqd(10)).finish();

            let mut request_a: Request<'_, Query<(&idkfa, &mut iddqd), ()>> = Request::fetch(&world);
            let request_b: Request<'_, Query<&idkfa, ()>> = Request::fetch(&world);

            let rq_a_ent = request_a.get_mut(&0).unwrap();
            let rq_b_ent = request_b.get(&0).unwrap();

            rq_a_ent.1.0 += 5;

            assert!(rq_a_ent.0.0 == rq_b_ent.0);
            assert!(rq_a_ent.1.0 == 15);

        }
        #[test]
        #[should_panic]
        fn test_overlap_panic(){
            let mut world = World::new();
            world.register_comp::<idkfa>();
            world.register_comp::<iddqd>();

            let _request_a: Request<'_, Query<(&idkfa, &mut iddqd), ()>> = Request::fetch(&world);
            let _request_b: Request<'_, Query<&mut idkfa, ()>> = Request::fetch(&world);

        }
    }
    mod test_event{
        use super::*;

        struct idkfa;
        struct iddqd;
        impl Event for idkfa{
            const ID: &'static str = "idkfa";
        }
        impl Event for iddqd{
            const ID: &'static str = "iddqd";
        }

        #[test]
        fn test(){
            let mut world = World::new();
            world.register_event::<idkfa>();
            world.register_event::<iddqd>();

            let mut request: Request<'_, (ReadEvent<idkfa>, WriteEvent<iddqd>)> = Request::fetch(&world);

            request.1.send(iddqd);

            assert!(request.0.event_count() == 0);
            assert!(request.1.current_event_count() == 1);
        }
        #[test]
        fn test_overlap_safe(){
            let mut world = World::new();
            world.register_event::<idkfa>();
            world.register_event::<iddqd>();

            let mut request_a: Request<'_, (ReadEvent<idkfa>, WriteEvent<iddqd>)> = Request::fetch(&world);
            let request_b: Request<'_, ReadEvent<idkfa>> = Request::fetch(&world);

            request_a.1.send(iddqd);

            assert!(request_a.0.event_count() == 0);
            assert!(request_a.1.current_event_count() == 1);
            assert!(request_b.event_count() == 0);
        }
        #[test]
        #[should_panic]
        fn test_overlap_panic(){
            let mut world = World::new();
            world.register_event::<idkfa>();
            world.register_event::<iddqd>();

            let _request_a: Request<'_, (ReadEvent<idkfa>, WriteEvent<iddqd>)> = Request::fetch(&world);
            let _request_b: Request<'_, (ReadEvent<idkfa>, WriteEvent<iddqd>)> = Request::fetch(&world);
        }
    }
    mod test_commands{
        use super::*;
        use crate::ECS::commands::Command;

        struct idkfa;
        struct iddqd;
        impl Command for idkfa{
            fn execute(&mut self, _world: &mut World) {}
        }
        impl Command for iddqd{
            fn execute(&mut self, _world: &mut World) {}
        }

        #[test]
        fn test(){
            let world = World::new();

            let mut request: Request<'_, Commands> = Request::fetch(&world);

            request.send(idkfa);
            request.send(iddqd);

            assert!(request.command_count() == 2)
        }
        #[test]
        #[should_panic]
        fn test_illegal_fetch(){
            let world = World::new();

            let _request_a: Request<'_, Commands> = Request::fetch(&world);
            let _request_b: Request<'_, Commands> = Request::fetch(&world);
        }
    }
    mod test_triggers{
        use super::*;

        #[test]
        fn test(){
            let world = World::new();

            let mut request: Request<'_, Triggers> = Request::fetch(&world);

            request.send("idkfa");
            request.send("iddqd");

            assert!(request.trigger_count() == 2)
        }
        #[test]
        #[should_panic]
        fn test_illegal_fetch(){
            let world = World::new();

            let _request_a: Request<'_, Triggers> = Request::fetch(&world);
            let _request_b: Request<'_, Triggers> = Request::fetch(&world);
        }
    }
}
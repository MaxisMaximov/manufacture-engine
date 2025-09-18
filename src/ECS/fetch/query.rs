use std::{collections::BTreeMap, ops::{Deref, DerefMut}};

use crate::ECS;
use ECS::entity::Entity;
use ECS::world::World;
use ECS::comp::Component;
use super::{Fetch, FetchMut};

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
/// Struct that queries the World and fetches the specified `QueryData`, usually Components
pub struct Query<'a, D: QueryData>{
    entities: &'a BTreeMap<usize, Entity>,
    data: D::Item<'a>
}
impl<'a, D: QueryData> Query<'a, D>{
    pub fn fetch(World: &'a World) -> Self{
        Self{
            entities: World.get_entities(),
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

///////////////////////////////////////////////////////////////////////////////
// Components
///////////////////////////////////////////////////////////////////////////////

impl<C:Component> QueryData for &C{
    type Item<'b> = Fetch<'b, C>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch::<C>()
    }
}
impl<C: Component> QueryData for &mut C{
    type Item<'b> = FetchMut<'b, C>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_mut::<C>()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tuples
///////////////////////////////////////////////////////////////////////////////

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
use std::marker::PhantomData;

use super::*;
use crate::ECS::fetch::Fetch;
use crate::ECS::entity::EntityBuilder;

mod vector;
pub use vector::*;

/// # Query Filter: With
/// Only allows Entities that have the specified Component to pass through
/// 
/// There is no need to include it in the filters if you fetch the Component,  
/// Query automatically checks whether the requested Components exist for an Entity
pub struct With<C: Component>(PhantomData<C>);
impl<C: Component> QueryFilter for With<C>{
    type Item<'b> = Fetch<'b, C>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch::<C>()
    }

    fn filter<'a>(fetched: &'a Self::Item<'a>, index: &usize) -> bool {
        fetched.get(index).is_some()
    }
}

/// # Query Filter: Without
/// Only allows Entities without the specified Component to pass through
pub struct Without<C: Component>(PhantomData<C>);
impl<C: Component> QueryFilter for Without<C>{
    type Item<'b> = Fetch<'b, C>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch::<C>()
    }

    fn filter<'a>(fetched: &'a Self::Item<'a>, index: &usize) -> bool {
        fetched.get(index).is_none()
    }
}

/// # Entity Prefab trait
/// A tiny rudimentary trait to make spawning Entities with Components easier
/// 
/// `spawn` method takes `&self` so that you can send custom data for Prefab to have right away
pub trait EntityPrefab{
    const ID: &'static str = "idkfa";
    fn spawn(&self, builder: EntityBuilder<'_>);
}
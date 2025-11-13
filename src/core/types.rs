use std::marker::PhantomData;

use super::*;
use crate::ECS::fetch::Fetch;

/// # Query Filter: With
/// Only allows Entities that have the specified component to pass through
/// 
/// There is no need to include it in the filters if you fetch the component,  
/// Query automatically checks whether the requested components exist for an Entity
pub struct With<C: Component>(PhantomData<C>);
impl<C: Component> QueryFilter for With<C>{
    type Item<'b> = Fetch<'b, C>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch::<C>()
    }

    fn filter<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> bool {
        Fetched.get(Index).is_some()
    }
}

/// # Query Filter: Without
/// Only allows Entities without the specified component to pass through
pub struct Without<C: Component>(PhantomData<C>);
impl<C: Component> QueryFilter for Without<C>{
    type Item<'b> = Fetch<'b, C>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch::<C>()
    }

    fn filter<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> bool {
        Fetched.get(Index).is_none()
    }
}
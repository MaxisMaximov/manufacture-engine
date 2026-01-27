use std::{collections::BTreeMap, ops::{Deref, DerefMut}};

use crate::ECS;
use ECS::entity;
use ECS::storage::Storage;
use ECS::entity::Entity;
use ECS::world::World;
use ECS::comp::Component;
use super::{Fetch, FetchMut};

/// # Query fetch trait
/// Required for `Query` to know what to fetch from the World
/// 
/// It is implemented by default on `&` and `&mut` Component references, 
/// `Option<&Component>` and `Option<&mut Component>`, 
/// as well as Tuples up to 12 elements
/// 
/// The return type `Item` is typically the type the trait gets implemented on
/// 
/// `AccItem` is what gets returned when getting data immutably
/// 
/// `MutAccItem` is what gets returned when getting data mutably.  
/// Read-only Components simply return their `AccItem` when getting mutably, such as `&Component`
pub trait QueryData{
    type Item<'b>;
    type AccItem<'b>;
    type MutAccItem<'b>;

    /// Fetch the data from the World
    fn fetch<'a>(world: &'a World) -> Self::Item<'a>;
    /// Access given Entity's data immutably
    fn get<'a>(fetched: &'a Self::Item<'a>, id: &usize) -> Option<Self::AccItem<'a>>;
    /// Access given Entity's data mutably
    fn get_mut<'a>(fetched: &'a mut Self::Item<'a>, id: &usize) -> Option<Self::MutAccItem<'a>>;
}

/// # Query Filter trait
/// Required for Query to filter out entities with matching Components.  
/// 
/// It is essentially a miniature System that checks whether an Entity meets  
/// the requirements for the main System to proceed
/// 
/// It is implemented by default on `With` and `Without` structs, as well as tuples up to 12 elements
/// 
/// The data Filter fetches must be **read-only**,  
/// they're not fetching data to be modified,  
/// only to read if the requirements are met
/// 
/// `Item` is the data the filter needs to filter out the Entity
pub trait QueryFilter{
    type Item<'b>;
    /// Fetch the needed data from the World
    fn fetch<'a>(world: &'a World) -> Self::Item<'a>;
    /// Check if the given entity passes this filter
    fn filter<'a>(fetched: &'a Self::Item<'a>, id: &usize) -> bool;
}

/// # World Query
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
pub struct WorldQuery<'a, D: QueryData, F: QueryFilter>{
    entities: &'a BTreeMap<usize, Entity>,
    filter_data: F::Item<'a>,
    data: D::Item<'a>
}
impl<'a, D: QueryData, F: QueryFilter> WorldQuery<'a, D, F>{
    /// Fetch `D`ata from the World
    pub fn fetch(world: &'a World) -> Self{
        Self{
            entities: world.get_entities(),
            filter_data: F::fetch(world),
            data: D::fetch(world)
        }
    }

    /// Get a set of Components for a given entity
    /// 
    /// It is generally discouraged to get Components this way if you're looking for a specific Entity  
    /// If you can, use `get_from_token` instead
    /// 
    /// Note that it returns `Some` only if the entity has *all* requested Components,  
    /// otherwise it returns `None`
    pub fn get(&'a self, id: &usize) -> Option<D::AccItem<'a>>{
        if self.entities.contains_key(id) && F::filter(&self.filter_data, id){
            D::get(&self.data, id)
        }else{
            None
        }
    }
    /// Get a set of Components for the Entity tracked by the Token.  
    /// It automatically validates the given Token as well
    /// 
    /// Note that it returns `Some` only if the entity has *all* requested Components,  
    /// otherwise it returns `None`
    pub fn get_from_token(&'a self, token: &mut entity::Token) -> Option<D::AccItem<'a>>{
        // We only accept valid Tokens
        if self.validate_token(token){
            self.get(&token.id())
        }else{
            None
        }

    }

    /// Get a mutable set of Components for a given entity
    /// 
    /// It is generally discouraged to get Components this way if you're looking for a specific Entity.  
    /// If you can, use `get_from_token_mut` instead
    /// 
    /// Note that it returns `Some` only if the entity has *all* requested Components,  
    /// otherwise it returns `None`
    pub fn get_mut(&'a mut self, id: &usize) -> Option<D::MutAccItem<'a>>{
        if !self.entities.contains_key(id){
            return None
        }
        if !F::filter(&self.filter_data, id){
            return None
        }
        D::get_mut(&mut self.data, id)
    }
    /// Get a mutable set of Components for the Entity tracked by the Token.  
    /// It automatically validates the given Token as well
    /// 
    /// Note that it returns `Some` only if the entity has *all* requested Components,  
    /// otherwise it returns `None`
    pub fn get_from_token_mut(&'a mut self, token: &mut entity::Token) -> Option<D::MutAccItem<'a>>{
        // We only accept valid Tokens
        if self.validate_token(token){
            self.get_mut(&token.id())
        }else{
            None
        }
    }

    /// Iterate over all matching entities immutably  
    /// 
    /// Entities that don't have at least one matching Component will not be iterated over
    pub fn iter(&'a self) -> Iter<'a, D, F>{
        Iter{
            data: &self.data,
            filters: &self.filter_data,
            ent_iter: self.entities.keys(),
        }
    }
    /// Iterate over all matching entities mutably  
    /// 
    /// Entities that don't have at least one matching Component will not be iterated over
    pub fn iter_mut(&'a mut self) -> IterMut<'a, D, F>{
        IterMut{
            data: &mut self.data,
            filters: &self.filter_data,
            ent_iter: self.entities.keys(),
        }
    }

    /// Validate an Entity Token  
    /// 
    /// Updates Token's `valid` flag and returns boolean whether it's still valid or not
    pub fn validate_token(&self, token: &mut entity::Token) -> bool{
        // If a Token is invalid, it can never again be valid
        token.valid() 
            && 
        // A check if the Entity exists and if the Token is still valid in one
        // Bit of a mess, I know
        self.entities.get(&token.id()).is_some_and(|entity| token.validate(entity))
    }
}
impl<'a, D:QueryData, F: QueryFilter> Deref for WorldQuery<'a, D, F>{
    type Target = D::Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, D: QueryData, F: QueryFilter> DerefMut for WorldQuery<'a, D, F>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

///////////////////////////////////////////////////////////////////////////////
// Iterators
///////////////////////////////////////////////////////////////////////////////

use std::collections::btree_map::Keys;
/// # Query Iterator
/// Iterates over entities that have all matching Components of `D`ata immutably
pub struct Iter<'a, D: QueryData, F: QueryFilter>{
    data: &'a D::Item<'a>,
    filters: &'a F::Item<'a>,
    ent_iter: Keys<'a, usize, Entity>
}
impl<'a, D: QueryData, F: QueryFilter> Iterator for Iter<'a, D, F>{
    type Item = D::AccItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop{
            let index = self.ent_iter.next()?;

            
            if let Some(fetched) = D::get(self.data, index){
                if F::filter(self.filters, index){
                    return Some(fetched)
                }
            }
        }
    }
}

/// # Mutable Query Iterator
/// Iterates over entities that have all matching Components of `D`ata mutably
pub struct IterMut<'a, D: QueryData, F: QueryFilter>{
    data: &'a mut D::Item<'a>,
    filters: &'a F::Item<'a>,
    ent_iter: Keys<'a, usize, Entity>
}
impl<'a, D: QueryData, F: QueryFilter> Iterator for IterMut<'a, D, F>{
    type Item = D::MutAccItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop{
            let index = self.ent_iter.next()?;

            if let Some(fetched) = 
                D::get_mut(
                    // SAFETY: I have no goddamn pecking idea
                    // But this is what 
                    // [this](stackoverflow.com/questions/61978903/how-do-i-create-mutable-iterator-over-struct-fields)
                    // post's last comment suggests for a whole different problem

                    // I PRESUME:
                    // 1. We -grade (up or down??) `self.data` - which is a mutable 
                    //    reference to Query's `data` field - into a mutable *pointer*
                    // 2. We dereference that pointer to get to the original
                    //    data, getting *it's* lifetime now instead of Query's
                    // 3. We then pass that direct original data as a mutable reference into the Getter

                    // I have no idea how it actually works, but the comment probably explains it better
                    // I might redo this later, or this hotwire will still be here
                    // Mark my words this will be unchanged since 3.10.2025
                    // (10.3.2025 for you American Burger Per Freedom Mile Eagles people)

                    // Unless I redo the engine 4th time in a row
                    unsafe{&mut *(self.data as *mut D::Item<'a>)}, 
                    index
                )
            {
                if F::filter(self.filters, index){
                    return Some(fetched)
                }
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Components
///////////////////////////////////////////////////////////////////////////////

impl<C:Component> QueryData for &C{
    type Item<'b> = Fetch<'b, C>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch::<C>()
    }
    
    type AccItem<'b> = &'b C;
    type MutAccItem<'b> = &'b C;
    
    fn get<'a>(fetched: &'a Self::Item<'a>, id: &usize) -> Option<Self::AccItem<'a>> {
        fetched.get(id)
    }
    fn get_mut<'a>(fetched: &'a mut Self::Item<'a>, id: &usize) -> Option<Self::MutAccItem<'a>> {
        fetched.get(id)
    }    
}
impl<C: Component> QueryData for &mut C{
    type Item<'b> = FetchMut<'b, C>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch_mut::<C>()
    }
    
    type AccItem<'b> = &'b C;
    type MutAccItem<'b> = &'b mut C;
    
    fn get<'a>(fetched: &'a Self::Item<'a>, id: &usize) -> Option<Self::AccItem<'a>> {
        fetched.get(id)
    }
    fn get_mut<'a>(fetched: &'a mut Self::Item<'a>, id: &usize) -> Option<Self::MutAccItem<'a>> {
        fetched.get_mut(id)
    }
}

impl<C: Component> QueryData for Option<&C>{
    type Item<'b> = Fetch<'b, C>;
    type AccItem<'b> = Option<&'b C>;
    type MutAccItem<'b> = Option<&'b C>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch::<C>()
    }
    
    fn get<'a>(fetched: &'a Self::Item<'a>, id: &usize) -> Option<Self::AccItem<'a>> {
        // Why is it wrapped in `Some`:
        // `Option` signifies an optional Component, so the return type is `Option`
        // The Systems handle the `Option`s themselves
        //
        // But we can't just use `get` like with normal Component references
        // `Some` essentially ensures the Getter functions always return  
        // a valid AccItem, and therefore a valid Component set
        Some(fetched.get(id))
    }
    fn get_mut<'a>(fetched: &'a mut Self::Item<'a>, id: &usize) -> Option<Self::MutAccItem<'a>> {
        Some(fetched.get(id))
    }
}
impl<C: Component> QueryData for Option<&mut C>{
    type Item<'b> = FetchMut<'b, C>;
    type AccItem<'b> = Option<&'b C>;
    type MutAccItem<'b> = Option<&'b mut C>;

    fn fetch<'a>(world: &'a World) -> Self::Item<'a> {
        world.fetch_mut::<C>()
    }
    
    fn get<'a>(fetched: &'a Self::Item<'a>, id: &usize) -> Option<Self::AccItem<'a>> {
        Some(fetched.get(id))
    }
    fn get_mut<'a>(fetched: &'a mut Self::Item<'a>, id: &usize) -> Option<Self::MutAccItem<'a>> {
        Some(fetched.get_mut(id))
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tuples
///////////////////////////////////////////////////////////////////////////////

impl QueryData for (){
    type Item<'b> = ();
    type AccItem<'b> = ();
    type MutAccItem<'b> = ();

    fn fetch<'a>(_world: &'a World) -> Self::Item<'a> {
        ()
    }
    fn get<'a>(_fetched: &'a Self::Item<'a>, _id: &usize) -> Option<Self::AccItem<'a>> {
        Some(())
    }
    fn get_mut<'a>(_fetched: &'a mut Self::Item<'a>, _id: &usize) -> Option<Self::MutAccItem<'a>> {
        Some(())
    }
}

impl QueryFilter for (){
    type Item<'b> = ();

    fn fetch<'a>(_world: &'a World) -> Self::Item<'a> {
        ()
    }
    fn filter<'a>(_fetched: &'a Self::Item<'a>, _index: &usize) -> bool {
        true
    }
}

macro_rules! query_impl {
    ($($x:tt), *) => {
        #[allow(non_snake_case)]
        impl<$($x: QueryData), *> QueryData for ($($x), *){
            type Item<'b> = ($($x::Item<'b>), *);

            fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
                ($($x::fetch(World)), *)
            }

            type AccItem<'b> = ($($x::AccItem<'b>), *);
            type MutAccItem<'b> = ($($x::MutAccItem<'b>), *);
            
            fn get<'a>(($($x), *): &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>> {
                Some(
                    ($($x::get($x, Index)?), *)
                )
            }
            fn get_mut<'a>(($($x), *): &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>> {
                Some(
                    ($($x::get_mut($x, Index)?), *)
                )
            }
        }
    }
}

macro_rules! filter_impl {
    ($($x:tt), *) => {
        #[allow(non_snake_case)]
        impl<$($x: QueryFilter), *> QueryFilter for ($($x), *){
            type Item<'b> = ($($x::Item<'b>), *);

            fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
                ($($x::fetch(World)), *)
            }

            fn filter<'a>(($($x), *): &'a Self::Item<'a>, Index: &usize) -> bool {
                $($x::filter($x, Index)) && *
            }
        }
    }
}


query_impl!(A, B);
query_impl!(A, B, C);
query_impl!(A, B, C, D);
query_impl!(A, B, C, D, E);
query_impl!(A, B, C, D, E, F);
query_impl!(A, B, C, D, E, F, G);
query_impl!(A, B, C, D, E, F, G, H);
query_impl!(A, B, C, D, E, F, G, H, I);
query_impl!(A, B, C, D, E, F, G, H, I, J);
query_impl!(A, B, C, D, E, F, G, H, I, J, K);
query_impl!(A, B, C, D, E, F, G, H, I, J, K, L);

filter_impl!(A, B);
filter_impl!(A, B, C);
filter_impl!(A, B, C, D);
filter_impl!(A, B, C, D, E);
filter_impl!(A, B, C, D, E, F);
filter_impl!(A, B, C, D, E, F, G);
filter_impl!(A, B, C, D, E, F, G, H);
filter_impl!(A, B, C, D, E, F, G, H, I);
filter_impl!(A, B, C, D, E, F, G, H, I, J);
filter_impl!(A, B, C, D, E, F, G, H, I, J, K);
filter_impl!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
mod tests{
    use super::*;
    mod test_fetch{
        use super::*;
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

            let _query: WorldQuery<'_, (&idkfa, &mut iddqd), ()> = WorldQuery::fetch(&world);
        }
        #[test]
        fn test_get(){
            let mut world = World::new();
            world.register_comp::<idkfa>();
            world.register_comp::<iddqd>();

            world.spawn().with(idkfa(5)).with(iddqd(10)).finish();

            let mut query: WorldQuery<'_, (&idkfa, &mut iddqd), ()> = WorldQuery::fetch(&world);

            assert!(query.0.get(&0).is_some());
            assert!(query.1.get_mut(&0).is_some());
        }
        #[test]
        fn test_iter(){
            let mut world = World::new();
            world.register_comp::<idkfa>();
            world.register_comp::<iddqd>();

            world.spawn().with(idkfa(5)).with(iddqd(10)).finish();

            let mut query: WorldQuery<'_, (&idkfa, &mut iddqd), ()> = WorldQuery::fetch(&world);

            for comps in query.iter(){

            }
        }
    }
    mod test_filter{}
}
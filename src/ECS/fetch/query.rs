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
/// Read-only components simply return their `AccItem` when getting mutably, such as `&Component`
pub trait QueryData{
    type Item<'b>;
    type AccItem<'b>;
    type MutAccItem<'b>;

    /// Fetch the data from the world
    fn fetch<'a>(World: &'a World) -> Self::Item<'a>;
    /// Access given Entity's data immutably
    fn get<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>>;
    /// Access given Entity's data mutably
    fn get_mut<'a>(Fetched: &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>>;
}

pub trait QueryFilter{
    type Item<'b>;
    /// Fetch the needed data from the World
    fn fetch<'a>(World: &'a World) -> Self::Item<'a>;
    /// Check if the given entity passes this filter
    fn filter<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> bool;
}
/// # World Query
/// Struct that queries the World and fetches the specified `QueryData`, usually Components
/// 
/// To access the underlying Storages directly, use a dereference `*`
/// 
/// To get a specific Entity's set of components, use `get`, `get_mut`, and their Token variations.  
/// Token variations of getters are preferred over normal getters
/// 
/// To iterate over all entities with all queried components, use `iter` and `iter_mut`
/// 
/// Query automatically validates Tokens in Getter functions, they can also be  
/// manually validated via `validate_token`
pub struct Query<'a, D: QueryData>{
    entities: &'a BTreeMap<usize, Entity>,
    data: D::Item<'a>
}
impl<'a, D: QueryData> Query<'a, D>{
    /// Fetch `D`ata from the World
    pub fn fetch(World: &'a World) -> Self{
        Self{
            entities: World.get_entities(),
            data: D::fetch(World)
        }
    }

    /// Get a set of components for a given entity
    /// 
    /// It is generally discouraged to get components this way if you're looking for a specific Entity  
    /// If you can, use `get_from_token` instead
    /// 
    /// Note that it returns `Some` only if the entity has *all* requested components,  
    /// otherwise it returns `None`
    pub fn get(&'a self, Index: &usize) -> Option<D::AccItem<'a>>{
        if !self.entities.contains_key(Index){
            return None
        }

        D::get(&self.data, Index)
    }
    /// Get a set of components for the Entity tracked by the Token.  
    /// It automatically validates the given Token as well
    /// 
    /// Note that it returns `Some` only if the entity has *all* requested components,  
    /// otherwise it returns `None`
    pub fn get_from_token(&'a self, Token: &mut entity::Token) -> Option<D::AccItem<'a>>{
        // We only accept valid Tokens
        if self.validate_token(Token){
            D::get(&self.data, &Token.id())
        }else{
            None
        }

    }

    /// Get a mutable set of components for a given entity
    /// 
    /// It is generally discouraged to get components this way if you're looking for a specific Entity.  
    /// If you can, use `get_from_token_mut` instead
    /// 
    /// Note that it returns `Some` only if the entity has *all* requested components,  
    /// otherwise it returns `None`
    pub fn get_mut(&'a mut self, Index: &usize) -> Option<D::MutAccItem<'a>>{
        if !self.entities.contains_key(Index){
            return None
        }
        
        D::get_mut(&mut self.data, Index)
    }
    /// Get a mutable set of components for the Entity tracked by the Token.  
    /// It automatically validates the given Token as well
    /// 
    /// Note that it returns `Some` only if the entity has *all* requested components,  
    /// otherwise it returns `None`
    pub fn get_from_token_mut(&'a mut self, Token: &mut entity::Token) -> Option<D::MutAccItem<'a>>{
        // We only accept valid Tokens
        if self.validate_token(Token){
            D::get_mut(&mut self.data, &Token.id())
        }else{
            None
        }
    }

    /// Iterate over all matching entities immutably  
    /// 
    /// Entities that don't have at least one matching component will not be iterated over
    pub fn iter(&'a self) -> Iter<'a, D>{
        Iter{
            data: &self.data,
            ent_iter: self.entities.keys(),
        }
    }
    /// Iterate over all matching entities mutably  
    /// 
    /// Entities that don't have at least one matching component will not be iterated over
    pub fn iter_mut(&'a mut self) -> IterMut<'a, D>{
        IterMut{
            data: &mut self.data,
            ent_iter: self.entities.keys(),
        }
    }

    /// Validate an Entity Token  
    /// 
    /// Updates Token's `valid` flag and returns boolean whether it's still valid or not
    pub fn validate_token(&self, Token: &mut entity::Token) -> bool{
        // If a Token is invalid, it can never again be valid
        if !Token.valid(){
            false
        }else{
            if let Some(entity) = self.entities.get(&Token.id()){
                Token.validate(entity)
            }else{
                false
            }
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
// Iterators
///////////////////////////////////////////////////////////////////////////////

use std::collections::btree_map::Keys;
/// # Query Iterator
/// Iterates over entities that have all matching components of `D`ata immutably
pub struct Iter<'a, D: QueryData>{
    data: &'a D::Item<'a>,
    ent_iter: Keys<'a, usize, Entity>
}
impl<'a, D: QueryData> Iterator for Iter<'a, D>{
    type Item = D::AccItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop{
            let index = self.ent_iter.next()?;

            if let Some(fetched) = D::get(self.data, index){
                return Some(fetched)
            }
        }
    }
}

/// # Mutable Query Iterator
/// Iterates over entities that have all matching components of `D`ata mutably
pub struct IterMut<'a, D: QueryData>{
    data: &'a mut D::Item<'a>,
    ent_iter: Keys<'a, usize, Entity>
}
impl<'a, D: QueryData> Iterator for IterMut<'a, D>{
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
                return Some(fetched)
            }
        }
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
    
    type AccItem<'b> = &'b C;
    type MutAccItem<'b> = &'b C;
    
    fn get<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>> {
        Fetched.get(Index)
    }
    fn get_mut<'a>(Fetched: &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>> {
        Fetched.get(Index)
    }    
}
impl<C: Component> QueryData for &mut C{
    type Item<'b> = FetchMut<'b, C>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_mut::<C>()
    }
    
    type AccItem<'b> = &'b C;
    type MutAccItem<'b> = &'b mut C;
    
    fn get<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>> {
        Fetched.get(Index)
    }
    fn get_mut<'a>(Fetched: &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>> {
        Fetched.get_mut(Index)
    }
}

impl<C: Component> QueryData for Option<&C>{
    type Item<'b> = Fetch<'b, C>;
    type AccItem<'b> = Option<&'b C>;
    type MutAccItem<'b> = Option<&'b C>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch::<C>()
    }
    
    fn get<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>> {
        // Why is it wrapped in `Some`:
        // `Option` signifies an optional Component, so the return type is `Option`
        // The systems handle the `Option`s themselves
        //
        // But we can't just use `get` like with normal Component references
        // `Some` essentially ensures the Getter functions always return  
        // a valid AccItem, and therefore a valid Component set
        Some(Fetched.get(Index))
    }
    fn get_mut<'a>(Fetched: &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>> {
        Some(Fetched.get(Index))
    }
}
impl<C: Component> QueryData for Option<&mut C>{
    type Item<'b> = FetchMut<'b, C>;
    type AccItem<'b> = Option<&'b C>;
    type MutAccItem<'b> = Option<&'b mut C>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_mut::<C>()
    }
    
    fn get<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>> {
        Some(Fetched.get(Index))
    }
    fn get_mut<'a>(Fetched: &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>> {
        Some(Fetched.get_mut(Index))
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tuples
///////////////////////////////////////////////////////////////////////////////

macro_rules! query_impl {
    ($($x:ident), *) => {
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
#![allow(type_alias_bounds)]
use std::cell::{RefMut, Ref};

use super::comp::Component;
use super::events::Event;
use super::resource::Resource;
use super::commands::CommandWrapper;

pub mod query;
pub mod request;

pub type Fetch<'a, C: Component> = Ref<'a, C::STORAGE>;
pub type FetchMut<'a, C: Component> = RefMut<'a, C::STORAGE>;

pub type FetchRes<'a, R: Resource> = Ref<'a, R>;
pub type FetchResMut<'a, R: Resource> = RefMut<'a, R>;

pub struct EventReader<'a, E: Event>{
    pub(super) inner: Ref<'a, Vec<E>>
}
pub struct EventWriter<'a, E: Event>{
    pub(super) inner: RefMut<'a, Vec<E>>
}

///////////////////////////////////////////////////////////////////////////////
// Reexports
///////////////////////////////////////////////////////////////////////////////

pub use query::*;
pub use request::*;
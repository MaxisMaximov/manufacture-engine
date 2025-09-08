#![allow(type_alias_bounds)]
use std::cell::{RefMut, Ref};
use std::collections::VecDeque;

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

pub type EventReader<'a, E: Event> = Ref<'a, VecDeque<E>>;
pub type EventWriter<'a, E: Event> = RefMut<'a, VecDeque<E>>;

pub type CommandWriter<'a> = RefMut<'a, Vec<Box<dyn CommandWrapper>>>;
pub type TriggerWriter<'a> = RefMut<'a, Vec<&'static str>>;
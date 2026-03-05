#![allow(nonstandard_style)]
//! # Manufacture Engine
//! A bare-bones ECS-based engine responsible for powering my projects
//! 
//! Provides a fairly simple but highly controllable API for game dev, from basic building blocks like Components and Resources to fine grained control over how Systems are ran
//! 
//! It's recommended to also get `manufacture-core-library` to start your projects
pub mod ECS;
pub use ECS::prelude;
use std::collections::HashSet;

use super::*;
use crate::core::storage::*;

/// A simple 2D Coordinate type
pub struct Vector2{
    pub x: f32,
    pub y: f32,
}
impl Component for Vector2{
    type STORAGE = BTreeMapStorage<Self>;

    const ID: &'static str = "Vector2";
}

/// A simple 3D coordinate type
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Component for Vector3{
    type STORAGE = BTreeMapStorage<Self>;

    const ID: &'static str = "Vector3";
}

/// Holds tags for a given Entity
pub struct Tags{
    pub inner: HashSet<&'static str>
}
impl Component for Tags{
    type STORAGE = HashMapStorage<Self>;

    const ID: &'static str = "Tags";
}

/// A Command-Line-exclusive sprite
/// 
/// Represents a 2D ASCII art image
pub struct CMDSprite{
    pub size_x: u8,
    pub size_y: u8,
    pub data: Vec<(char, (u8, u8, u8), (u8, u8, u8))> // Symbol, Foreground RGB, Background RGB
}
impl Component for CMDSprite{
    type STORAGE = HashMapStorage<Self>;

    const ID: &'static str = "CMDSprite";
}

/// Identifies an Entity as being controlled by the player
/// 
/// Typically used to control player movement
pub struct PlayerController{
    pub pid: u16,
    pub active: bool,
}
impl Component for PlayerController{
    type STORAGE = VecStorage<Self>;

    const ID: &'static str = "PlayerController";
}
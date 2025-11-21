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

use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign
};
/// A simple 2D coordinate type
#[derive(Clone, Copy)]
pub struct Vector2{
    pub x: f32,
    pub y: f32
}
impl Vector2{
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn dot(&self, other: &Self) -> f32{
        (self.x * other.x) + (self.y * other.y)
    }
    pub fn project(&self, other: &Self) -> Self{
        let scalar = self.dot(other)/other.length().powi(2);
        Self{
            x: other.x * scalar,
            y: other.y * scalar,
        }
    }
    pub fn reflected(&self, other: &Self) -> Self{
        self.project(other) * 2.0 - *self
    }
    pub fn distance(&self, other: &Self) -> f32{
        (*self - *other).length()
    }
    pub fn reverse(&mut self){
        self.x = -self.x;
        self.y = -self.y;
    }
    pub fn reversed(&self) -> Self{
        Self{
            x: -self.x,
            y: -self.y,
        }
    }
    pub fn normalize(&mut self){
        let len = self.length();
        self.x /= len;
        self.y /= len;
    }
    pub fn normalized(&self) -> Self{
        let len = self.length();
        Self{
            x: self.x / len,
            y: self.y / len,
        }
    }
    pub fn angle_between(&self, other: &Self) -> f32{
        (self.dot(&other) / (self.length() * other.length())).acos()
    }
    pub fn length(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

}
impl std::fmt::Display for Vector2{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}
impl Add for Vector2{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Vector2{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Vector2{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Vector2{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y
    }
}
impl Mul<f32> for Vector2{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
impl MulAssign<f32> for Vector2{
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl Div<f32> for Vector2{
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

/// A simple 3D coordinate type
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
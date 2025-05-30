#![allow(unused_labels)]
#![warn(unused_crate_dependencies)]

pub mod ECS;
// pub mod manufacture;

// START HERE
fn main() {

    // Switch to Raw Mode
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut WORLD = ECS::world::gmWorld::new();
    let mut DISPATCHER = ECS::dispatcher::gmDispatcher::new();

    // manufacture::init::init(&mut WORLD, &mut DISPATCHER);

    loop {
        DISPATCHER.dispatch(&mut WORLD);
    }
    
    return;
}
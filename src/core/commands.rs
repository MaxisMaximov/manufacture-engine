use super::*;

use events::{EntitySpawned, EntityDespawned};
use types::EntityPrefab;

/// Send a Command to spawn a new Entity
/// 
/// Creates a new entity and sends an Event with the Entity's Token for Systems to use
/// 
/// Note: This creates a new Entity **without any Components**, you will have to add those later yourself. If you wish to create an Entity with predefined Components, use `SpawnPrefab` along with `Prefab` trait from `types`
pub struct Spawn;
impl Command for Spawn{
    fn execute(&mut self, world: &mut World) {
        let token = world.spawn().get_token();
        world.get_event_writer::<EntitySpawned>().send(EntitySpawned(token));
    }
}

/// Send a Command to spawn a new Entity with Components
/// 
/// Creates a new Entity using the Prefab's instructions and sends an Event with the Entity's Token
pub struct SpawnPrefab<T: EntityPrefab>(T);
impl<T: EntityPrefab + 'static> Command for SpawnPrefab<T>{
    fn execute(&mut self, world: &mut World) {
        let builder = world.spawn();
        let token = builder.get_token(); // Quickly yoink it because the Prefab will consume the builder

        T::spawn(&self.0, builder);
        world.get_event_writer::<EntitySpawned>().send(EntitySpawned(token));
    }
}

/// Send a Command to despawn an Entity via ID
/// 
/// It's generally discouraged to despawn Entities this way. If you can, use `DespawnToken` instead
pub struct DespawnID(pub usize);
impl Command for DespawnID{
    fn execute(&mut self, world: &mut World) {
        if world.despawn(self.0){
            world.get_event_writer::<EntityDespawned>().send(EntityDespawned(self.0));
        }
    }
}

/// Send a Command to despan Entity via Token
/// 
/// This is the preferred way to despawn Entities
pub struct DespawnToken(pub Token);
impl Command for DespawnToken{
    fn execute(&mut self, world: &mut World) {
        if world.despawn_with_token(self.0){
            world.get_event_writer::<EntityDespawned>().send(EntityDespawned(self.0.id()));
        }
    }
}
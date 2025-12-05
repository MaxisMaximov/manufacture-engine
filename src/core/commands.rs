use super::*;

use events::{EntitySpawned, EntityDespawned};

/// Send a Command to spawn a new Entity
/// 
/// Creates a new entity and sends an event with the Entity's Token
/// 
/// Note: This creates a new Entity *without any Components*, you will have to add those later yourself. If you wish to create an Entity with predefined components, create a Command for it
/// 
/// TODO: Add Prefab Spawn Command
pub struct Spawn;
impl Command for Spawn{
    fn execute(&mut self, World: &mut World) {
        let token = World.spawn().get_token();
        World.get_event_writer::<EntitySpawned>().send(EntitySpawned(token));
    }
}

/// Send a Command to despawn an Entity via ID
/// 
/// It's generally discouraged to despawn Entities this way. If you can, use `DespawnToken` instead
pub struct DespawnID(pub usize);
impl Command for DespawnID{
    fn execute(&mut self, World: &mut World) {
        World.despawn(self.0);
        World.get_event_writer::<EntityDespawned>().send(EntityDespawned(self.0));
    }
}

/// Send a Command to despan Entity via Token
/// 
/// This is the preferred way to despawn Entities
pub struct DespawnToken(pub Token);
impl Command for DespawnToken{
    fn execute(&mut self, World: &mut World) {
        if World.despawn_with_token(self.0){
            World.get_event_writer::<EntityDespawned>().send(EntityDespawned(self.0.id()));
        }
    }
}
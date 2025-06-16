use crate::ECS::world::gmWorld;

/// # Entity struct
/// Identifies a single Entity within the World
/// 
/// Stores it's own ID and Hash for collision checks
pub struct Entity{
    pub id: usize,
    pub hash: u32    
}
impl Entity{
    /// Create a new Entity with given ID
    pub fn new(Id: usize) -> Self{
        Self{
            id: Id,
            hash: rand::random(),
        }
    }
    /// Get a Token for this Entity
    pub fn get_token(&self) -> Token{
        Token{
            id: self.id,
            hash: self.hash,
            valid: true,
        }
    }
    /// Read this Entity's ID
    fn id(&self) -> usize{
        self.id
    }
    /// Read this Entity's Hash
    fn hash(&self) -> u32{
        self.hash
    }
}

/// # Entity Token
/// A "reference" to a specific Entity within the world
/// 
/// Holds the Entity's ID, Hash, and whether it's a valid Token
/// 
/// Tokens whose Entities no longer exist are invalid  
/// This is checked through the Hash value
pub struct Token{
    id: usize,
    hash: u32,
    valid: bool
}
impl Token{
    /// Read the tracked Entity's ID
    pub fn id(&self) -> usize{
        self.id
    }
    /// Read the tracked Entity's Hash
    pub fn hash(&self) -> u32{
        self.hash
    }
    /// Read if the Token is valid
    pub fn valid(&self) -> bool{
        self.valid
    }
    /// Check if the Token is still valid within the World
    /// 
    /// Updates it's own `valid` flag and returns it
    pub fn validate(&mut self, World: &gmWorld) -> bool{
        self.valid = World.validateToken(self);
        self.valid
    }
}
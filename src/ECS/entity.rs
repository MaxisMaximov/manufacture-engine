
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

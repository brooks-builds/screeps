pub type CarryParts = usize;

pub enum Role {
    Worker(CarryParts),
}

impl Default for Role {
    fn default() -> Self {
        Self::Worker(1)
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Not enough energy. Needed {needed:?} but had {had:?}")]
    NotEnoughEnergy { needed: u32, had: u32 },
}

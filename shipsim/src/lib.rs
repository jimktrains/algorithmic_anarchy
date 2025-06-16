pub mod computers;
pub mod engines;
pub mod ships;

pub trait SpaceshipPart {
    fn mass(&self) -> u64;
}

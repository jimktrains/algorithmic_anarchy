pub mod bubbly_byter;
mod spf;

use crate::ships::everyday_explorer::EverydayExplorer;
use crate::SpaceshipPart;
use rvemu::exception::Exception;
use std::fmt;
use std::sync::{Arc, Mutex};

pub trait Computer: SpaceshipPart + fmt::Debug {
    fn reset(&mut self);
    fn load_kernel(&mut self, kernel_img: &str);
    fn execute(&mut self, max_cycle: u64) -> Result<(), Exception>;
    fn set_ship(&mut self, ship: Arc<Mutex<EverydayExplorer>>);
}

pub enum System {
    BubblyByter = 1,
}

pub fn new(system: System) -> impl Computer {
    match system {
        System::BubblyByter => bubbly_byter::BubblyByter::new(),
    }
}

pub fn new_basic() -> Box<dyn Computer> {
    Box::new(bubbly_byter::BubblyByter::new())
}

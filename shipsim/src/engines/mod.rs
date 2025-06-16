use crate::ships::Stuff;
use crate::SpaceshipPart;
use std::fmt;

pub trait Engine: SpaceshipPart + fmt::Debug {
    fn can_use_fuel(&self, stuff: Stuff) -> bool;
    fn isp(&self, stuff: Stuff) -> f64;
    fn current_mass_flow(&self) -> Option<f64>;
    fn set_mass_flow(&mut self, req_flow: f64) -> Option<f64>;
}

#[derive(Debug)]
pub struct Vasmir {
    req_flow: Option<f64>,
}

#[derive(Debug)]
pub struct PulsedPlasma {
    is_active: bool,
}

impl SpaceshipPart for Vasmir {
    fn mass(&self) -> u64 {
        1_000
    }
}
impl Engine for Vasmir {
    fn can_use_fuel(&self, _stuff: Stuff) -> bool {
        true
    }

    fn isp(&self, _stuff: Stuff) -> f64 {
        2_000.0
    }

    fn current_mass_flow(&self) -> Option<f64> {
        self.req_flow
    }

    fn set_mass_flow(&mut self, req_flow: f64) -> Option<f64> {
        self.req_flow = Some(req_flow);
        self.req_flow
    }
}

impl SpaceshipPart for PulsedPlasma {
    fn mass(&self) -> u64 {
        50
    }
}
impl Engine for PulsedPlasma {
    fn can_use_fuel(&self, _stuff: Stuff) -> bool {
        true
    }

    fn isp(&self, _stuff: Stuff) -> f64 {
        500.0
    }

    fn current_mass_flow(&self) -> Option<f64> {
        Some(10.0)
    }

    fn set_mass_flow(&mut self, req_flow: f64) -> Option<f64> {
        self.is_active = req_flow > 0.0;
        self.current_mass_flow()
    }
}

pub fn new_basic_main_engine() -> Box<dyn Engine> {
    Box::new(Vasmir { req_flow: None })
}

pub fn new_basic_attitude_thruster() -> Box<dyn Engine> {
    Box::new(PulsedPlasma { is_active: false })
}

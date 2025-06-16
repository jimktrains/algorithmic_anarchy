pub mod everyday_explorer;

use crate::computers;
use crate::engines;
use crate::engines::Engine;
use crate::SpaceshipPart;

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub enum Stuff {
    Empty = 0,
    CarbonaceousRocks = 1,
    NickelIronRocks = 2,
    Water = 3,
    Argon = 4,
}

#[derive(Debug)]
pub struct StuffBox {
    max_capacity: u64,
    amount: u64,
    stuff: Stuff,
}

impl SpaceshipPart for StuffBox {
    fn mass(&self) -> u64 {
        25 + self.amount
    }
}

impl StuffBox {
    pub fn new(max_capacity: u64, stuff: Stuff) -> StuffBox {
        StuffBox {
            max_capacity: max_capacity,
            amount: 0,
            stuff: stuff,
        }
    }

    pub fn stuff(&self) -> Stuff {
        self.stuff
    }

    pub fn add(&mut self, amount: u64) -> u64 {
        let old_amount = self.amount;
        self.amount = self.amount.saturating_add(amount).max(self.max_capacity);
        self.amount - old_amount
    }

    pub fn remove(&mut self, amount: u64) -> u64 {
        let old_amount = self.amount;
        self.amount = self.amount.saturating_sub(amount);
        old_amount - self.amount
    }
}

#[derive(Debug)]
pub struct AttituteThrusters {
    pub yaw_plus: Box<dyn Engine>,
    pub pitch_plus: Box<dyn Engine>,
    pub roll_plus: Box<dyn Engine>,
    pub yaw_neg: Box<dyn Engine>,
    pub pitch_neg: Box<dyn Engine>,
    pub roll_neg: Box<dyn Engine>,
}

impl SpaceshipPart for AttituteThrusters {
    fn mass(&self) -> u64 {
        self.yaw_plus.mass()
            + self.pitch_plus.mass()
            + self.roll_plus.mass()
            + self.yaw_neg.mass()
            + self.pitch_neg.mass()
            + self.roll_neg.mass()
    }
}

#[derive(Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Default)]
pub struct Iru {
    pub locaction: Vec3,
    pub attitude: Vec3,
    pub velocity: Vec3,
    pub rotational_velocity: Vec3,
}

#[derive(Debug, Default)]
pub struct Imu {
    pub gyro: Vec3,
    pub accl: Vec3,
}

impl SpaceshipPart for Imu {
    fn mass(&self) -> u64 {
        20
    }
}

pub fn new_basic() -> Arc<Mutex<everyday_explorer::EverydayExplorer>> {
    let ship = Arc::new(Mutex::new(everyday_explorer::EverydayExplorer {
        imu: Imu::default(),
        iru: Iru::default(),
        computer: computers::new_basic(),
        attitude_thrusters: AttituteThrusters {
            yaw_plus: engines::new_basic_attitude_thruster(),
            pitch_plus: engines::new_basic_attitude_thruster(),
            roll_plus: engines::new_basic_attitude_thruster(),
            yaw_neg: engines::new_basic_attitude_thruster(),
            pitch_neg: engines::new_basic_attitude_thruster(),
            roll_neg: engines::new_basic_attitude_thruster(),
        },
        engine: engines::new_basic_main_engine(),
        cargo: StuffBox::new(2_000, Stuff::Empty),
        fuel: StuffBox::new(2_000, Stuff::Argon),
        attitude_fuel: StuffBox::new(500, Stuff::Argon),
    }));
    ship.lock().unwrap().computer.set_ship(ship.clone());
    ship
}

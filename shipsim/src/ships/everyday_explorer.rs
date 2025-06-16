use super::AttituteThrusters;
use super::StuffBox;
use crate::computers::Computer;
use crate::engines::Engine;
use crate::ships::Imu;
use crate::ships::Iru;
use crate::SpaceshipPart;

#[derive(Debug)]
pub struct EverydayExplorer {
    pub imu: Imu,
    pub iru: Iru,
    pub computer: Box<dyn Computer>,
    pub attitude_thrusters: AttituteThrusters,
    pub engine: Box<dyn Engine>,
    pub cargo: StuffBox,
    pub fuel: StuffBox,
    pub attitude_fuel: StuffBox,
}

impl EverydayExplorer {
    pub fn get_paramf(&self, param: u64) -> Option<f64> {
        let p = param >> (32 - 12);
        match p {
            0x000 => Some(self.iru.locaction.x),
            0x001 => Some(self.iru.locaction.y),
            0x002 => Some(self.iru.locaction.z),
            0x010 => Some(self.iru.velocity.x),
            0x011 => Some(self.iru.velocity.y),
            0x012 => Some(self.iru.velocity.z),
            0x020 => Some(self.iru.rotational_velocity.x),
            0x021 => Some(self.iru.rotational_velocity.y),
            0x022 => Some(self.iru.rotational_velocity.z),
            0x030 => Some(self.iru.attitude.x),
            0x031 => Some(self.iru.attitude.y),
            0x032 => Some(self.iru.attitude.z),
            0x100 => Some(self.imu.gyro.x),
            0x101 => Some(self.imu.gyro.y),
            0x102 => Some(self.imu.gyro.z),
            0x110 => Some(self.imu.accl.x),
            0x111 => Some(self.imu.accl.y),
            0x112 => Some(self.imu.accl.z),
            0x200 => self.engine.current_mass_flow(),
            _ => None,
        }
    }

    pub fn set_paramf(&mut self, param: u64, v: f64) -> Option<f64> {
        let p = param >> (32 - 12);
        match p {
            0x002 => self.engine.set_mass_flow(v),
            _ => None,
        }
    }
}

impl SpaceshipPart for EverydayExplorer {
    fn mass(&self) -> u64 {
        1_000
            + self.computer.mass()
            + self.attitude_thrusters.mass()
            + self.engine.mass()
            + self.cargo.mass()
            + self.fuel.mass()
            + self.attitude_fuel.mass()
    }
}

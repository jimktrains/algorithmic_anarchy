// ref https://patrickyoussef.com/blog/nbody/

use std::default::Default;
use std::ops::{Add, AddAssign, MulAssign};

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::Split;
use std::time::Instant;

// used for write_all and I don't feel like editing the top and bottom
// of the file if I want the text output back, so here's a comment.
// use std::io::Write;

use uom::si::f64::Length;
use uom::si::length::meter;

use nalgebra::Vector3;

use gravsim::simstate_capnp::physical_constants;
use gravsim::simstate_capnp::sim_state;

use capnp::message;
use capnp::serialize_packed;

type Vec3 = Vector3<f64>;

// OK, so it looks this is 1/g_0 * 10^11.
// I'm not sure where the 10^11 comes from.
const GM_TO_GRAM: f64 = 1.498e19;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PhysicalConstants {
    spkid: u64,
    name: String,
    gm: f64,
    radius: Length,
}

pub struct PhysicalConstantsVec {
    v: Vec<PhysicalConstants>,
}

impl PhysicalConstantsVec {
    pub fn new(v: Vec<PhysicalConstants>) -> Self {
        Self { v: v }
    }

    pub fn push(&mut self, p: PhysicalConstants) {
        self.v.push(p);
    }

    pub fn append(&mut self, p: &mut PhysicalConstantsVec) {
        self.v.append(&mut p.v);
    }

    pub fn by_spkid(&self, spkid: u64) -> Option<&PhysicalConstants> {
        self.v.iter().filter(|p| p.spkid == spkid).last()
    }

    pub fn by_name(&self, name: &str) -> Option<PhysicalConstants> {
        for p in self.v.iter() {
            if p.name.to_uppercase() == name.to_uppercase() {
                return Some(p.clone());
            }
        }
        None
    }
}

impl PhysicalConstants {
    fn new_r_in_km(parts: &mut Split<'_, &str>) -> PhysicalConstants {
        let idx_part = parts.next().unwrap().trim();
        let idx = idx_part
            .parse::<u64>()
            .expect(format!("idx {idx_part} is not a u64").as_str());

        let spkid_part = parts.next().unwrap().trim();
        let name = parts.next().unwrap().trim().to_owned();
        let gm_part = parts.next().unwrap().trim();
        let radius_part = parts.next().unwrap().trim();
        PhysicalConstants {
            spkid: spkid_part
                .parse::<u64>()
                .expect(format!("spkid {spkid_part} is not a valid u64for idx {idx}").as_str()),
            name: name,
            gm: gm_part
                .parse::<f64>()
                .expect(format!("gm {gm_part} is not a valid f64 for idx {idx}").as_str()),
            radius: Length::new::<meter>(
                radius_part.parse::<f64>().expect(
                    format!("radius {radius_part} is not a valid f64 for idx {idx}").as_str(),
                ) * 1000.0,
            ),
        }
    }

    fn from_file_r_in_km<P>(filename: P) -> std::io::Result<PhysicalConstantsVec>
    where
        P: AsRef<Path>,
    {
        let mut res = vec![];
        let f = File::open(filename)?;
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        while 0 != reader.read_line(&mut line)? {
            let mut parts = line.split("\t");
            let s = PhysicalConstants::new_r_in_km(&mut parts);
            res.push(s);
            line.clear();
        }
        Ok(PhysicalConstantsVec::new(res))
    }
}

#[derive(Debug)]
struct InitState {
    spkid: u64,
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}

impl InitState {
    fn new(parts: &mut Split<'_, &str>) -> Self {
        let spkid_part = parts.next().unwrap();
        let spkid = spkid_part
            .parse::<u64>()
            .expect(format!("invalid spkid {spkid_part}").as_str());
        let x_part = parts.next().unwrap().trim();
        let y_part = parts.next().unwrap().trim();
        let z_part = parts.next().unwrap().trim();
        let dx_part = parts.next().unwrap().trim();
        let dy_part = parts.next().unwrap().trim();
        let dz_part = parts.next().unwrap().trim();
        Self {
            spkid: spkid_part
                .parse::<u64>()
                .expect(format!("invalid spkid {spkid_part} for spkid {spkid}").as_str()),
            x: x_part
                .parse::<f64>()
                .expect(format!("invalid x {x_part} for spkid {spkid}").as_str()),
            y: y_part
                .parse::<f64>()
                .expect(format!("invalid y {y_part} for spkid {spkid}").as_str()),
            z: z_part
                .parse::<f64>()
                .expect(format!("invalid z {z_part} for spkid {spkid}").as_str()),
            dx: dx_part
                .parse::<f64>()
                .expect(format!("invalid dx {dx_part} for spkid {spkid}").as_str()),
            dy: dy_part
                .parse::<f64>()
                .expect(format!("invalid dy {dy_part} for spkid {spkid}").as_str()),
            dz: dz_part
                .parse::<f64>()
                .expect(format!("invalid dz {dz_part} for spkid {spkid}").as_str()),
        }
    }

    fn from_file<P>(filename: P) -> std::io::Result<Vec<Self>>
    where
        P: AsRef<Path>,
    {
        let mut res = vec![];
        let f = File::open(filename)?;
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        while 0 != reader.read_line(&mut line)? {
            let mut parts = line.split("\t");
            let s = Self::new(&mut parts);
            res.push(s);
            line.clear();
        }
        Ok(res)
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
struct SimObjDerivative {
    spkid: u64,
    velocity: Vec3,
    acceleration: Vec3,
}

impl SimObjDerivative {
    pub fn step(&self, dt: f64) -> SimObj {
        SimObj {
            position: &self.velocity * dt,
            velocity: &self.acceleration * dt,
            mass: 0.0,
            spkid: self.spkid,
        }
    }

    pub fn apply_acceleration(&mut self, a: &Vec3) {
        self.acceleration += a;
    }
}

impl MulAssign<f64> for SimObjDerivative {
    fn mul_assign(&mut self, rhs: f64) {
        self.velocity *= rhs;
        self.acceleration *= rhs;
    }
}

impl Add for SimObjDerivative {
    type Output = SimObjDerivative;
    fn add(self, rhs: SimObjDerivative) -> Self::Output {
        Self::Output {
            velocity: self.velocity + rhs.velocity,
            acceleration: self.acceleration + rhs.acceleration,
            spkid: self.spkid,
        }
    }
}

impl Add<&SimObjDerivative> for SimObjDerivative {
    type Output = SimObjDerivative;
    fn add(self, rhs: &SimObjDerivative) -> Self::Output {
        Self::Output {
            velocity: self.velocity + &rhs.velocity,
            acceleration: self.acceleration + &rhs.acceleration,
            spkid: self.spkid,
        }
    }
}

impl Add<&SimObjDerivative> for &SimObjDerivative {
    type Output = SimObjDerivative;
    fn add(self, rhs: &SimObjDerivative) -> Self::Output {
        Self::Output {
            velocity: &self.velocity + &rhs.velocity,
            acceleration: &self.acceleration + &rhs.acceleration,
            spkid: self.spkid,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
struct SimObj {
    spkid: u64,
    position: Vec3,
    velocity: Vec3,
    mass: f64,
}

#[allow(dead_code)]
impl SimObj {
    #[allow(non_snake_case)]
    pub fn stable_orbit(&self, G: f64, r: f64) -> f64 {
        (G * self.mass / r).sqrt()
    }

    pub fn derivative(&self) -> SimObjDerivative {
        SimObjDerivative {
            velocity: self.velocity.clone(),
            acceleration: Vec3::default(),
            spkid: self.spkid,
        }
    }
}

impl AddAssign<&SimObj> for SimObj {
    fn add_assign(&mut self, rhs: &SimObj) {
        self.position += &rhs.position;
        self.velocity += &rhs.velocity;
    }
}

impl Add<&SimObj> for &SimObj {
    type Output = SimObj;
    fn add(self, rhs: &SimObj) -> Self::Output {
        let mut new = self.clone();
        new.position += &rhs.position;
        new.velocity += &rhs.velocity;
        new
    }
}

trait Derivative<T> {
    fn step(&self, dt: f64) -> T;
}

trait Differentiable<T>
where
    T: Derivative<Self> + Default,
    Self: Sized,
{
    fn derivative(&self) -> T;
}

#[derive(Debug, Clone)]
struct NBodySimulationDerivative {
    bodies: Vec<SimObjDerivative>,
}

impl Derivative<NBodySimulation> for NBodySimulationDerivative {
    fn step(&self, dt: f64) -> NBodySimulation {
        let mut s = NBodySimulation::default();
        s.bodies.reserve(self.bodies.len());
        for b in self.bodies.iter() {
            s.bodies.push(b.step(dt));
        }
        s
    }
}

impl MulAssign<f64> for NBodySimulationDerivative {
    fn mul_assign(&mut self, rhs: f64) {
        for i in self.bodies.iter_mut() {
            *i *= rhs;
        }
    }
}

impl Default for NBodySimulationDerivative {
    fn default() -> Self {
        NBodySimulationDerivative { bodies: vec![] }
    }
}

impl Add<NBodySimulationDerivative> for NBodySimulationDerivative {
    type Output = NBodySimulationDerivative;
    fn add(self, rhs: NBodySimulationDerivative) -> Self::Output {
        let mut new = self.clone();
        for i in 0..self.bodies.len() {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
struct NBodySimulation {
    bodies: Vec<SimObj>,
    dt: f64,
    G: f64,
    t: f64,
}

impl Add<NBodySimulation> for NBodySimulation {
    type Output = NBodySimulation;
    fn add(self, rhs: NBodySimulation) -> Self::Output {
        let mut new = self.clone();
        for i in 0..self.bodies.len() {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}

impl Add<&NBodySimulation> for NBodySimulation {
    type Output = NBodySimulation;
    fn add(self, rhs: &NBodySimulation) -> Self::Output {
        let mut new = self.clone();
        for i in 0..self.bodies.len() {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}

impl Add<NBodySimulation> for &NBodySimulation {
    type Output = NBodySimulation;
    fn add(self, rhs: NBodySimulation) -> Self::Output {
        let mut new = self.clone();
        for i in 0..self.bodies.len() {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}
impl AddAssign<NBodySimulation> for NBodySimulation {
    fn add_assign(&mut self, rhs: NBodySimulation) {
        for i in 0..self.bodies.len() {
            self.bodies[i] += &rhs.bodies[i];
        }
    }
}

impl Add<&NBodySimulation> for &NBodySimulation {
    type Output = NBodySimulation;
    fn add(self, rhs: &NBodySimulation) -> Self::Output {
        let mut new = self.clone();
        for i in 0..self.bodies.len() {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}

impl Default for NBodySimulation {
    fn default() -> Self {
        NBodySimulation {
            bodies: vec![],
            G: 6.67408e-11,
            dt: 1.0,
            t: 0.0,
        }
    }
}

trait Integrator {
    fn step<'a, T, DT>(self, start: &'a mut T, dt: f64)
    where
        T: Differentiable<DT> + Add<Output = T> + AddAssign + 'a,
        &'a T: Add<T, Output = T>,
        DT: Derivative<T>
            + Default
            + MulAssign<f64>
            + Add<Output = DT>
            + Add<&'a DT, Output = DT>
            + 'a,
        &'a DT: MulAssign<f64>;
}

struct RungeKutta3;

/*
 * error[E0502]: cannot borrow `start` as mutable because it is also borrowed as immutable
 *   --> src/main.rs:456:9
 *    |
 *432 |     fn step<'a, T, DT>(self, mut start: &'a mut T, dt: f64)
 *    |             -- lifetime `'a` defined here
 *...
 *445 |         let s = &*start;
 *    |                 -------
 *    |                 |
 *    |                 immutable borrow occurs here
 *    |                 assignment requires that `*start` is borrowed for `'a`
 *...
 *456 |         start += d.step(dt);
 *    |         ^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
 *
 *For more information about this error, try `rustc --explain E0502`.
 *
 */
impl Integrator for RungeKutta3 {
    fn step<'a, T, DT>(self, start: &'a mut T, dt: f64)
    where
        T: Differentiable<DT> + Add<Output = T> + AddAssign + 'a,
        &'a T: Add<T, Output = T>,
        DT: Derivative<T>
            + Default
            + MulAssign<f64>
            + Add<Output = DT>
            + Add<&'a DT, Output = DT>
            + 'a,
        &'a DT: MulAssign<f64>,
    {
        let s = &*start;
        let k1 = s.derivative();
        let mut k2 = (s + k1.step(dt / 2.0)).derivative();
        let mut k3 = (s + k2.step(dt / 2.0)).derivative();
        let k4 = (s + k3.step(dt)).derivative();

        k2 *= 2.0;
        k3 *= 2.0;

        let mut d = k1 + k2 + k3 + k4;
        d *= 1.0 / 6.0;
        *start += d.step(dt);
    }
}

impl NBodySimulation {
    fn update(&mut self) {
        self.simple_step();
        self.t += self.dt;
    }

    #[allow(dead_code)]
    fn simple_step(&mut self) {
        let d = self.derivative().step(self.dt);
        *self += d;
    }

    #[allow(dead_code)]
    fn runge_kutta_3(&mut self) {
        let k1 = self.derivative();
        let mut k2 = (&*self + k1.step(self.dt / 2.0)).derivative();
        let mut k3 = (&*self + k2.step(self.dt / 2.0)).derivative();
        let k4 = (&*self + k3.step(self.dt)).derivative();

        k2 *= 2.0;
        k3 *= 2.0;

        let mut d = k1 + k2 + k3 + k4;
        d *= 1.0 / 6.0;
        let d = d.step(self.dt);
        *self += d;
    }

    pub fn from_init_state(
        init_states: Vec<InitState>,
        p: &PhysicalConstantsVec,
    ) -> NBodySimulation {
        let mut sim = NBodySimulation::default();
        sim.bodies.reserve(init_states.len());
        for i in init_states {
            if let Some(p) = p.by_spkid(i.spkid) {
                sim.bodies.push(SimObj {
                    spkid: i.spkid,
                    position: Vec3::new(i.x, i.y, i.z),
                    velocity: Vec3::new(i.dx, i.dy, i.dz),
                    mass: p.gm * GM_TO_GRAM,
                });
            } else {
                println!("Cannot find physical constants for spkid {}", i.spkid);
            }
        }
        sim
    }
}
impl Differentiable<NBodySimulationDerivative> for NBodySimulation {
    fn derivative(&self) -> NBodySimulationDerivative {
        let mut d = NBodySimulationDerivative::default();
        d.bodies.reserve(self.bodies.len());
        for b in self.bodies.iter() {
            d.bodies.push(b.derivative());
        }
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                let a = &self.bodies[i];
                let b = &self.bodies[j];
                let r_vec = a.position - b.position;
                let r = r_vec.norm();
                // r^3 because it's r^2 in the law of gravity and an
                // extra r to normalize the r̄ vector.
                let f = r_vec * self.G * a.mass * b.mass / r.powi(3);
                if let Ok([ad, bd]) = d.bodies.get_disjoint_mut([i, j]) {
                    ad.apply_acceleration(&(-&f / a.mass));
                    bd.apply_acceleration(&(&f / b.mass));
                }
            }
        }
        d
    }
}

fn main() -> std::io::Result<()> {
    let mut physical_constants = PhysicalConstants::from_file_r_in_km("../spice/simbodiespc")?;
    let mut init_state = InitState::from_file("../spice/sim-description")?;

    let g_0: f64 = 6.67408e-11;

    physical_constants.push(PhysicalConstants {
        spkid: 999999999001,
        name: "MySatellite".to_owned(),
        gm: 250.0e3 / GM_TO_GRAM,
        radius: Length::new::<meter>(10.0),
    });
    let pc_earth = physical_constants.by_name("earth").unwrap();
    let rz = 6.3781e6 + 1e8;
    let orbit_v = -(g_0 * pc_earth.gm * GM_TO_GRAM / rz).sqrt() - 3.5e2;
    for i in 0..init_state.len() {
        if init_state[i].spkid == pc_earth.spkid {
            let init_earth = &init_state[i];
            init_state.push(InitState {
                spkid: 999999999001,
                x: init_earth.x + rz,
                y: init_earth.y,
                z: init_earth.z,
                dx: init_earth.dx,
                dy: init_earth.dy + orbit_v * 1.0 / (2.0f64.sqrt()),
                dz: init_earth.dz + orbit_v * 1.0 / (2.0f64.sqrt()),
            });
        }
    }

    print!(
        "n_pc={} n_is={} ",
        &physical_constants.v.len(),
        &init_state.len(),
    );

    let mut sim = NBodySimulation::from_init_state(init_state, &physical_constants);
    println!("n_bodies={}", sim.bodies.len());

    let fc = File::create("output.cnp.bin")?;
    let mut message = message::Builder::new_default();
    let state = message.init_root::<physical_constants::Builder>();
    let mut bodies = state.init_bodies(sim.bodies.len() as u32);
    for (i, b) in sim.bodies.iter().enumerate() {
        let pc = physical_constants.by_spkid(b.spkid).unwrap();
        let mut body = bodies.reborrow().get(i as u32);
        body.set_spkid(pc.spkid);
        body.set_name(&pc.name);
        body.set_mass(pc.gm * GM_TO_GRAM);
        body.set_radius(pc.radius.value);
    }
    let r = serialize_packed::write_message(&fc, &message);
    if let Err(e) = r {
        println!("{:?}", e);
    }

    let minute = 60.0;
    let hour = 60.0 * minute;
    let day = 24.0 * hour;
    #[allow(unused_variables)]
    let year = 365.25 * day;

    let t_max = year;
    sim.dt = minute;

    let start = Instant::now();
    while sim.t < t_max {
        sim.update();
        if sim.t % (7.0 * day) < 0.001 {
            println!(
                "time={} (days={}) rate= {} sim-cycles/real-s ({} sim-days/real-s)",
                sim.t,
                sim.t / day,
                (sim.t / sim.dt) / (Instant::now() - start).as_secs_f64(),
                sim.t / day / (Instant::now() - start).as_secs_f64(),
            );
        }

        let mut message = message::Builder::new_default();
        let mut state = message.init_root::<sim_state::Builder>();
        state.set_time(sim.t);
        let mut bodies = state.init_bodies(sim.bodies.len() as u32);

        for (i, b) in sim.bodies.iter().enumerate() {
            let p = &b.position;

            let mut body = bodies.reborrow().get(i as u32);
            body.set_x(p.x);
            body.set_y(p.y);
            body.set_z(p.z);
        }
        let r = serialize_packed::write_message(&fc, &message);
        if let Err(e) = r {
            println!("{:?}", e);
        }
    }
    println!(
        "total time={} (days={}) rate= {} sim-cycles/real-s ({} sim-days/real-s)",
        sim.t,
        sim.t / day,
        (sim.t / sim.dt) / (Instant::now() - start).as_secs_f64(),
        sim.t / day / (Instant::now() - start).as_secs_f64(),
    );
    Ok(())
}

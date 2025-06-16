// ref https://patrickyoussef.com/blog/nbody/

use std::default::Default;
use std::io::Write;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::Split;
use std::time::Instant;

use binary_stream::Endian;
use serde::ser::{SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};

use uom::si::f64::Length;
use uom::si::length::meter;

use nalgebra::Vector3;

// OK, so it looks this is 1/g_0 * 10^11.
// I'm not sure where the 10^11 comes from.
const GM_TO_GRAM: f64 = 1.498e19;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PhysicalConstants {
    spkid: usize,
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

    pub fn by_spkid(&self, spkid: usize) -> Option<&PhysicalConstants> {
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
            .parse::<usize>()
            .expect(format!("idx {idx_part} is not a usize").as_str());

        let spkid_part = parts.next().unwrap().trim();
        let name = parts.next().unwrap().trim().to_owned();
        let gm_part = parts.next().unwrap().trim();
        let radius_part = parts.next().unwrap().trim();
        PhysicalConstants {
            spkid: spkid_part
                .parse::<usize>()
                .expect(format!("spkid {spkid_part} is not a valid usize for idx {idx}").as_str()),
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
    spkid: usize,
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
            .parse::<usize>()
            .expect(format!("invalid spkid {spkid_part}").as_str());
        let x_part = parts.next().unwrap().trim();
        let y_part = parts.next().unwrap().trim();
        let z_part = parts.next().unwrap().trim();
        let dx_part = parts.next().unwrap().trim();
        let dy_part = parts.next().unwrap().trim();
        let dz_part = parts.next().unwrap().trim();
        Self {
            spkid: spkid_part
                .parse::<usize>()
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl Vec3 {
    fn gpformat(&self) -> Vec<u8> {
        format!("{}\t{}\t{}\n", self.x, self.y, self.z).into_bytes()
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[allow(dead_code)]
impl Vec3 {
    pub fn l2_norm(&self) -> f64 {
        let xd = self.x;
        let yd = self.y;
        let zd = self.z;
        (xd.powi(2) + yd.powi(2) + zd.powi(2)).sqrt()
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
struct SimObjDerivative {
    spkid: usize,
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

#[derive(Serialize, Deserialize, Debug)]
struct SimVizState {
    spkid: usize,
    bodyid: usize,
    t: f32,
    x: f32,
    y: f32,
    z: f32,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct SimObj {
    spkid: usize,
    position: Vec3,
    velocity: Vec3,
    mass: f64,
}

// const G_0: f64 = 6.67408e-11;
// const G: f64 = 1.0;

#[allow(dead_code)]
impl SimObj {
    pub fn viz_state(&self, i: usize, t: f64) -> SimVizState {
        SimVizState {
            bodyid: i,
            spkid: self.spkid,
            t: t as f32,
            x: self.position.x as f32,
            y: self.position.y as f32,
            z: self.position.z as f32,
        }
    }

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

#[derive(Debug, Clone)]
struct NBodySimulationDerivative {
    bodies: Vec<SimObjDerivative>,
}

impl NBodySimulationDerivative {
    pub fn step(&self, dt: f64) -> NBodySimulation {
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

impl Serialize for NBodySimulation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(1 + self.bodies.len())?;
        tup.serialize_element(&self.t)?;
        for b in &self.bodies {
            tup.serialize_element(&(b.spkid, b.position.x, b.position.y, b.position.z))?;
        }
        tup.end()
    }
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

impl NBodySimulation {
    fn update(&mut self) {
        let k1 = self.derivative();
        let mut k2 = (&*self + k1.step(self.dt / 2.0)).derivative();
        let mut k3 = (&*self + k2.step(self.dt / 2.0)).derivative();
        let k4 = (&*self + k3.step(self.dt)).derivative();

        k2 *= 2.0;
        k3 *= 2.0;

        let mut d = k1 + k2 + k3 + k4;
        d *= 1.0 / 6.0;
        let d = d.step(self.dt);
        for i in 0..self.bodies.len() {
            self.bodies[i] += &d.bodies[i];
        }

        self.t += self.dt;
    }

    fn derivative(&mut self) -> NBodySimulationDerivative {
        let mut d = NBodySimulationDerivative::default();
        d.bodies.reserve(self.bodies.len());
        for b in self.bodies.iter() {
            d.bodies.push(b.derivative());
        }
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                if let Ok([a, b]) = self.bodies.get_disjoint_mut([i, j]) {
                    let r_vec = &a.position - &b.position;
                    let r = r_vec.l2_norm();
                    // r^3 because it's r^2 in the law of gravity and an
                    // extra r to normalize the r̄ vector.
                    let f = r_vec * self.G * a.mass * b.mass / r.powi(3);
                    if let Ok([ad, bd]) = d.bodies.get_disjoint_mut([i, j]) {
                        ad.apply_acceleration(&(-&f / a.mass));
                        bd.apply_acceleration(&(&f / b.mass));
                    }
                }
            }
        }
        d
    }

    pub fn from_init_state(
        init_states: Vec<InitState>,
        p: PhysicalConstantsVec,
    ) -> NBodySimulation {
        let mut sim = NBodySimulation::default();
        sim.bodies.reserve(init_states.len());
        for i in init_states {
            if let Some(p) = p.by_spkid(i.spkid) {
                sim.bodies.push(SimObj {
                    spkid: i.spkid,
                    position: Vec3 {
                        x: i.x,
                        y: i.y,
                        z: i.z,
                    },
                    velocity: Vec3 {
                        x: i.dx,
                        y: i.dy,
                        z: i.dz,
                    },
                    mass: p.gm * GM_TO_GRAM,
                });
            } else {
                println!("Cannot find physical constants for spkid {}", i.spkid);
            }
        }
        sim
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
    let mut sim = NBodySimulation::from_init_state(init_state, physical_constants);
    println!("n_bodies={}", sim.bodies.len());

    let minute = 60.0;
    let hour = 60.0 * minute;
    let day = 24.0 * hour;
    #[allow(unused_variables)]
    let year = 365.25 * day;

    let t_max = year;
    sim.dt = minute;

    let mut fb = File::create("output.bin")?;
    let mut ft = File::create("output.txt")?;

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
        //f.write_all(&sim.bodies[0].position.gpformat())?;
        let enc_res = serde_binary::to_vec(&sim, Endian::Little);
        if let Ok(bytes) = enc_res {
            fb.write_all(&bytes)?;
        } else {
            println!("{:?}", enc_res);
        }
        // for i in 0..sim.bodies.len() {
        //     let b = &sim.bodies[i];
        //     let p = &b.position;
        //     // let c = COLORS[i];
        //     // let state = b.viz_state(i, sim.t);
        //     // let enc_res = serde_binary::to_vec(&sim, Endian::Little);
        //     // if let Ok(bytes) = enc_res {
        //     //     fb.write_all(&bytes)?;
        //     // } else {
        //     //     println!("{:?}", enc_res);
        //     // }

        //     ft.write_all(
        //         format!("{} {} {} {} {} {}\n", p.x, p.y, p.z, i, sim.t, b.spkid).as_bytes(),
        //     )?;
        // }
        //}
    }
    Ok(())
}

use std::default::Default;
use std::fs::File;
use std::io::Write;
#[allow(dead_code)]
// use rusty_units::dimension::Dimension;
// use rusty_units::units::*;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Default, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

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
    velocity: Vec3,
    acceleration: Vec3,
}

impl SimObjDerivative {
    pub fn step(&self, dt: f64) -> SimObj {
        SimObj {
            position: &self.velocity * dt,
            velocity: &self.acceleration * dt,
            mass: 0.0,
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
        }
    }
}

impl Add<&SimObjDerivative> for SimObjDerivative {
    type Output = SimObjDerivative;
    fn add(self, rhs: &SimObjDerivative) -> Self::Output {
        Self::Output {
            velocity: self.velocity + &rhs.velocity,
            acceleration: self.acceleration + &rhs.acceleration,
        }
    }
}

impl Add<&SimObjDerivative> for &SimObjDerivative {
    type Output = SimObjDerivative;
    fn add(self, rhs: &SimObjDerivative) -> Self::Output {
        Self::Output {
            velocity: &self.velocity + &rhs.velocity,
            acceleration: &self.acceleration + &rhs.acceleration,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
struct SimObj {
    position: Vec3,
    velocity: Vec3,
    mass: f64,
}

// const G: f64 = 6.67408e-11;
// const G: f64 = 1.0;

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
struct NBodySimulationDerivative<const N: usize> {
    bodies: [SimObjDerivative; N],
}

impl<const N: usize> NBodySimulationDerivative<N> {
    pub fn step(&self, dt: f64) -> NBodySimulation<N> {
        let mut s = NBodySimulation::<N>::default();
        for i in 0..N {
            s.bodies[i] = self.bodies[i].step(dt);
        }
        s
    }
}

impl<const N: usize> MulAssign<f64> for NBodySimulationDerivative<N> {
    fn mul_assign(&mut self, rhs: f64) {
        for i in self.bodies.iter_mut() {
            *i *= rhs;
        }
    }
}

impl<const N: usize> Default for NBodySimulationDerivative<N> {
    fn default() -> Self {
        NBodySimulationDerivative {
            bodies: [(); N].map(|_| SimObjDerivative::default()),
        }
    }
}

impl<const N: usize> Add<NBodySimulationDerivative<N>> for NBodySimulationDerivative<N> {
    type Output = NBodySimulationDerivative<N>;
    fn add(self, rhs: NBodySimulationDerivative<N>) -> Self::Output {
        let mut new = self.clone();
        for i in 0..N {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
struct NBodySimulation<const N: usize> {
    bodies: [SimObj; N],
    dt: f64,
    G: f64,
    t: f64,
}

impl<const N: usize> Add<NBodySimulation<N>> for NBodySimulation<N> {
    type Output = NBodySimulation<N>;
    fn add(self, rhs: NBodySimulation<N>) -> Self::Output {
        let mut new = self.clone();
        for i in 0..N {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}

impl<const N: usize> Add<&NBodySimulation<N>> for NBodySimulation<N> {
    type Output = NBodySimulation<N>;
    fn add(self, rhs: &NBodySimulation<N>) -> Self::Output {
        let mut new = self.clone();
        for i in 0..N {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}

impl<const N: usize> Add<NBodySimulation<N>> for &NBodySimulation<N> {
    type Output = NBodySimulation<N>;
    fn add(self, rhs: NBodySimulation<N>) -> Self::Output {
        let mut new = self.clone();
        for i in 0..N {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}

impl<const N: usize> Add<&NBodySimulation<N>> for &NBodySimulation<N> {
    type Output = NBodySimulation<N>;
    fn add(self, rhs: &NBodySimulation<N>) -> Self::Output {
        let mut new = self.clone();
        for i in 0..N {
            new.bodies[i] = &self.bodies[i] + &rhs.bodies[i];
        }
        new
    }
}

impl<const N: usize> Default for NBodySimulation<N> {
    fn default() -> Self {
        NBodySimulation {
            bodies: [(); N].map(|_| SimObj::default()),
            G: 6.67408e-11,
            dt: 1.0,
            t: 0.0,
        }
    }
}

impl<'a, const N: usize> NBodySimulation<N> {
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
        for i in 0..N {
            self.bodies[i] += &d.bodies[i];
        }

        self.t += self.dt;
    }

    fn derivative(&mut self) -> NBodySimulationDerivative<N> {
        let mut d = NBodySimulationDerivative::<N>::default();
        for (i, b) in self.bodies.iter().enumerate() {
            d.bodies[i] = b.derivative();
        }
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                if let Ok([a, b]) = self.bodies.get_disjoint_mut([i, j]) {
                    let r_vec = &a.position - &b.position;
                    let r = r_vec.l2_norm();
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

    pub fn from_matrix(m: &[[f64; 7]]) -> NBodySimulation<N> {
        let mut sim = NBodySimulation::<N>::default();

        for i in 0..N {
            sim.bodies[i].position.x = m[i][0];
            sim.bodies[i].position.y = m[i][1];
            sim.bodies[i].position.z = m[i][2];

            sim.bodies[i].velocity.x = m[i][3];
            sim.bodies[i].velocity.y = m[i][4];
            sim.bodies[i].velocity.z = m[i][5];

            sim.bodies[i].mass = m[i][6];
        }

        sim
    }
}

fn main() -> std::io::Result<()> {
    // let r_earth: f64 = 6.3781e6;
    // let r_sat: f64 = 760e3;
    // let m_earth: f64 = 5.974e24;
    // let m_sat: f64 = 250.0;

    // Stable Figure 8
    // #[rustfmt::skip]
    // let mut sim = NBodySimulation::<3>::from_matrix(&[
    //     [-0.3092050 ,  0.0        , 0.0 ,  0.0        , -0.50436399 , 0.0 , 1.0 / 3.0] ,
    //     [ 0.1546025 , -0.09875616 , 0.0 , -1.18437049 ,  0.25218199 , 0.0 , 1.0 / 3.0] ,
    //     [ 0.1546025 ,  0.09875616 , 0.0 ,  1.18437049 ,  0.25218199 , 0.0 , 1.0 / 3.0] ,
    // ]);

    // sim.G = 1.0;
    // sim.dt = 1e-3;
    // let t_max = 3.50;

    #[rustfmt::skip]
    let mut sim = NBodySimulation::<171>::from_matrix(&[
    //     // SUN
    //     [1.81899E+8   , 9.83630E+8   , -1.58778E+8  , -1.12474E+1  , 7.54876E+0   , 2.68723E-1    , 1.98854E+30] ,
    //     // Mercury
    //     [-5.67576E+10 , -2.73592E+10 , 2.89173E+9   , 1.16497E+4   , -4.14793E+4  , -4.45952E+3   , 3.30200E+23] ,
    //     // Venus
    //     [4.28480E+10  , 1.00073E+11  , -1.11872E+09 , -3.22930E+04 , 1.36960E+04  , 2.05091E+03   , 4.86850E+24] ,
    //     // Earth
    //     [-1.43778E+11 , -4.00067E+10 , -1.38875E+07 , 7.65151E+03  , -2.87514E+04 , 2.08354E+00   , 5.97219E+24] ,
    //     // Moon
    //     [-1.43842359E+11,-4.04035995E+10,1.40544867E+07,8.60820833E+03, -2.89425909E+04, -4.93064422E+01, 	7.346E22],
    //     // Mars
    //     [-1.14746E+11 , -1.96294E+11 , -1.32908E+09 , 2.18369E+04  , -1.01132E+04 , -7.47957E+02  , 6.41850E+23] ,
    //     // Jupiter
    //     [-5.66898244E+11, -5.77495658E+11,  1.50754956E+10,  9.16794553E+03, -8.53243020E+03, -1.69767366E+02, 1.89813E+27],
    //     // [-5.66899E+11   , -5.77495E+11   ,  1.50755E+10  ,   9.16793E+03  ,  -8.53244E+03, -1.69767E+02  , 1.89813E+27] ,
    //     // Saturn
    //     [8.20513E+10  , -1.50241E+12 , 2.28565E+10  , 9.11312E+03  , 4.96372E+02  , -3.71643E+02  , 5.68319E+26] ,
    //     // Neptine
    //     [2.62506E+12  , 1.40273E+12  , -2.87982E+10 , -3.25937E+03 , 5.68878E+03  , 6.32569E+01   , 8.68103E+25] ,
    //     // Uranas
    //     [4.30300E+12  , -1.24223E+12 , -7.35857E+10 , 1.47132E+03  , 5.25363E+03  , -1.42701E+02  , 1.02410E+26] ,
    //     // Pluto
    //     [1.65554E+12  , -4.73503E+12 , 2.77962E+10  , 5.24541E+03  , 6.38510E+02  , -1.60709E+03  , 1.30700E+22],
    //     // IO
    //     [-5.66484294E+11,-5.77584810E+11,  1.50781135E+10,  1.27817075E+04, 8.34018110E+03, 4.87423691E+02, 8.931938E+22],
    //     // Europa
    //     [-5.67121973E+11,-5.78126978E+11,1.50442790E+10,2.21744712E+04,-1.30018270E+04,-1.11889690E+02, 	4.7998e22],
    //     // Kleopatra
    //     [3.01319079E+11,  9.54200757E+10,  2.26385079E+10, -7.72788312E+03,  2.09052414E+04, -5.01025165E+03, 3E18],
    //     // Eros
    //     [1.72133283E+11,-2.01945835E+11,5.26601305E+09,1.42570599E+04,1.31272739E+04,3.66911542E+03,6.6E15],
    
    //   0 SUN (soleil)
    [181791441.0,983586429.0,-15862871.7,-11.247433,7.54873918,0.26870813,1.989e30],
    //   1 MERCURY (mercure)
    [-56756892400.0,-27362120500.0,2891439180.0,11651.7117,-41478.2712,-4459.6276100000005,3.30114e23],
    //   2 VENUS (venus)
    [42845612200.0,100073999000.0,-1118564860.0,-32293.298,13695.317,2050.9152400000003,4.86747e24],
    //   3 EARTH (terre)
    [-143778018000.0,-40008764800.0,-13872507.600000001,7651.9051,-28751.263000000003,2.08365652,5.97237e24],
    //   4 MOON (lune) (around terre)
    [-143842359000.0,-40403599500.0,14054486.7,8608.20833,-28942.5909,-49.3064422,7.346e22],
    //   5 MARS (mars)
    [-114744935000.0,-196295080000.0,-1329114000.0,21836.9598,-10113.0524,-747.956241,6.41712e23],
    //   6 DEIMOS (deimos) (around mars)
    [-114732192000.0,-196313255000.0,-1336708500.0,22819.162,-9265.93877,-1126.33673,1.4762e15],
    //   7 PHOBOS (phobos) (around mars)
    [-114739293000.0,-196288170000.0,-1331632070.0,20403.0628,-8690.630009999999,23.7325114,1.06e16],
    //   8 JUPITER (jupiter)
    [-566898244000.0,-577495658000.0,15075495600.0,9167.94553,-8532.4302,-169.767366,1.89819e27],
    //   9 ADRASTEA (adrastee) (around jupiter)
    [-566783620000.0,-577554916000.0,15075056300.0,23585.8093,19373.923300000002,1043.28769,2.0e15],
    //  10 AITNE (aitne) (around jupiter)
    [-574739861000.0,-549903747000.0,12061151500.0,10818.582400000001,-7985.66881,254.287474,4.5e13],
    //  11 AMALTHEA (amalthee) (around jupiter)
    [-566750517000.0,-577599886000.0,15073718100.0,24501.7872,13134.937100000001,657.7883850000001,7.5e18],
    //  12 ANANKE (ananke) (around jupiter)
    [-587465699000.0,-568232333000.0,2141849970.0000002,9541.723810000001,-6673.26685,-38.8636796,4.0e16],
    //  13 AOEDE (aoede) (around jupiter)
    [-544428427000.0,-596201610000.0,3072248480.0,8789.31198,-10048.2307,54.928479,9.0e13],
    //  14 ARCHE (arche) (around jupiter)
    [-553508817000.0,-559997792000.0,14740926100.0,11448.4149,-9376.66427,351.376691,4.15e13],
    //  15 AUTONOE (autonoe) (around jupiter)
    [-581768136000.0,-559206424000.0,6937260910.0,10446.8141,-7062.33441,-1092.3068899999998,9.0e13],
    //  16 CALLIRRHOE (callirrhoe) (around jupiter)
    [-588198351000.0,-575344641000.0,18093444700.0,8117.9219,-6485.606040000001,-1234.85267,8.7e14],
    //  17 CALLISTO (callisto) (around jupiter)
    [-565774484000.0,-578997235000.0,15043040400.0,15731.1574,-3562.7346000000002,75.09034000000001,1.0759e23],
    //  18 CARME (carme) (around jupiter)
    [-588275259000.0,-589252348000.0,13643235400.0,8546.60093,-6447.4005099999995,390.113342,1.0e17],
    //  19 CARPO (carpo) (around jupiter)
    [-558842494000.0,-587529040000.0,-760246482.0,10482.9498,-7266.02243,1059.9748499999998,4.5e13],
    //  20 CHALDENE (chaldene) (around jupiter)
    [-565831683000.0,-595318638000.0,10879635000.0,6326.0249699999995,-8565.96944,219.46827199999998,7.5e13],
    //  21 CYLLENE (cyllene) (around jupiter)
    [-554615187000.0,-601691780000.0,23348792900.0,8048.598419999999,-9706.33905,697.366736,1.5e13],
    //  22 DAVIDA (s2022j3) (around jupiter)
    [419613737000.0,-244968687000.0,-92946896000.0,4781.59847,15041.6431,-2602.99516, 26e18],
    //  23 DIA (dia) (around jupiter)
    [-556547715000.0,-571557711000.0,20312879800.0,7493.26425,-6180.40562,-1077.66562,9.0e13],
    //  24 EIRENE (eirene) (around jupiter)
    [-555668271000.0,-562950141000.0,12738362200.0,11492.955399999999,-10040.6434,-942.920251,9.0e13],
    //  25 ELARA (elara) (around jupiter)
    [-571589256000.0,-564840760000.0,17611357900.0,7013.00717,-9694.074519999998,1091.89968,8.0e17],
    //  26 ERINOME (erinome) (around jupiter)
    [-548866065000.0,-572330542000.0,15513426400.0,9229.05327,-11298.9473,458.68636699999996,4.5e13],
    //  27 EUANTHE (euanthe) (around jupiter)
    [-561062550000.0,-559973404000.0,3419300190.0,11053.7505,-9904.67188,-576.778461,4.5e13],
    //  28 EUKELADE (eukelade) (around jupiter)
    [-542308089000.0,-572714147000.0,7976983590.0,9281.84932,-10598.2071,-217.488947,9.0e13],
    //  29 EUPHEME (eupheme) (around jupiter)
    [-562289031000.0,-596265657000.0,17486118400.0,6884.2833900000005,-8095.462940000001,1045.03375,1.5e13],
    //  30 EUPORIE (euporie) (around jupiter)
    [-581238131000.0,-587671235000.0,3597620630.0,7881.626759999999,-6587.30905,-540.335856,1.5e13],
    //  31 EUROPA (europe) (around jupiter)
    [-567121973000.0,-578126978000.0,15044279000.0,22174.4712,-13001.827000000001,-111.88969,4.8e22],
    //  32 EURYDOME (eurydome) (around jupiter)
    [-545227387000.0,-577508122000.0,17402912700.0,10040.4056,-10554.5029,955.770579,4.5e13],
    //  33 GANYMEDE (ganymede) (around jupiter)
    [-566475681000.0,-578477122000.0,15043981900.0,19172.8137,-4227.2144,126.24042599999999,1.4819e23],
    //  34 HARPALYKE (harpalyke) (around jupiter)
    [-580861313000.0,-560875252000.0,6127221970.0,10326.6234,-6897.81394,594.348925,1.2e14],
    //  35 HEGEMONE (hegemone) (around jupiter)
    [-564713131000.0,-606371331000.0,24858131200.0,7622.18463,-8050.81876,-716.155646,4.5e13],
    //  36 HELIKE (helice) (around jupiter)
    [-573766298000.0,-594793169000.0,6019926750.0,7020.18341,-7248.258940000001,-181.564451,9.0e13],
    //  37 HERMIPPE (hermippe) (around jupiter)
    [-562016600000.0,-560803609000.0,8095793680.0,11689.258,-9092.811899999999,772.044661,9.0e13],
    //  38 HERSE (herse) (around jupiter)
    [-592304093000.0,-561589137000.0,11179893100.0,9902.98115,-7110.35301,-599.720024,1.5e13],
    //  39 HIMALIA (himalia) (around jupiter)
    [-560464074000.0,-585578462000.0,9100110020.0,11320.186599999999,-6192.2697,-99.18617959999999,9.5e18],
    //  40 IO (io) (around jupiter)
    [-566484294000.0,-577584810000.0,15078113500.0,12781.7075,8340.1811,487.423691,8.932e22],
    //  41 IOCASTE (iocaste) (around jupiter)
    [-579011249000.0,-560376980000.0,9978758790.0,10649.3799,-7150.17878,-1397.94116,1.9e14],
    //  42 ISONOE (isonoe) (around jupiter)
    [-595012017000.0,-578014772000.0,18124733800.0,9032.60415,-6731.09759,305.171651,7.5e13],
    //  43 KALE (cale) (around jupiter)
    [-586201082000.0,-559169689000.0,15635867800.0,10747.1792,-7485.6963399999995,333.00120100000004,1.5e13],
    //  44 KALLICHORE (callichore) (around jupiter)
    [-577680107000.0,-599490314000.0,11239090800.0,7093.31265,-7932.68333,-607.961005,1.5e13],
    //  45 KALYKE (kalyke) (around jupiter)
    [-547061814000.0,-597436742000.0,15978869100.0,7685.89854,-9502.5604,-649.198119,1.9e14],
    //  46 KORE (core) (around jupiter)
    [-549668589000.0,-593009837000.0,31983971300.0,7602.0331,-9444.73884,131.815119,1.5e13],
    //  47 LEDA (leda) (around jupiter)
    [-568917424000.0,-587774486000.0,20121961300.0,12210.5325,-9274.72763,629.582466,6.0e17],
    //  48 LYSITHEA (lysithea) (around jupiter)
    [-563985044000.0,-589138964000.0,9808366120.0,11943.6304,-7919.32971,469.899764,8.0e16],
    //  49 MEGACLITE (megaclite) (around jupiter)
    [-581248202000.0,-592147139000.0,25992871800.0,6925.25194,-7799.32789,-479.90182,2.1e14],
    //  50 METIS (metis) (around jupiter)
    [-566795708000.0,-577572129000.0,15074268500.0,28029.353499999997,16777.9117,1013.31336,1.0e17],
    //  51 MNEME (mneme) (around jupiter)
    [-590815515000.0,-569403297000.0,1892153860.0,9597.09563,-6943.54023,-219.633441,1.5e13],
    //  52 ORTHOSIE (orthosie) (around jupiter)
    [-580824899000.0,-595775355000.0,26007091100.0,7580.12885,-7699.35041,744.3106809999999,1.5e13],
    //  53 PASIPHAE (pasiphae) (around jupiter)
    [-558565304000.0,-603890282000.0,28923844800.0,7597.15086,-8191.9419100000005,-590.2171870000001,3.0e17],
    //  54 PASITHEE (pasithee) (around jupiter)
    [-585480205000.0,-573197070000.0,11309724300.0,10331.284,-6084.5543,-426.567065,1.5e13],
    //  55 PHILOPHROSYNE (philophrosyne) (around jupiter)
    [-546165073000.0,-583860111000.0,3265899690.0,8647.68323,-10409.1744,699.835128,1.5e13],
    //  56 PRAXIDIKE (praxidike) (around jupiter)
    [-568101252000.0,-564040851000.0,7806573850.0,12256.9512,-8705.54357,-1116.19881,4.3e14],
    //  57 SINOPE (sinope) (around jupiter)
    [-591753268000.0,-582695775000.0,15626950700.0,8039.255280000001,-6816.37122,-903.013875,8.0e16],
    //  58 SPONDE (sponde) (around jupiter)
    [-569682789000.0,-605045037000.0,1576616760.0,7516.1618,-8484.90887,199.920163,1.5e13],
    //  59 TAYGETE (taygete) (around jupiter)
    [-559803680000.0,-603607924000.0,21845135600.0,7404.75151,-9152.540509999999,-134.339537,1.6e14],
    //  60 THEBE (thebe) (around jupiter)
    [-566696818000.0,-577410588000.0,15082186300.0,-57.3343085,13879.6062,945.215837,8.0e17],
    //  61 THELXINOE (thelxinoe) (around jupiter)
    [-560477718000.0,-555785637000.0,20248198400.0,11158.2486,-8785.46895,-1139.51208,1.5e13],
    //  62 THEMISTO (themisto) (around jupiter)
    [-571310829000.0,-574855910000.0,11500484600.0,5205.82533,-10882.6346,1272.44849,6.9e14],
    //  63 THYONE (thyone) (around jupiter)
    [-546680421000.0,-573991694000.0,1417796740.0,10025.8698,-10409.426399999998,0.928590719,9.0e13],
    //  64 SATURN (saturne)
    [82051851400.0,-1502412370000.0,22856478900.0,9113.12193,496.37608500000005,-371.642536,5.68336e26],
    //  65 AEGAEON (egeon) (around saturne)
    [82142602300.0,-1502540490000.0,22914819500.0,21716.7574,7253.6223,-5133.64153,1.0e11],
    //  66 AEGIR (aegir) (around saturne)
    [84406191300.0,-1517736490000.0,19420929500.0,7440.15489,122.71711,-279.069852,1.5e14],
    //  67 ALBIORIX (albiorix) (around saturne)
    [101899578000.0,-1503333350000.0,8928159880.0,8986.33353,1364.0186899999999,-274.834002,2.23e16],
    //  68 ANTHE (anthe) (around saturne)
    [82006453800.0,-1502240400000.0,22770749500.0,-4345.33022,-1809.55511,2143.8638600000004,5.0e12],
    //  69 ATLAS (atlas) (around saturne)
    [82187690800.0,-1502433610000.0,22854454200.0,11271.679900000001,15040.591600000002,-8200.83852,7.0e15],
    //  70 BEBHIONN (bebhionn) (around saturne)
    [103008967000.0,-1494786310000.0,18153328400.0,8915.61757,1270.4384200000002,-1038.37046,1.5e14],
    //  71 BERGELMIR (bergelmir) (around saturne)
    [75269830400.0,-1522449990000.0,18765996200.0,7961.05174,770.695874,36.229481799999995,1.5e14],
    //  72 BESTLA (bestla) (around saturne)
    [102984706000.0,-1508181840000.0,18171468500.0,8212.38962,-21.6773403,313.470304,2.3e14],
    //  73 CALYPSO (calypso) (around saturne)
    [81895928200.0,-1502625270000.0,22987780000.0,18709.892,-5110.87537,1921.63057,6.5e15],
    //  74 DAPHNIS (daphnis) (around saturne)
    [81946150300.0,-1502332070000.0,22824646600.0,-1363.75342,-10589.259900000001,6451.759550000001,1.0e14],
    //  75 DIONE (dione) (around saturne)
    [81988323800.0,-1502739100000.0,23033887800.0,18963.9496,-1414.50738,-320.268844,1.095e21],
    //  76 ENCELADUS (encelade) (around saturne)
    [81919600600.0,-1502232150000.0,22774812800.0,-1370.08105,-5283.16938,3673.1801800000003,1.08e20],
    //  77 EPIMETHEUS (epimethee) (around saturne)
    [81941284000.0,-1502317080000.0,22818001000.0,-1735.24693,-9381.50109,5776.362,5.3e17],
    //  78 ERRIAPUS (erriapo) (around saturne)
    [77228369300.0,-1525339620000.0,35574691500.0,9765.58964,105.72255,-722.5439240000001,6.8e14],
    //  79 FARBAUTI (farbauti) (around saturne)
    [66247430500.0,-1506295140000.0,18626309700.0,8803.67755,1990.52026,174.676603,9.0e13],
    //  80 FENRIR (fenrir) (around saturne)
    [69692881300.0,-1522758580000.0,24423915900.0,8172.5446,1199.1643000000001,-37.717869,5.0e13],
    //  81 FORNJOT (fornjot) (around saturne)
    [79856878000.0,-1523209440000.0,24582719200.0,7690.3179,539.053085,-84.1953832,1.5e14],
    //  82 GREIP (greip) (around saturne)
    [71709411700.0,-1509087320000.0,23943124200.0,8190.01323,2278.0698899999998,-574.00656,1.5e14],
    //  83 HATI (hati) (around saturne)
    [97273602800.0,-1500235840000.0,20229953500.0,8959.70863,-1140.47354,95.4068642,1.5e14],
    //  84 HYPERION (hyperion) (around saturne)
    [80686396100.0,-1503186430000.0,23375629800.0,11781.0569,-2846.25304,1044.82421,5.6e18],
    //  85 HYRROKKIN (hyrrokkin) (around saturne)
    [73600893000.0,-1496955520000.0,18046599800.0,10303.1185,2314.7056199999997,-280.881027,3.5e14],
    //  86 IAPETUS (japet) (around saturne)
    [80382476800.0,-1505454860000.0,23897822000.0,11924.8408,-1035.4694900000002,-585.193094,1.805e21],
    //  87 IJIRAQ (ijiraq) (around saturne)
    [75786830200.0,-1495400040000.0,20665041600.0,8381.58424,-694.562057,1230.3159699999999,1.18e15],
    //  88 JANUS (janus) (around saturne)
    [81977000500.0,-1502291810000.0,22800759000.0,-4562.114689999999,-5831.6865099999995,4221.7525,1.9e18],
    //  89 JARNSAXA (jarnsaxa) (around saturne)
    [93034143100.0,-1487871490000.0,20621814000.0,10456.1161,-24.7825937,-58.8023679,1.0e14],
    //  90 KARI (kari) (around saturne)
    [103023342000.0,-1505748510000.0,12804410700.0,9511.35865,-640.672357,-207.584779,2.3e14],
    //  91 KIVIUQ (kiviuq) (around saturne)
    [72407303800.0,-1506376800000.0,16000318200.0,9961.258530000001,-563.1016970000001,-1311.14796,2.79e15],
    //  92 LOGE (loge) (around saturne)
    [91477574200.0,-1518371130000.0,26469876400.0,7801.66589,-277.86108199999995,-149.038044,1.5e14],
    //  93 METHONE (methone) (around saturne)
    [82175631400.0,-1502285160000.0,22777814600.0,-1613.53494,8846.090119999999,-3709.17883,2.0e13],
    //  94 MIMAS (mimas) (around saturne)
    [81988565100.0,-1502564890000.0,22946836800.0,22248.6731,-4543.090099999999,704.630137,3.79e19],
    //  95 MUNDILFARI (mundilfari) (around saturne)
    [101214771000.0,-1497489170000.0,26400440500.0,9667.97587,-688.790591,-295.879977,2.3e14],
    //  96 NARVI (narvi) (around saturne)
    [93651150500.0,-1495873780000.0,26870173300.0,9868.21551,-678.572256,-1613.63587,2.3e14],
    //  97 PAALIAQ (paaliaq) (around saturne)
    [92516292200.0,-1489643570000.0,37781707200.0,8266.8821,898.294057,-366.41569899999996,7.25e15],
    //  98 PALLENE (pallene) (around saturne)
    [81840857200.0,-1502396180000.0,22867760200.0,8493.17663,-11329.5009,5907.05599,6.0e13],
    //  99 PAN (pan) (around saturne)
    [82148416800.0,-1502497650000.0,22891806100.0,20696.2949,10891.582100000001,-6940.69849,4.95e15],
    //  100 PANDORA (pandore) (around saturne)
    [82192486600.0,-1502412900000.0,22843258500.0,8411.3244,15084.5555,-7940.86636,1.4e17],
    //  101 PHOEBE (phoebe) (around saturne)
    [70599802200.0,-1495965160000.0,24264753700.0,9703.996480000002,2060.82633,-437.12988,8.292e18],
    //  102 POLYDEUCES (pollux) (around saturne)
    [81742763300.0,-1502600160000.0,22983909900.0,14986.1708,-6762.27526,2835.35659,1.0e13],
    //  103 PROMETHEUS (promethee) (around saturne)
    [82165050300.0,-1502488850000.0,22885581400.0,18683.6952,11996.9046,-7327.49023,1.6e17],
    //  104 RHEA (rhea) (around saturne)
    [82427631500.0,-1502752730000.0,22999147300.0,15020.5684,5670.89353,-3598.8708699999997,2.3e21],
    //  105 SIARNAQ (siarnaq) (around saturne)
    [78333916600.0,-1497225100000.0,29131670800.0,7153.014810000001,-1051.94494,-41.1953242,4.35e16],
    //  106 SKATHI (skathi) (around saturne)
    [80206336200.0,-1520728850000.0,28211711000.0,8014.42001,662.560421,177.987663,3.5e14],
    //  107 SKOLL (skoll) (around saturne)
    [72728842900.0,-1521025470000.0,30859797500.0,8353.10708,1315.3658599999999,-316.47157500000003,1.5e14],
    //  108 SURTUR (surtur) (around saturne)
    [76200830400.0,-1471736120000.0,24103746700.0,9983.25804,572.346382,-583.214567,1.5e14],
    //  109 SUTTUNGR (suttungr) (around saturne)
    [101779888000.0,-1493212110000.0,21011592500.0,9691.02047,-583.04575,-443.121276,2.3e14],
    //  110 TARQEQ (tarqeq) (around saturne)
    [78963187500.0,-1487682830000.0,28138816900.0,8159.60739,-203.122055,738.0961649999999,2.3e14],
    //  111 TARVOS (tarvos) (around saturne)
    [85556381400.0,-1513834880000.0,16948695100.000002,10231.9743,1980.96289,-779.694661,2.3e15],
    //  112 TELESTO (telesto) (around saturne)
    [82344695300.0,-1502417440000.0,22824727200.0,8708.98077,10511.054100000001,-5706.0867499999995,1.0e16],
    //  113 TETHYS (tethys) (around saturne)
    [82202034600.0,-1502641380000.0,22965125900.0,18820.7319,5158.5378,-3966.40559,6.18e20],
    //  114 THRYMR (thrymr) (around saturne)
    [87895157200.0,-1488166350000.0,22357099600.0,10852.5106,631.794615,-541.945058,2.3e14],
    //  115 TITAN (titan) (around saturne)
    [81301730200.0,-1503250910000.0,23363178700.0,13555.9591,-2571.0411999999997,767.935262,1.3452e23],
    //  116 YMIR (ymir) (around saturne)
    [99075002400.0,-1523790190000.0,19256269500.0,8114.862459999999,150.410969,-305.310078,3.97e15],
    //  117 URANUS (uranus)
    [2625058610000.0,1402735060000.0,-28798360200.0,-3259.37743,5688.77758,63.26113360000001,8.68127e25],
    //  118 ARIEL (ariel) (around uranus)
    [2625019090000.0,1402717890000.0,-28984402000.0,-8520.43808,6980.02868,1058.4750099999999,12.9e20],
    //  119 BELINDA (belinda) (around uranus)
    [2625130560000.0,1402717470000.0,-28811662200.0,-5036.39702,4887.8253,-8496.28124,4.9e17],
    //  120 BIANCA (bianca) (around uranus)
    [2625080610000.0,1402722880000.0,-28851948800.0,-12214.8561,7108.9749,-3925.16175,9.2e16],
    //  121 CALIBAN (caliban) (around uranus)
    [2632668150000.0,1401750480000.0,-29098164900.0,-3386.4366,5060.19144,-474.83507099999997,2.5e17],
    //  122 CORDELIA (cordelia) (around uranus)
    [2625077080000.0,1402737540000.0,-28752180900.0,6511.46655,3004.41142,-3706.3068000000003,4.5e16],
    //  123 CRESSIDA (cressida) (around uranus)
    [2625011810000.0,1402750530000.0,-28761178300.0,2719.02797,5443.15629,7691.589150000001,3.4e17],
    //  124 CUPID (cupid) (around uranus)
    [2624994930000.0,1402753690000.0,-28764082200.0,926.729592,5866.60494,7803.6957600000005,3.8e15],
    //  125 DESDEMONA (desdemona) (around uranus)
    [2625024430000.0,1402735330000.0,-28850835200.0,-11055.7181,8110.98986,5156.95782,1.8e17],
    //  126 FERDINAND (ferdinand) (around uranus)
    [2615722680000.0,1376593720000.0,-31026359300.0,-3603.46447,5789.25444,132.386441,5.4e15],
    //  127 FRANCISCO (francisco) (around uranus)
    [2627716730000.0,1400528920000.0,-27479728500.0,-3841.56344,4609.55763,-461.425143,7.2e15],
    //  128 JULIET (juliet) (around uranus)
    [2625053370000.0,1402727400000.0,-28862083400.0,-12499.4844,7784.74004,575.8354730000001,5.6e17],
    //  129 MAB (mab) (around uranus)
    [2624980050000.0,1402744530000.0,-28855282500.0,-7533.64898,7492.1449,6234.2647,1.0e16],
    //  130 MARGARET (margaret) (around uranus)
    [2630683190000.0,1402442280000.0,-31878744000.0,-3708.70334,6291.138540000001,979.888729,5.4e15],
    //  131 MIRANDA (miranda) (around uranus)
    [2625165080000.0,1402730840000.0,-28724039700.0,450.050032,4408.74411,-5339.54359,6.6e19],
    //  132 OBERON (oberon) (around uranus)
    [2625410290000.0,1402724690000.0,-28332866000.0,-836.6137769999999,4898.338360000001,-1789.66489,28.8e20],
    //  133 OPHELIA (ophelia) (around uranus)
    [2625051200000.0,1402729440000.0,-28850956900.0,-13391.5331,8082.89875,1313.78876,5.4e16],
    //  134 PERDITA (perdita) (around uranus)
    [2625029650000.0,1402750900000.0,-28729558700.0,4610.47425,4463.59295,3644.3832,1.8e19],
    //  135 PORTIA (portia) (around uranus)
    [2624999750000.0,1402744020000.0,-28827032900.0,-7023.9766,7673.22627,8409.15534,1.7e18],
    //  136 PROSPERO (prospero) (around uranus)
    [2620507970000.0,1411754040000.0,-31840462400.0,-2603.58412,5997.28143,-400.8234,8.5e16],
    //  137 PUCK (puck) (around uranus)
    [2625072060000.0,1402743360000.0,-28713829600.0,4661.95729,3815.79257,-1013.2190700000001,2.9e18],
    //  138 ROSALIND (rosalind) (around uranus)
    [2625044840000.0,1402728600000.0,-28866614000.0,-11979.7674,7806.85882,1622.19412,0.25e17],
    //  139 SETEBOS (setebos) (around uranus)
    [2633803730000.0,1427835620000.0,-29614606900.0,-3006.28826,5574.72514,-107.708678,7.5e16],
    //  140 STEPHANO (stephano) (around uranus)
    [2632361430000.0,1398912540000.0,-32720435700.0,-3712.72613,5202.781739999999,-251.16906900000004,2.2e16],
    //  141 SYCORAX (sycorax) (around uranus)
    [2630682630000.0,1397262750000.0,-32061285700.0,-4029.5073300000004,5263.11767,400.04381,2.3e18],
    //  142 TITANIA (titania) (around uranus)
    [2624775240000.0,1402750850000.0,-29129818800.0,-5923.54588,6596.80965,2377.4414300000003,34.2e20],
    //  143 TRINCULO (trinculo) (around uranus)
    [2619923060000.0,1394249040000.0,-30326764000.0,-3880.16761,5963.46125,176.556426,3.9e15],
    //  144 UMBRIEL (umbriel) (around uranus)
    [2624814800000.0,1402799500000.0,-28716589600.0,-1704.57312,5959.12379,4472.0871400000005,12.2e20],
    //  145 NEPTUNE (neptune)
    [4303002260000.0,-1242228790000.0,-73585854400.0,1471.32544,5253.63425,-142.70049300000002,1.02413e26],
    //  146 DESPINA (despina) (around neptune)
    [4303030890000.0,-1242265240000.0,-73610567700.0,10115.430999999999,12652.2574,-1042.18418,2.0e18],
    //  147 GALATEA (galatee) (around neptune)
    [4303025230000.0,-1242172260000.0,-73575112000.0,-7521.58937,8019.3619100000005,4535.01875,2.0e18],
    //  148 HALIMEDE (halimede) (around neptune)
    [4309778630000.0,-1244498830000.0,-88136451000.0,1093.4920100000002,4805.20907,-436.84502200000003,2.0e17],
    //  149 LAOMEDEIA (laomedie) (around neptune)
    [4331910310000.0,-1232993280000.0,-86273590800.0,1427.77976,5556.28056,38.6741626,1.0e17],
    //  150 LARISSA (larissa) (around neptune)
    [4302978450000.0,-1242296820000.0,-73600292400.0,9924.94279,3189.78521,-4311.00071,5.0e18],
    //  151 NAIAD (naiade) (around neptune)
    [4302962330000.0,-1242253450000.0,-73574720800.0,6219.975619999999,-4581.50433,-4900.26045,2.0e17],
    //  152 NEREID (nereide) (around neptune)
    [4307441740000.0,-1239481570000.0,-73146154400.0,1796.8546299999998,6374.67026,-48.135714,3.0e19],
    //  153 NESO (neso) (around neptune)
    [4244548140000.0,-1236149560000.0,-111323033000.0,1412.19412,5457.89296,-273.32036,1.0e17],
    //  154 PROTEUS (protee) (around neptune)
    [4302898380000.0,-1242207820000.0,-73534888100.0,-773.97549,-1843.1241,-1803.49725,5.0e19],
    //  155 PSAMATHE (psamathee) (around neptune)
    [4292985710000.0,-1208871410000.0,-101856546000.0,1763.15577,5242.65532,-412.66560400000003,2.0e16],
    //  156 SAO (sao) (around neptune)
    [4294831220000.0,-1264149600000.0,-78475918000.0,1775.35167,5165.8824,-546.9364519999999,1.0e17],
    //  157 THALASSA (thalassa) (around neptune)
    [4303041580000.0,-1242249200000.0,-73609168800.0,7356.50577,15294.6542,990.395174,4.0e17],
    //  158 TRITON (triton) (around neptune)
    [4302696960000.0,-1242325550000.0,-73433252400.0,1819.0857,8601.26902,2675.7117,2.14e22],
    //  159 PLUTO (pluton)
    [1655540210000.0,-4735026090000.0,27796291900.0,5245.39823,638.52345,-1607.05298,1.303e22],
    //  160 CHARON (charon) (around pluton)
    [1655525960000.0,-4735035760000.0,27805640000.0,5268.97151,773.7946949999999,-1431.23785,1.58e21],
    //  161 HYDRA (hydra) (around pluton)
    [1655528910000.0,-4735000920000.0,27855566100.0,5336.6923799999995,736.4234859999999,-1609.5134,4.8e16],
    //  162 KERBEROS (kerberos) (around pluton)
    [1655499000000.0,-4735046330000.0,27834609100.0,5283.51235,745.229755,-1502.86589,1.64e16],
    //  163 NIX (nix) (around pluton)
    [ 1655570180000.0,-4735014680000.0,27762306300.0,5198.56397,548.333983,-1670.0018,4.5e16],
    //  164 STYX (styx) (around pluton)
    [ 1655550360000.0,-4734997340000.0,27824787300.0,5353.29238,704.859404,-1688.5688,7.5e15],
    //  165 CERES (ceres)
    [-318011280000.0,203620159000.0,65024979300.0,-10049.0725,-16431.148100000002,1333.3465600000002,9.393e20],
    //  166 EROS (eros)
    [ 172133283000.0,-201945835000.0,5266013050.0,14257.0599,13127.2739,3669.1154199999996,7.2e15],
    //  167 KLEOPATRA (kleopatra)
    [ 301319079000.0,95420075700.0,22638507900.0,-7727.88312,20905.241400000003,-5010.25165,3.0e18],
    //  168 LUTETIA (lutetia)
    [-373516483000.0,-132908384000.0,18595647700.0,8038.3889899999995,-15437.5084,-556.283262,1.7e18],
    //  169 PALLAS (pallas)
    [ 11931751300.0,272893936000.0,-188865804000.0,-21829.1296,-565.1004379999999,2224.47486,2.11e20],
    //  170 VESTA (vesta)
    [-140739055000.0,-287794032000.0,25766623000.0,18988.5743,-9083.84002,-2039.22514,2.7e20],
    ]);

    let minute = 60.0;
    let hour = 60.0 * minute;
    let day = 24.0 * hour;
    #[allow(unused_variables)]
    let year = 365.25 * day;

    let t_max = 1.0 * year;
    sim.dt = minute;

    let mut f = File::create("output.xyz")?;

    while sim.t < t_max {
        sim.update();
        //if sim.t % day < 0.1 {
        f.write_all(&sim.bodies[0].position.gpformat())?;
        for i in [1, 2, 3, 5] {
            f.write_all(&sim.bodies[i].position.gpformat())?;
        }
        //}
    }
    Ok(())
}

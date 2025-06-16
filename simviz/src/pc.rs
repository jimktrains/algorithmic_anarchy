use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::Split;

#[derive(Debug, Clone)]
pub struct PhysicalConstants {
    pub spkid: usize,
    pub name: String,
    pub gm: f64,
    pub radius: f32,
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

    pub fn by_spkid(&self, spkid: usize) -> Option<PhysicalConstants> {
        for p in self.v.iter() {
            if p.spkid == spkid {
                return Some(p.clone());
            }
        }
        None
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
    pub fn new_r_in_km(parts: &mut Split<'_, &str>) -> PhysicalConstants {
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
            radius: radius_part
                .parse::<f32>()
                .expect(format!("radius {radius_part} is not a valid f64 for idx {idx}").as_str())
                * 1000.0,
        }
    }

    pub fn from_file_r_in_km<P>(filename: P) -> std::io::Result<PhysicalConstantsVec>
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

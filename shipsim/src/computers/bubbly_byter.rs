use rvemu::cpu;
use rvemu::cpu::Cpu;
use rvemu::csr;
use rvemu::devices::dram::Dram;
use rvemu::exception::Exception;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

use super::spf::SysProvided;
use super::Computer;
use crate::SpaceshipPart;

use crate::ships::everyday_explorer::EverydayExplorer;

const DRAM_BASE: u64 = 0x40000000;
const DRAM_SIZE: u64 = 0x2000;

#[allow(dead_code)]
const MMIO_BASE: u64 = 0x90000000;
#[allow(dead_code)]
const MMIO_SIZE: u64 = 0x100000;

const ROM_BASE: u64 = 0x80000000;
const ROM_SIZE: u64 = 0x100000;

const NVRAM_BASE: u64 = 0xa0000400;
const NVRAM_SIZE: u64 = 0x100000;

pub struct BubblyByter {
    cpu: Cpu,
    #[allow(dead_code)]
    dram: Arc<Mutex<Dram>>,
    #[allow(dead_code)]
    nvram: Arc<Mutex<Dram>>,

    ship: Option<Arc<Mutex<EverydayExplorer>>>,
}

impl SpaceshipPart for BubblyByter {
    fn mass(&self) -> u64 {
        1
    }
}

impl BubblyByter {
    pub fn new() -> BubblyByter {
        let mut cpu = Cpu::new();

        let dram = Arc::new(Mutex::new(Dram::new(DRAM_SIZE)));
        cpu.bus.mount(DRAM_BASE, dram.clone());

        let rom = Arc::new(Mutex::new(Dram::new(ROM_SIZE)));
        cpu.bus.mount(ROM_BASE, rom.clone());

        let nvram = Arc::new(Mutex::new(Dram::new(NVRAM_SIZE)));
        cpu.bus.mount(NVRAM_BASE, nvram.clone());

        let mut b = BubblyByter {
            cpu,
            dram,
            nvram,
            ship: None,
        };
        b.reset();

        b
    }
}

impl fmt::Debug for BubblyByter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BubblyByter").finish()
    }
}

impl Computer for BubblyByter {
    fn reset(&mut self) {
        self.cpu.reset();
        self.cpu.pc = DRAM_BASE;
        self.cpu
            .xregs
            .write(cpu::REG_SP, DRAM_BASE + DRAM_SIZE - 0x400);
    }

    fn load_kernel(&mut self, kernel_img: &str) {
        let mut prog = vec![];
        let mut f = File::open(kernel_img).unwrap();
        f.read_to_end(&mut prog).unwrap();
        self.dram.lock().unwrap().initialize(prog);
    }

    fn execute(&mut self, max_cycle: u64) -> Result<(), Exception> {
        while self.cpu.state.read(csr::TIME) < max_cycle {
            let (_int, _ec) = self.cpu.cycle()?;
        }
        Ok(())
    }

    fn set_ship(&mut self, ship: Arc<Mutex<EverydayExplorer>>) {
        self.ship = Some(ship.clone());
        let jh = SysProvided::new(ship);
        self.cpu.with_jump_link_handler(Box::new(jh));
    }
}

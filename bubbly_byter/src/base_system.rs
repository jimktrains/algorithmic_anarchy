use rvemu::cpu;
use rvemu::cpu::Cpu;
use rvemu::csr;
use rvemu::devices::dram::Dram;
use rvemu::exception::Exception;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::spf::SysProvided;

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
    dram: Arc<Mutex<Dram>>,
    #[allow(dead_code)]
    nvram: Arc<Mutex<Dram>>,
}

impl BubblyByter {
    pub fn new() -> BubblyByter {
        let jh = SysProvided {};
        let mut cpu = Cpu::new();
        cpu.with_jump_link_handler(Box::new(jh));

        let dram = Arc::new(Mutex::new(Dram::new(DRAM_SIZE)));
        cpu.bus.mount(DRAM_BASE, dram.clone());

        let rom = Arc::new(Mutex::new(Dram::new(ROM_SIZE)));
        cpu.bus.mount(ROM_BASE, rom.clone());

        let nvram = Arc::new(Mutex::new(Dram::new(NVRAM_SIZE)));
        cpu.bus.mount(NVRAM_BASE, nvram.clone());

        let mut b = BubblyByter { cpu, dram, nvram };
        b.reset();
        b
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.cpu.pc = DRAM_BASE;
        self.cpu
            .xregs
            .write(cpu::REG_SP, DRAM_BASE + DRAM_SIZE - 0x400);
    }

    pub fn load_kernel<P: AsRef<Path>>(&self, kernel_img: P) {
        let mut prog = vec![];
        let mut f = File::open(kernel_img).unwrap();
        f.read_to_end(&mut prog).unwrap();
        self.dram.lock().unwrap().initialize(prog);
    }

    pub fn execute(&mut self, max_cycle: u64) -> Result<(), Exception> {
        while self.cpu.state.read(csr::TIME) < max_cycle {
            let (_, (_fetch_pc, _fetch_p_pc, _instr)) = self.cpu.cycle()?;
        }
        Ok(())
    }
}

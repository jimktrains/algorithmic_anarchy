use rvemu::cpu;
use rvemu::cpu::Cpu;
use rvemu::csr;
use rvemu::devices::dram::Dram;
use rvemu::exception::Exception;
use std::fs::File;
use std::io::Read;

use bubbly_byter::spf::SysProvided;

const DRAM_BASE: u64 = 0x40000000;
const DRAM_SIZE: u64 = 0x2000;

const MMIO_BASE: u64 = 0x90000000;
const MMIO_SIZE: u64 = 0x100000;

fn main() -> Result<(), Exception> {
    let mut prog = vec![];
    let mut f = File::open("../bubbly_byter_cc/build/kernel.img").unwrap();
    f.read_to_end(&mut prog).unwrap();

    let jh = SysProvided {};
    let mut cpu = Cpu::new();
    cpu.with_jump_link_handler(Box::new(jh));

    let mut dram = Box::new(Dram::new(DRAM_SIZE));
    dram.initialize(prog);
    cpu.bus.mount(DRAM_BASE, dram);

    let dram = Box::new(Dram::new(MMIO_SIZE));
    cpu.bus.mount(MMIO_BASE, dram);

    cpu.reset();
    cpu.pc = DRAM_BASE;
    cpu.xregs.write(cpu::REG_SP, DRAM_BASE + DRAM_SIZE - 0x400);

    loop {
        let (_, (fetch_pc, _fetch_p_pc, instr)) = cpu.cycle()?;
        println!(
            "pc={:x}  instr={:08x}  fa0={}   t={}",
            fetch_pc,
            instr,
            cpu.fregs.read(cpu::REG_FA0),
            cpu.state.read(csr::TIME),
        );
    }
}

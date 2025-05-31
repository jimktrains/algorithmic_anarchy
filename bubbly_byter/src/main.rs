use rvemu::cpu;
use rvemu::cpu::{Cpu, FRegisters, JumpLinkHandler, XRegisters};
use rvemu::csr;
use rvemu::devices::dram::Dram;
use rvemu::exception::Exception;
use std::fs::File;
use std::io::Read;

const SPF_MIN_ADDR: u64 = 0x8000_0000;

const SPF_FCN: u64 = 0x80000400;

// const SPF_SNPRINTF: u64 = 0x80000420;
// const SPF_DEBUGLOG: u64 = 0x80000428;
// const SPF_DEBUGLOGF: u64 = 0x80000430;
// const SPF_FMAX: u64 = 0x80000450;
// const SPF_FMIN: u64 = 0x80000458;
const SPF_EXP: u64 = 0x80000460;
const SPF_EXPM1: u64 = 0x80000468;
const SPF_LOG: u64 = 0x80000470;
const SPF_LOG1P: u64 = 0x80000478;
const SPF_SQRT: u64 = 0x80000480;
const SPF_CBRT: u64 = 0x80000488;
// const SPF_CUBE: u64 = 0x80000490;
// const SPF_HYPOT2: u64 = 0x80000498;
const SPF_CEIL: u64 = 0x800004a0;
const SPF_FLOOR: u64 = 0x800004a8;
const SPF_ROUND: u64 = 0x800004b0;
const SPF_FPOWF: u64 = 0x800004b8;
// const SPF_MODPOW: u64 = 0x800004c0;
// const SPF_DIVMOD: u64 = 0x800004c8;
const SPF_SIN: u64 = 0x800004d0;
const SPF_COS: u64 = 0x800004d8;
const SPF_TAN: u64 = 0x800004e0;
const SPF_ASIN: u64 = 0x800004e8;
const SPF_ACOS: u64 = 0x800004f0;
const SPF_ATAN: u64 = 0x800004f8;
const SPF_ATAN2: u64 = 0x80000500;
const SPF_SINH: u64 = 0x80000508;
const SPF_COSH: u64 = 0x80000510;
const SPF_TANH: u64 = 0x80000518;
const SPF_ASINH: u64 = 0x80000520;
const SPF_ACOSH: u64 = 0x80000528;
const SPF_ATANH: u64 = 0x80000530;
// const SPF_DIST33: u64 = 0x80000538;
// const SPF_ADD33: u64 = 0x80000540;
// const SPF_SUB33: u64 = 0x80000548;
// const SPF_MULS3: u64 = 0x80000550;
// const SPF_DIVS3: u64 = 0x80000558;
// const SPF_DOT33: u64 = 0x80000560;
// const SPF_CROSS33: u64 = 0x80000568;
// const SPF_NORM2: u64 = 0x80000570;
// const SPF_NORMALIZE2: u64 = 0x80000578;
// const SPF_NORM3: u64 = 0x80000580;
// const SPF_NORMALIZE3: u64 = 0x80000588;
// const SPF_LERP33S: u64 = 0x80000590;
// const SPF_VDIST33: u64 = 0x80000598;
// const SPF_VADD33: u64 = 0x800005a0;
// const SPF_VSUB33: u64 = 0x800005a8;
// const SPF_VMULS3: u64 = 0x800005b0;
// const SPF_VDIVS3: u64 = 0x800005b8;
// const SPF_VDOT33: u64 = 0x800005c0;
// const SPF_VCROSS33: u64 = 0x800005c8;
// const SPF_VNORM3: u64 = 0x800005d0;
// const SPF_VNORMALIZE3: u64 = 0x800005d8;
// const SPF_VLERP33S: u64 = 0x800005e0;

const DRAM_BASE: u64 = 0x40000000;
const DRAM_SIZE: u64 = 0x2000;

const MMIO_BASE: u64 = 0x90000000;
const MMIO_SIZE: u64 = 0x100000;

struct SysProvided {}

impl JumpLinkHandler for SysProvided {
    fn should_handle(&self, new_pc: u64) -> bool {
        new_pc >= SPF_MIN_ADDR
    }
    fn handle(&self, new_pc: u64, cpu: &Cpu) -> (XRegisters, FRegisters) {
        let xregs = cpu.xregs.clone();
        let mut fregs = cpu.fregs.clone();
        match new_pc {
            SPF_EXP => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).exp()),
            SPF_EXPM1 => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).exp_m1()),
            SPF_LOG => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).ln()),
            SPF_LOG1P => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).ln_1p()),
            SPF_SQRT => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).sqrt()),
            SPF_CBRT => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).cbrt()),
            SPF_CEIL => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).ceil()),
            SPF_FLOOR => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).floor()),
            SPF_ROUND => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).round()),
            SPF_FPOWF => fregs.write(
                cpu::REG_FA0,
                fregs.read(cpu::REG_FA0).powf(fregs.read(cpu::REG_FA1)),
            ),
            SPF_SIN => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).sin()),
            SPF_COS => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).cos()),
            SPF_TAN => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).tan()),
            SPF_ASIN => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).asin()),
            SPF_ACOS => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).acos()),
            SPF_ATAN => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).atan()),
            SPF_ATAN2 => fregs.write(
                cpu::REG_FA0,
                fregs.read(cpu::REG_FA0).atan2(fregs.read(cpu::REG_FA1)),
            ),
            SPF_SINH => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).sinh()),
            SPF_COSH => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).cosh()),
            SPF_TANH => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).tanh()),
            SPF_ASINH => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).asinh()),
            SPF_ACOSH => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).acosh()),
            SPF_ATANH => fregs.write(cpu::REG_FA0, fregs.read(cpu::REG_FA0).atanh()),
            SPF_FCN => {
                fregs.write(cpu::REG_FA1, 654.321);
            }
            _ => {}
        }

        (xregs, fregs)
    }
}

fn main() -> Result<(), Exception> {
    // #[rustfmt::skip]
    // let prog: Vec<u8> = vec![
    //     //  80000000: f2000553                fmv.d.x fa0,zero
    //     0x53, 0x05, 0x00, 0xf2,
    //     //  80000004: 3fc000ef                jal     80000400 <fcn>
    //     0xef, 0x00, 0xc0, 0x3f,
    // ];

    let mut prog = vec![];
    let mut f = File::open(
        "/home/jim/projects/other/stupid-ideas/programminggame/bubbly_byter_cc/build/kernel.img",
    )
    .unwrap();
    f.read_to_end(&mut prog).unwrap();

    let jh = SysProvided {};
    let mut cpu = Cpu::new();
    cpu.with_jump_link_handler(Box::new(jh));
    println!("Making ram of size {}", DRAM_SIZE);
    let mut dram = Box::new(Dram::new(DRAM_SIZE));
    dram.initialize(prog);
    cpu.bus.mount(DRAM_BASE, dram);

    let dram = Box::new(Dram::new(MMIO_SIZE));
    cpu.bus.mount(MMIO_BASE, dram);

    cpu.reset();
    cpu.pc = DRAM_BASE;
    let stack_start = DRAM_BASE + DRAM_SIZE - 0x400;
    cpu.xregs.write(cpu::REG_SP, DRAM_BASE + DRAM_SIZE - 0x400);

    //cpu.print_registers();
    loop {
        let (_, (fetch_pc, fetch_p_pc, instr)) = cpu.cycle()?;
        //cpu.print_registers();
        // println!("sp={:x}", cpu.xregs.read(cpu::REG_SP));
        // for i in (cpu.xregs.read(cpu::REG_SP)..=stack_start).step_by(8) {
        //     println!(
        //         "{:08x}:{:08x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
        //         i,
        //         i + 7,
        //         cpu.bus.read(i, cpu::BYTE)?,
        //         cpu.bus.read(i + 1, cpu::BYTE)?,
        //         cpu.bus.read(i + 2, cpu::BYTE)?,
        //         cpu.bus.read(i + 3, cpu::BYTE)?,
        //         cpu.bus.read(i + 4, cpu::BYTE)?,
        //         cpu.bus.read(i + 5, cpu::BYTE)?,
        //         cpu.bus.read(i + 6, cpu::BYTE)?,
        //         cpu.bus.read(i + 7, cpu::BYTE)?,
        //     )
        // }
        println!(
            "pc={:x}  instr={:08x}  fa0={}   t={}",
            fetch_pc,
            instr,
            cpu.fregs.read(cpu::REG_FA0),
            cpu.state.read(csr::TIME),
        );
    }
}

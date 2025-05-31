// System Provided Function

use rvemu::cpu;
use rvemu::cpu::{Cpu, FRegisters, JumpLinkHandler, XRegisters};

pub const SPF_MIN_ADDR: u64 = 0x8000_0000;

pub const SPF_FCN: u64 = 0x80000400;

// pub const SPF_SNPRINTF: u64 = 0x80000420;
// pub const SPF_DEBUGLOG: u64 = 0x80000428;
// pub const SPF_DEBUGLOGF: u64 = 0x80000430;
// pub const SPF_FMAX: u64 = 0x80000450;
// pub const SPF_FMIN: u64 = 0x80000458;
pub const SPF_EXP: u64 = 0x80000460;
pub const SPF_EXPM1: u64 = 0x80000468;
pub const SPF_LOG: u64 = 0x80000470;
pub const SPF_LOG1P: u64 = 0x80000478;
pub const SPF_SQRT: u64 = 0x80000480;
pub const SPF_CBRT: u64 = 0x80000488;
// pub const SPF_CUBE: u64 = 0x80000490;
// pub const SPF_HYPOT2: u64 = 0x80000498;
pub const SPF_CEIL: u64 = 0x800004a0;
pub const SPF_FLOOR: u64 = 0x800004a8;
pub const SPF_ROUND: u64 = 0x800004b0;
pub const SPF_FPOWF: u64 = 0x800004b8;
// pub const SPF_MODPOW: u64 = 0x800004c0;
// pub const SPF_DIVMOD: u64 = 0x800004c8;
pub const SPF_SIN: u64 = 0x800004d0;
pub const SPF_COS: u64 = 0x800004d8;
pub const SPF_TAN: u64 = 0x800004e0;
pub const SPF_ASIN: u64 = 0x800004e8;
pub const SPF_ACOS: u64 = 0x800004f0;
pub const SPF_ATAN: u64 = 0x800004f8;
pub const SPF_ATAN2: u64 = 0x80000500;
pub const SPF_SINH: u64 = 0x80000508;
pub const SPF_COSH: u64 = 0x80000510;
pub const SPF_TANH: u64 = 0x80000518;
pub const SPF_ASINH: u64 = 0x80000520;
pub const SPF_ACOSH: u64 = 0x80000528;
pub const SPF_ATANH: u64 = 0x80000530;
// pub const SPF_DIST33: u64 = 0x80000538;
// pub const SPF_ADD33: u64 = 0x80000540;
// pub const SPF_SUB33: u64 = 0x80000548;
// pub const SPF_MULS3: u64 = 0x80000550;
// pub const SPF_DIVS3: u64 = 0x80000558;
// pub const SPF_DOT33: u64 = 0x80000560;
// pub const SPF_CROSS33: u64 = 0x80000568;
// pub const SPF_NORM2: u64 = 0x80000570;
// pub const SPF_NORMALIZE2: u64 = 0x80000578;
// pub const SPF_NORM3: u64 = 0x80000580;
// pub const SPF_NORMALIZE3: u64 = 0x80000588;
// pub const SPF_LERP33S: u64 = 0x80000590;
// pub const SPF_VDIST33: u64 = 0x80000598;
// pub const SPF_VADD33: u64 = 0x800005a0;
// pub const SPF_VSUB33: u64 = 0x800005a8;
// pub const SPF_VMULS3: u64 = 0x800005b0;
// pub const SPF_VDIVS3: u64 = 0x800005b8;
// pub const SPF_VDOT33: u64 = 0x800005c0;
// pub const SPF_VCROSS33: u64 = 0x800005c8;
// pub const SPF_VNORM3: u64 = 0x800005d0;
// pub const SPF_VNORMALIZE3: u64 = 0x800005d8;
// pub const SPF_VLERP33S: u64 = 0x800005e0;

pub struct SysProvided {}

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

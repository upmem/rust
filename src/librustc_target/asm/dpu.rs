use super::{InlineAsmArch, InlineAsmType};
use rustc_macros::HashStable_Generic;
use std::fmt;

def_reg_class! {
    DPU DpuInlineAsmRegClass {
        reg32,
        reg64,
    }
}

impl DpuInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<(char, &'static str)> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<(char, &'static str)> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<&'static str>)] {
        match self {
            Self::reg32 => types! { _: I8, I16, I32; },
            Self::reg64 => types! { _: I8, I16, I32, I64; },
        }
    }
}

def_regs! {
    DPU DpuInlineAsmReg DpuInlineAsmRegClass {
        r0: reg32 = ["r0"],
        r1: reg32 = ["r1"],
        r2: reg32 = ["r2"],
        r3: reg32 = ["r3"],
        r4: reg32 = ["r4"],
        r5: reg32 = ["r5"],
        r6: reg32 = ["r6"],
        r7: reg32 = ["r7"],
        r8: reg32 = ["r8"],
        r9: reg32 = ["r9"],
        r10: reg32 = ["r10"],
        r11: reg32 = ["r11"],
        r12: reg32 = ["r12"],
        r13: reg32 = ["r13"],
        r14: reg32 = ["r14"],
        r15: reg32 = ["r15"],
        r16: reg32 = ["r16"],
        r17: reg32 = ["r17"],
        r18: reg32 = ["r18"],
        r19: reg32 = ["r19"],
        r20: reg32 = ["r20"],
        r21: reg32 = ["r21"],
        r22: reg32 = ["r22"],
        r23: reg32 = ["r23"],
        d0: reg64 = ["d0"],
        d1: reg64 = ["d1"],
        d2: reg64 = ["d2"],
        d3: reg64 = ["d3"],
        d4: reg64 = ["d4"],
        d5: reg64 = ["d5"],
        d6: reg64 = ["d6"],
        d7: reg64 = ["d7"],
        d8: reg64 = ["d8"],
        d9: reg64 = ["d9"],
        d10: reg64 = ["d10"],
        d11: reg64 = ["d11"],
        d12: reg64 = ["d12"],
        d13: reg64 = ["d13"],
        d14: reg64 = ["d14"],
        d15: reg64 = ["d15"],
        d16: reg64 = ["d16"],
        d17: reg64 = ["d17"],
        d18: reg64 = ["d18"],
        d19: reg64 = ["d19"],
        d20: reg64 = ["d20"],
        d21: reg64 = ["d21"],
        d22: reg64 = ["d22"],
        d23: reg64 = ["d23"],
    }
}

impl DpuInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }

    pub fn overlapping_regs(self, mut cb: impl FnMut(DpuInlineAsmReg)) {
        cb(self);

        macro_rules! reg_conflicts {
            (
                $(
                    $d_high:ident : $r0_high:ident $r1_high:ident
                ),*;
            ) => {
                match self {
                    $(
                        Self::$d_high => {
                            cb(Self::$r0_high);
                            cb(Self::$r1_high);
                        }
                        Self::$r0_high | Self::$r1_high => {
                            cb(Self::$d_high);
                        }
                    )*
                    _ => {},
                }
            };
        }

        reg_conflicts! {
            d0 : r0 r1,
            d2 : r2 r3,
            d4 : r4 r5,
            d6 : r6 r7,
            d8 : r8 r9,
            d10 : r10 r11,
            d12 : r12 r13,
            d14 : r14 r15,
            d16 : r16 r17,
            d18 : r18 r19,
            d20 : r20 r21,
            d22 : r22 r23;
        }
    }
}

bitflags! {
    struct Aj55Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Aj55Reg1 {
    const REG: u32 = FIOPAD_AJ55_REG1_OFFSET;
}

impl From<u32> for Aj55Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Aj55Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Aj55Reg1 {

}

bitflags! {
    struct Al55Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Al55Reg1 {
    const REG: u32 = FIOPAD_AL55_REG1_OFFSET;
}

impl From<u32> for Al55Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Al55Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Al55Reg1 {

}

bitflags! {
    struct Al53Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Al53Reg1 {
    const REG: u32 = FIOPAD_AL53_REG1_OFFSET;
}

impl From<u32> for Al53Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Al53Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Al53Reg1 {

}

bitflags! {
    struct An51Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for An51Reg1 {
    const REG: u32 = FIOPAD_AN51_REG1_OFFSET;
}

impl From<u32> for An51Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for An51Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for An51Reg1 {

}

bitflags! {
    struct Ar51Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ar51Reg1 {
    const REG: u32 = FIOPAD_AR51_REG1_OFFSET;
}

impl From<u32> for Ar51Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ar51Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ar51Reg1 {

}

bitflags! {
    struct Aj57Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Aj57Reg1 {
    const REG: u32 = FIOPAD_AJ57_REG1_OFFSET;
}

impl From<u32> for Aj57Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Aj57Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Aj57Reg1 {

}

bitflags! {
    struct Ag59Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ag59Reg1 {
    const REG: u32 = FIOPAD_AG59_REG1_OFFSET;
}

impl From<u32> for Ag59Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ag59Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ag59Reg1 {

}

bitflags! {
    struct Ag57Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ag57Reg1 {
    const REG: u32 = FIOPAD_AG57_REG1_OFFSET;
}

impl From<u32> for Ag57Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ag57Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ag57Reg1 {

}

bitflags! {
    struct Ae59Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ae59Reg1 {
    const REG: u32 = FIOPAD_AE59_REG1_OFFSET;
}

impl From<u32> for Ae59Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ae59Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ae59Reg1 {

}

bitflags! {
    struct Ba55Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ba55Reg1 {
    const REG: u32 = FIOPAD_BA55_REG1_OFFSET;
}

impl From<u32> for Ba55Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ba55Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ba55Reg1 {

}

bitflags! {
    struct Ba53Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ba53Reg1 {
    const REG: u32 = FIOPAD_BA53_REG1_OFFSET;
}

impl From<u32> for Ba53Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ba53Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ba53Reg1 {

}

bitflags! {
    struct Ar59Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ar59Reg1 {
    const REG: u32 = FIOPAD_AR59_REG1_OFFSET;
}

impl From<u32> for Ar59Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ar59Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ar59Reg1 {

}

bitflags! {
    struct Au59Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Au59Reg1 {
    const REG: u32 = FIOPAD_AU59_REG1_OFFSET;
}

impl From<u32> for Au59Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Au59Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Au59Reg1 {

}

bitflags! {
    struct A45Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for A45Reg1 {
    const REG: u32 = FIOPAD_A45_REG1_OFFSET;
}

impl From<u32> for A45Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for A45Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for A45Reg1 {

}

bitflags! {
    struct C45Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for C45Reg1 {
    const REG: u32 = FIOPAD_C45_REG1_OFFSET;
}

impl From<u32> for C45Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for C45Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for C45Reg1 {

}

bitflags! {
    struct A47Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for A47Reg1 {
    const REG: u32 = FIOPAD_A47_REG1_OFFSET;
}

impl From<u32> for A47Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for A47Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for A47Reg1 {

}

bitflags! {
    struct A49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for A49Reg1 {
    const REG: u32 = FIOPAD_A49_REG1_OFFSET;
}

impl From<u32> for A49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for A49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for A49Reg1 {

}

bitflags! {
    struct C49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for C49Reg1 {
    const REG: u32 = FIOPAD_C49_REG1_OFFSET;
}

impl From<u32> for C49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for C49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for C49Reg1 {

}

bitflags! {
    struct A51Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for A51Reg1 {
    const REG: u32 = FIOPAD_A51_REG1_OFFSET;
}

impl From<u32> for A51Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for A51Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for A51Reg1 {

}

bitflags! {
    struct A33Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for A33Reg1 {
    const REG: u32 = FIOPAD_A33_REG1_OFFSET;
}

impl From<u32> for A33Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for A33Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for A33Reg1 {

}

bitflags! {
    struct C33Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for C33Reg1 {
    const REG: u32 = FIOPAD_C33_REG1_OFFSET;
}

impl From<u32> for C33Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for C33Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for C33Reg1 {

}

bitflags! {
    struct C31Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for C31Reg1 {
    const REG: u32 = FIOPAD_C31_REG1_OFFSET;
}

impl From<u32> for C31Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for C31Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for C31Reg1 {

}

bitflags! {
    struct A31Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for A31Reg1 {
    const REG: u32 = FIOPAD_A31_REG1_OFFSET;
}

impl From<u32> for A31Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for A31Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for A31Reg1 {

}

bitflags! {
    struct Aj53Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Aj53Reg1 {
    const REG: u32 = FIOPAD_AJ53_REG1_OFFSET;
}

impl From<u32> for Aj53Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Aj53Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Aj53Reg1 {

}

bitflags! {
    struct Al49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Al49Reg1 {
    const REG: u32 = FIOPAD_AL49_REG1_OFFSET;
}

impl From<u32> for Al49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Al49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Al49Reg1 {

}

bitflags! {
    struct Al47Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Al47Reg1 {
    const REG: u32 = FIOPAD_AL47_REG1_OFFSET;
}

impl From<u32> for Al47Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Al47Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Al47Reg1 {

}

bitflags! {
    struct An49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for An49Reg1 {
    const REG: u32 = FIOPAD_AN49_REG1_OFFSET;
}

impl From<u32> for An49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for An49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for An49Reg1 {

}

bitflags! {
    struct Ag51Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ag51Reg1 {
    const REG: u32 = FIOPAD_AG51_REG1_OFFSET;
}

impl From<u32> for Ag51Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ag51Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ag51Reg1 {

}

bitflags! {
    struct Aj51Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Aj51Reg1 {
    const REG: u32 = FIOPAD_AJ51_REG1_OFFSET;
}

impl From<u32> for Aj51Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Aj51Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Aj51Reg1 {

}

bitflags! {
    struct Ag49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ag49Reg1 {
    const REG: u32 = FIOPAD_AG49_REG1_OFFSET;
}

impl From<u32> for Ag49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ag49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ag49Reg1 {

}

bitflags! {
    struct Ae55Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ae55Reg1 {
    const REG: u32 = FIOPAD_AE55_REG1_OFFSET;
}

impl From<u32> for Ae55Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ae55Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ae55Reg1 {

}

bitflags! {
    struct Ae53Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ae53Reg1 {
    const REG: u32 = FIOPAD_AE53_REG1_OFFSET;
}

impl From<u32> for Ae53Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ae53Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ae53Reg1 {

}

bitflags! {
    struct Ag55Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ag55Reg1 {
    const REG: u32 = FIOPAD_AG55_REG1_OFFSET;
}

impl From<u32> for Ag55Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ag55Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ag55Reg1 {

}

bitflags! {
    struct Aj49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Aj49Reg1 {
    const REG: u32 = FIOPAD_AJ49_REG1_OFFSET;
}

impl From<u32> for Aj49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Aj49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Aj49Reg1 {

}

bitflags! {
    struct Ac55Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ac55Reg1 {
    const REG: u32 = FIOPAD_AC55_REG1_OFFSET;
}

impl From<u32> for Ac55Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ac55Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ac55Reg1 {

}

bitflags! {
    struct Ac53Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ac53Reg1 {
    const REG: u32 = FIOPAD_AC53_REG1_OFFSET;
}

impl From<u32> for Ac53Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ac53Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ac53Reg1 {

}

bitflags! {
    struct Ae51Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ae51Reg1 {
    const REG: u32 = FIOPAD_AE51_REG1_OFFSET;
}

impl From<u32> for Ae51Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ae51Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ae51Reg1 {

}

bitflags! {
    struct W51Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for W51Reg1 {
    const REG: u32 = FIOPAD_W51_REG1_OFFSET;
}

impl From<u32> for W51Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for W51Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for W51Reg1 {

}

bitflags! {
    struct W53Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for W53Reg1 {
    const REG: u32 = FIOPAD_W53_REG1_OFFSET;
}

impl From<u32> for W53Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for W53Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for W53Reg1 {

}

bitflags! {
    struct U55Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for U55Reg1 {
    const REG: u32 = FIOPAD_U55_REG1_OFFSET;
}

impl From<u32> for U55Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for U55Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for U55Reg1 {

}

bitflags! {
    struct U53Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for U53Reg1 {
    const REG: u32 = FIOPAD_U53_REG1_OFFSET;
}

impl From<u32> for U53Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for U53Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for U53Reg1 {

}

bitflags! {
    struct Ae49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ae49Reg1 {
    const REG: u32 = FIOPAD_AE49_REG1_OFFSET;
}

impl From<u32> for Ae49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ae49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ae49Reg1 {

}

bitflags! {
    struct Ac49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ac49Reg1 {
    const REG: u32 = FIOPAD_AC49_REG1_OFFSET;
}

impl From<u32> for Ac49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ac49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ac49Reg1 {

}

bitflags! {
    struct Ae47Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Ae47Reg1 {
    const REG: u32 = FIOPAD_AE47_REG1_OFFSET;
}

impl From<u32> for Ae47Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Ae47Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Ae47Reg1 {

}

bitflags! {
    struct Aa47Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Aa47Reg1 {
    const REG: u32 = FIOPAD_AA47_REG1_OFFSET;
}

impl From<u32> for Aa47Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Aa47Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Aa47Reg1 {

}

bitflags! {
    struct Aa49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Aa49Reg1 {
    const REG: u32 = FIOPAD_AA49_REG1_OFFSET;
}

impl From<u32> for Aa49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Aa49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Aa49Reg1 {

}

bitflags! {
    struct W49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for W49Reg1 {
    const REG: u32 = FIOPAD_W49_REG1_OFFSET;
}

impl From<u32> for W49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for W49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for W49Reg1 {

}

bitflags! {
    struct Aa51Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for Aa51Reg1 {
    const REG: u32 = FIOPAD_AA51_REG1_OFFSET;
}

impl From<u32> for Aa51Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for Aa51Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for Aa51Reg1 {

}

bitflags! {
    struct U49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for U49Reg1 {
    const REG: u32 = FIOPAD_U49_REG1_OFFSET;
}

impl From<u32> for U49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for U49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for U49Reg1 {

}

bitflags! {
    struct J59Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J59Reg1 {
    const REG: u32 = FIOPAD_J59_REG1_OFFSET;
}

impl From<u32> for J59Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J59Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J59Reg1 {

}

bitflags! {
    struct L57Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L57Reg1 {
    const REG: u32 = FIOPAD_L57_REG1_OFFSET;
}

impl From<u32> for L57Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L57Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L57Reg1 {

}

bitflags! {
    struct C59Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for C59Reg1 {
    const REG: u32 = FIOPAD_C59_REG1_OFFSET;
}

impl From<u32> for C59Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for C59Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for C59Reg1 {

}

bitflags! {
    struct E59Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for E59Reg1 {
    const REG: u32 = FIOPAD_E59_REG1_OFFSET;
}

impl From<u32> for E59Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for E59Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for E59Reg1 {

}

bitflags! {
    struct J57Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J57Reg1 {
    const REG: u32 = FIOPAD_J57_REG1_OFFSET;
}

impl From<u32> for J57Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J57Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J57Reg1 {

}

bitflags! {
    struct L59Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L59Reg1 {
    const REG: u32 = FIOPAD_L59_REG1_OFFSET;
}

impl From<u32> for L59Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L59Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L59Reg1 {

}

bitflags! {
    struct N59Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for N59Reg1 {
    const REG: u32 = FIOPAD_N59_REG1_OFFSET;
}

impl From<u32> for N59Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for N59Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for N59Reg1 {

}

bitflags! {
    struct E31Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for E31Reg1 {
    const REG: u32 = FIOPAD_E31_REG1_OFFSET;
}

impl From<u32> for E31Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for E31Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for E31Reg1 {

}

bitflags! {
    struct G31Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for G31Reg1 {
    const REG: u32 = FIOPAD_G31_REG1_OFFSET;
}

impl From<u32> for G31Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for G31Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for G31Reg1 {

}

bitflags! {
    struct N41Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for N41Reg1 {
    const REG: u32 = FIOPAD_N41_REG1_OFFSET;
}

impl From<u32> for N41Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for N41Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for N41Reg1 {

}

bitflags! {
    struct N39Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for N39Reg1 {
    const REG: u32 = FIOPAD_N39_REG1_OFFSET;
}

impl From<u32> for N39Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for N39Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for N39Reg1 {

}

bitflags! {
    struct J33Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J33Reg1 {
    const REG: u32 = FIOPAD_J33_REG1_OFFSET;
}

impl From<u32> for J33Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J33Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J33Reg1 {

}

bitflags! {
    struct N33Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for N33Reg1 {
    const REG: u32 = FIOPAD_N33_REG1_OFFSET;
}

impl From<u32> for N33Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for N33Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for N33Reg1 {

}

bitflags! {
    struct L33Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L33Reg1 {
    const REG: u32 = FIOPAD_L33_REG1_OFFSET;
}

impl From<u32> for L33Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L33Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L33Reg1 {

}

bitflags! {
    struct N45Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for N45Reg1 {
    const REG: u32 = FIOPAD_N45_REG1_OFFSET;
}

impl From<u32> for N45Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for N45Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for N45Reg1 {

}

bitflags! {
    struct N43Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for N43Reg1 {
    const REG: u32 = FIOPAD_N43_REG1_OFFSET;
}

impl From<u32> for N43Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for N43Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for N43Reg1 {

}

bitflags! {
    struct L31Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L31Reg1 {
    const REG: u32 = FIOPAD_L31_REG1_OFFSET;
}

impl From<u32> for L31Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L31Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L31Reg1 {

}

bitflags! {
    struct J31Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J31Reg1 {
    const REG: u32 = FIOPAD_J31_REG1_OFFSET;
}

impl From<u32> for J31Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J31Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J31Reg1 {

}

bitflags! {
    struct J29Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J29Reg1 {
    const REG: u32 = FIOPAD_J29_REG1_OFFSET;
}

impl From<u32> for J29Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J29Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J29Reg1 {

}

bitflags! {
    struct E29Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for E29Reg1 {
    const REG: u32 = FIOPAD_E29_REG1_OFFSET;
}

impl From<u32> for E29Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for E29Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for E29Reg1 {

}

bitflags! {
    struct G29Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for G29Reg1 {
    const REG: u32 = FIOPAD_G29_REG1_OFFSET;
}

impl From<u32> for G29Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for G29Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for G29Reg1 {

}

bitflags! {
    struct J37Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J37Reg1 {
    const REG: u32 = FIOPAD_J37_REG1_OFFSET;
}

impl From<u32> for J37Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J37Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J37Reg1 {

}

bitflags! {
    struct J39Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J39Reg1 {
    const REG: u32 = FIOPAD_J39_REG1_OFFSET;
}

impl From<u32> for J39Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J39Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J39Reg1 {

}

bitflags! {
    struct G41Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for G41Reg1 {
    const REG: u32 = FIOPAD_G41_REG1_OFFSET;
}

impl From<u32> for G41Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for G41Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for G41Reg1 {

}

bitflags! {
    struct E43Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for E43Reg1 {
    const REG: u32 = FIOPAD_E43_REG1_OFFSET;
}

impl From<u32> for E43Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for E43Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for E43Reg1 {

}

bitflags! {
    struct L43Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L43Reg1 {
    const REG: u32 = FIOPAD_L43_REG1_OFFSET;
}

impl From<u32> for L43Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L43Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L43Reg1 {

}

bitflags! {
    struct C43Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for C43Reg1 {
    const REG: u32 = FIOPAD_C43_REG1_OFFSET;
}

impl From<u32> for C43Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for C43Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for C43Reg1 {

}

bitflags! {
    struct E41Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for E41Reg1 {
    const REG: u32 = FIOPAD_E41_REG1_OFFSET;
}

impl From<u32> for E41Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for E41Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for E41Reg1 {

}

bitflags! {
    struct L45Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L45Reg1 {
    const REG: u32 = FIOPAD_L45_REG1_OFFSET;
}

impl From<u32> for L45Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L45Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L45Reg1 {

}

bitflags! {
    struct J43Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J43Reg1 {
    const REG: u32 = FIOPAD_J43_REG1_OFFSET;
}

impl From<u32> for J43Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J43Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J43Reg1 {

}

bitflags! {
    struct J41Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J41Reg1 {
    const REG: u32 = FIOPAD_J41_REG1_OFFSET;
}

impl From<u32> for J41Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J41Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J41Reg1 {

}

bitflags! {
    struct L39Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L39Reg1 {
    const REG: u32 = FIOPAD_L39_REG1_OFFSET;
}

impl From<u32> for L39Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L39Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L39Reg1 {

}

bitflags! {
    struct E37Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for E37Reg1 {
    const REG: u32 = FIOPAD_E37_REG1_OFFSET;
}

impl From<u32> for E37Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for E37Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for E37Reg1 {

}

bitflags! {
    struct E35Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for E35Reg1 {
    const REG: u32 = FIOPAD_E35_REG1_OFFSET;
}

impl From<u32> for E35Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for E35Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for E35Reg1 {

}

bitflags! {
    struct G35Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for G35Reg1 {
    const REG: u32 = FIOPAD_G35_REG1_OFFSET;
}

impl From<u32> for G35Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for G35Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for G35Reg1 {

}

bitflags! {
    struct L55Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L55Reg1 {
    const REG: u32 = FIOPAD_L55_REG1_OFFSET;
}

impl From<u32> for L55Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L55Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L55Reg1 {

}

bitflags! {
    struct J55Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J55Reg1 {
    const REG: u32 = FIOPAD_J55_REG1_OFFSET;
}

impl From<u32> for J55Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J55Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J55Reg1 {

}

bitflags! {
    struct J45Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J45Reg1 {
    const REG: u32 = FIOPAD_J45_REG1_OFFSET;
}

impl From<u32> for J45Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J45Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J45Reg1 {

}

bitflags! {
    struct E47Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for E47Reg1 {
    const REG: u32 = FIOPAD_E47_REG1_OFFSET;
}

impl From<u32> for E47Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for E47Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for E47Reg1 {

}

bitflags! {
    struct G47Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for G47Reg1 {
    const REG: u32 = FIOPAD_G47_REG1_OFFSET;
}

impl From<u32> for G47Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for G47Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for G47Reg1 {

}

bitflags! {
    struct J47Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J47Reg1 {
    const REG: u32 = FIOPAD_J47_REG1_OFFSET;
}

impl From<u32> for J47Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J47Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J47Reg1 {

}

bitflags! {
    struct J49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J49Reg1 {
    const REG: u32 = FIOPAD_J49_REG1_OFFSET;
}

impl From<u32> for J49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J49Reg1 {

}

bitflags! {
    struct N49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for N49Reg1 {
    const REG: u32 = FIOPAD_N49_REG1_OFFSET;
}

impl From<u32> for N49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for N49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for N49Reg1 {

}

bitflags! {
    struct L51Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L51Reg1 {
    const REG: u32 = FIOPAD_L51_REG1_OFFSET;
}

impl From<u32> for L51Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L51Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L51Reg1 {

}

bitflags! {
    struct L49Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for L49Reg1 {
    const REG: u32 = FIOPAD_L49_REG1_OFFSET;
}

impl From<u32> for L49Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for L49Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for L49Reg1 {

}

bitflags! {
    struct N53Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for N53Reg1 {
    const REG: u32 = FIOPAD_N53_REG1_OFFSET;
}

impl From<u32> for N53Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for N53Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for N53Reg1 {

}

bitflags! {
    struct J53Reg1: u32 {
        const OUT_DELAY_EN = 1 << 8;
        const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
        const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
        const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
        const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
        const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
        const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
        const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
        const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
        const IN_DELAY_EN = 1 << 0;
        const IN_DELAY_DELICATE_MASK = genmask!(3,1);
        const IN_DELAY_DELICATE_BIT0 = 1 << 1;
        const IN_DELAY_DELICATE_BIT1 = 1 << 2;
        const IN_DELAY_DELICATE_BIT2 = 1 << 3;
        const IN_DELAY_ROUGH_MASK = genmask!(6,4);
        const IN_DELAY_ROUGH_BIT0 = 1 << 4;
        const IN_DELAY_ROUGH_BIT1 = 1 << 5;
        const IN_DELAY_ROUGH_BIT2 = 1 << 6;
    }
}

impl FlagReg for J53Reg1 {
    const REG: u32 = FIOPAD_J53_REG1_OFFSET;
}

impl From<u32> for J53Reg1 {
    fn from(x: u32) -> Self {
        Self::from_bits_truncate(x)
    }
}

impl Into<u32> for J53Reg1 {
    fn into(self) -> u32 {
        self.bits()
    }
}

impl XReg1 for J53Reg1 {

}


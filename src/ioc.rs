#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pa0_sel: Pa0Sel,
    pa1_sel: Pa1Sel,
    pa2_sel: Pa2Sel,
    pa3_sel: Pa3Sel,
    pa4_sel: Pa4Sel,
    pa5_sel: Pa5Sel,
    pa6_sel: Pa6Sel,
    pa7_sel: Pa7Sel,
    pb0_sel: Pb0Sel,
    pb1_sel: Pb1Sel,
    pb2_sel: Pb2Sel,
    pb3_sel: Pb3Sel,
    pb4_sel: Pb4Sel,
    pb5_sel: Pb5Sel,
    pb6_sel: Pb6Sel,
    pb7_sel: Pb7Sel,
    pc0_sel: Pc0Sel,
    pc1_sel: Pc1Sel,
    pc2_sel: Pc2Sel,
    pc3_sel: Pc3Sel,
    pc4_sel: Pc4Sel,
    pc5_sel: Pc5Sel,
    pc6_sel: Pc6Sel,
    pc7_sel: Pc7Sel,
    pd0_sel: Pd0Sel,
    pd1_sel: Pd1Sel,
    pd2_sel: Pd2Sel,
    pd3_sel: Pd3Sel,
    pd4_sel: Pd4Sel,
    pd5_sel: Pd5Sel,
    pd6_sel: Pd6Sel,
    pd7_sel: Pd7Sel,
    pa0_over: Pa0Over,
    pa1_over: Pa1Over,
    pa2_over: Pa2Over,
    pa3_over: Pa3Over,
    pa4_over: Pa4Over,
    pa5_over: Pa5Over,
    pa6_over: Pa6Over,
    pa7_over: Pa7Over,
    pb0_over: Pb0Over,
    pb1_over: Pb1Over,
    pb2_over: Pb2Over,
    pb3_over: Pb3Over,
    pb4_over: Pb4Over,
    pb5_over: Pb5Over,
    pb6_over: Pb6Over,
    pb7_over: Pb7Over,
    pc0_over: Pc0Over,
    pc1_over: Pc1Over,
    pc2_over: Pc2Over,
    pc3_over: Pc3Over,
    pc4_over: Pc4Over,
    pc5_over: Pc5Over,
    pc6_over: Pc6Over,
    pc7_over: Pc7Over,
    pd0_over: Pd0Over,
    pd1_over: Pd1Over,
    pd2_over: Pd2Over,
    pd3_over: Pd3Over,
    pd4_over: Pd4Over,
    pd5_over: Pd5Over,
    pd6_over: Pd6Over,
    pd7_over: Pd7Over,
    uartrxd_uart0: UartrxdUart0,
    uartcts_uart1: UartctsUart1,
    uartrxd_uart1: UartrxdUart1,
    clk_ssi_ssi0: ClkSsiSsi0,
    ssirxd_ssi0: SsirxdSsi0,
    ssifssin_ssi0: SsifssinSsi0,
    clk_ssiin_ssi0: ClkSsiinSsi0,
    clk_ssi_ssi1: ClkSsiSsi1,
    ssirxd_ssi1: SsirxdSsi1,
    ssifssin_ssi1: SsifssinSsi1,
    clk_ssiin_ssi1: ClkSsiinSsi1,
    i2cmssda: I2cmssda,
    i2cmsscl: I2cmsscl,
    gpt0ocp1: Gpt0ocp1,
    gpt0ocp2: Gpt0ocp2,
    gpt1ocp1: Gpt1ocp1,
    gpt1ocp2: Gpt1ocp2,
    gpt2ocp1: Gpt2ocp1,
    gpt2ocp2: Gpt2ocp2,
    gpt3ocp1: Gpt3ocp1,
    gpt3ocp2: Gpt3ocp2,
}
impl RegisterBlock {
    #[doc = "0x00 - Peripheral select control for PA0"]
    #[inline(always)]
    pub const fn pa0_sel(&self) -> &Pa0Sel {
        &self.pa0_sel
    }
    #[doc = "0x04 - Peripheral select control for PA1"]
    #[inline(always)]
    pub const fn pa1_sel(&self) -> &Pa1Sel {
        &self.pa1_sel
    }
    #[doc = "0x08 - Peripheral select control for PA2"]
    #[inline(always)]
    pub const fn pa2_sel(&self) -> &Pa2Sel {
        &self.pa2_sel
    }
    #[doc = "0x0c - Peripheral select control for PA3"]
    #[inline(always)]
    pub const fn pa3_sel(&self) -> &Pa3Sel {
        &self.pa3_sel
    }
    #[doc = "0x10 - Peripheral select control for PA4"]
    #[inline(always)]
    pub const fn pa4_sel(&self) -> &Pa4Sel {
        &self.pa4_sel
    }
    #[doc = "0x14 - Peripheral select control for PA5"]
    #[inline(always)]
    pub const fn pa5_sel(&self) -> &Pa5Sel {
        &self.pa5_sel
    }
    #[doc = "0x18 - Peripheral select control for PA6"]
    #[inline(always)]
    pub const fn pa6_sel(&self) -> &Pa6Sel {
        &self.pa6_sel
    }
    #[doc = "0x1c - Peripheral select control for PA7"]
    #[inline(always)]
    pub const fn pa7_sel(&self) -> &Pa7Sel {
        &self.pa7_sel
    }
    #[doc = "0x20 - Peripheral select control for PB0"]
    #[inline(always)]
    pub const fn pb0_sel(&self) -> &Pb0Sel {
        &self.pb0_sel
    }
    #[doc = "0x24 - Peripheral select control for PB1"]
    #[inline(always)]
    pub const fn pb1_sel(&self) -> &Pb1Sel {
        &self.pb1_sel
    }
    #[doc = "0x28 - Peripheral select control for PB2"]
    #[inline(always)]
    pub const fn pb2_sel(&self) -> &Pb2Sel {
        &self.pb2_sel
    }
    #[doc = "0x2c - Peripheral select control for PB3"]
    #[inline(always)]
    pub const fn pb3_sel(&self) -> &Pb3Sel {
        &self.pb3_sel
    }
    #[doc = "0x30 - Peripheral select control for PB4"]
    #[inline(always)]
    pub const fn pb4_sel(&self) -> &Pb4Sel {
        &self.pb4_sel
    }
    #[doc = "0x34 - Peripheral select control for PB5"]
    #[inline(always)]
    pub const fn pb5_sel(&self) -> &Pb5Sel {
        &self.pb5_sel
    }
    #[doc = "0x38 - Peripheral select control for PB6"]
    #[inline(always)]
    pub const fn pb6_sel(&self) -> &Pb6Sel {
        &self.pb6_sel
    }
    #[doc = "0x3c - Peripheral select control for PB7"]
    #[inline(always)]
    pub const fn pb7_sel(&self) -> &Pb7Sel {
        &self.pb7_sel
    }
    #[doc = "0x40 - Peripheral select control for PC0"]
    #[inline(always)]
    pub const fn pc0_sel(&self) -> &Pc0Sel {
        &self.pc0_sel
    }
    #[doc = "0x44 - Peripheral select control for PC1"]
    #[inline(always)]
    pub const fn pc1_sel(&self) -> &Pc1Sel {
        &self.pc1_sel
    }
    #[doc = "0x48 - Peripheral select control for PC2"]
    #[inline(always)]
    pub const fn pc2_sel(&self) -> &Pc2Sel {
        &self.pc2_sel
    }
    #[doc = "0x4c - Peripheral select control for PC3"]
    #[inline(always)]
    pub const fn pc3_sel(&self) -> &Pc3Sel {
        &self.pc3_sel
    }
    #[doc = "0x50 - Peripheral select control for PC4"]
    #[inline(always)]
    pub const fn pc4_sel(&self) -> &Pc4Sel {
        &self.pc4_sel
    }
    #[doc = "0x54 - Peripheral select control for PC5"]
    #[inline(always)]
    pub const fn pc5_sel(&self) -> &Pc5Sel {
        &self.pc5_sel
    }
    #[doc = "0x58 - Peripheral select control for PC6"]
    #[inline(always)]
    pub const fn pc6_sel(&self) -> &Pc6Sel {
        &self.pc6_sel
    }
    #[doc = "0x5c - Peripheral select control for PC7"]
    #[inline(always)]
    pub const fn pc7_sel(&self) -> &Pc7Sel {
        &self.pc7_sel
    }
    #[doc = "0x60 - Peripheral select control for PD0"]
    #[inline(always)]
    pub const fn pd0_sel(&self) -> &Pd0Sel {
        &self.pd0_sel
    }
    #[doc = "0x64 - Peripheral select control for PD1"]
    #[inline(always)]
    pub const fn pd1_sel(&self) -> &Pd1Sel {
        &self.pd1_sel
    }
    #[doc = "0x68 - Peripheral select control for PD2"]
    #[inline(always)]
    pub const fn pd2_sel(&self) -> &Pd2Sel {
        &self.pd2_sel
    }
    #[doc = "0x6c - Peripheral select control for PD3"]
    #[inline(always)]
    pub const fn pd3_sel(&self) -> &Pd3Sel {
        &self.pd3_sel
    }
    #[doc = "0x70 - Peripheral select control for PD4"]
    #[inline(always)]
    pub const fn pd4_sel(&self) -> &Pd4Sel {
        &self.pd4_sel
    }
    #[doc = "0x74 - Peripheral select control for PD5"]
    #[inline(always)]
    pub const fn pd5_sel(&self) -> &Pd5Sel {
        &self.pd5_sel
    }
    #[doc = "0x78 - Peripheral select control for PD6"]
    #[inline(always)]
    pub const fn pd6_sel(&self) -> &Pd6Sel {
        &self.pd6_sel
    }
    #[doc = "0x7c - Peripheral select control for PD7"]
    #[inline(always)]
    pub const fn pd7_sel(&self) -> &Pd7Sel {
        &self.pd7_sel
    }
    #[doc = "0x80 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa0_over(&self) -> &Pa0Over {
        &self.pa0_over
    }
    #[doc = "0x84 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa1_over(&self) -> &Pa1Over {
        &self.pa1_over
    }
    #[doc = "0x88 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa2_over(&self) -> &Pa2Over {
        &self.pa2_over
    }
    #[doc = "0x8c - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa3_over(&self) -> &Pa3Over {
        &self.pa3_over
    }
    #[doc = "0x90 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa4_over(&self) -> &Pa4Over {
        &self.pa4_over
    }
    #[doc = "0x94 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa5_over(&self) -> &Pa5Over {
        &self.pa5_over
    }
    #[doc = "0x98 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa6_over(&self) -> &Pa6Over {
        &self.pa6_over
    }
    #[doc = "0x9c - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa7_over(&self) -> &Pa7Over {
        &self.pa7_over
    }
    #[doc = "0xa0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb0_over(&self) -> &Pb0Over {
        &self.pb0_over
    }
    #[doc = "0xa4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb1_over(&self) -> &Pb1Over {
        &self.pb1_over
    }
    #[doc = "0xa8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb2_over(&self) -> &Pb2Over {
        &self.pb2_over
    }
    #[doc = "0xac - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb3_over(&self) -> &Pb3Over {
        &self.pb3_over
    }
    #[doc = "0xb0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb4_over(&self) -> &Pb4Over {
        &self.pb4_over
    }
    #[doc = "0xb4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb5_over(&self) -> &Pb5Over {
        &self.pb5_over
    }
    #[doc = "0xb8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb6_over(&self) -> &Pb6Over {
        &self.pb6_over
    }
    #[doc = "0xbc - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb7_over(&self) -> &Pb7Over {
        &self.pb7_over
    }
    #[doc = "0xc0 - This is the overide configuration register for each pad. PC0 has high drive capability."]
    #[inline(always)]
    pub const fn pc0_over(&self) -> &Pc0Over {
        &self.pc0_over
    }
    #[doc = "0xc4 - This is the overide configuration register for each pad. PC1 has high drive capability."]
    #[inline(always)]
    pub const fn pc1_over(&self) -> &Pc1Over {
        &self.pc1_over
    }
    #[doc = "0xc8 - This is the overide configuration register for each pad. PC2 has high drive capability."]
    #[inline(always)]
    pub const fn pc2_over(&self) -> &Pc2Over {
        &self.pc2_over
    }
    #[doc = "0xcc - This is the overide configuration register for each pad. PC3 has high drive capability."]
    #[inline(always)]
    pub const fn pc3_over(&self) -> &Pc3Over {
        &self.pc3_over
    }
    #[doc = "0xd0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pc4_over(&self) -> &Pc4Over {
        &self.pc4_over
    }
    #[doc = "0xd4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pc5_over(&self) -> &Pc5Over {
        &self.pc5_over
    }
    #[doc = "0xd8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pc6_over(&self) -> &Pc6Over {
        &self.pc6_over
    }
    #[doc = "0xdc - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pc7_over(&self) -> &Pc7Over {
        &self.pc7_over
    }
    #[doc = "0xe0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd0_over(&self) -> &Pd0Over {
        &self.pd0_over
    }
    #[doc = "0xe4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd1_over(&self) -> &Pd1Over {
        &self.pd1_over
    }
    #[doc = "0xe8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd2_over(&self) -> &Pd2Over {
        &self.pd2_over
    }
    #[doc = "0xec - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd3_over(&self) -> &Pd3Over {
        &self.pd3_over
    }
    #[doc = "0xf0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd4_over(&self) -> &Pd4Over {
        &self.pd4_over
    }
    #[doc = "0xf4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd5_over(&self) -> &Pd5Over {
        &self.pd5_over
    }
    #[doc = "0xf8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd6_over(&self) -> &Pd6Over {
        &self.pd6_over
    }
    #[doc = "0xfc - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd7_over(&self) -> &Pd7Over {
        &self.pd7_over
    }
    #[doc = "0x100 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX."]
    #[inline(always)]
    pub const fn uartrxd_uart0(&self) -> &UartrxdUart0 {
        &self.uartrxd_uart0
    }
    #[doc = "0x104 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS."]
    #[inline(always)]
    pub const fn uartcts_uart1(&self) -> &UartctsUart1 {
        &self.uartcts_uart1
    }
    #[doc = "0x108 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX."]
    #[inline(always)]
    pub const fn uartrxd_uart1(&self) -> &UartrxdUart1 {
        &self.uartrxd_uart1
    }
    #[doc = "0x10c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK."]
    #[inline(always)]
    pub const fn clk_ssi_ssi0(&self) -> &ClkSsiSsi0 {
        &self.clk_ssi_ssi0
    }
    #[doc = "0x110 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX."]
    #[inline(always)]
    pub const fn ssirxd_ssi0(&self) -> &SsirxdSsi0 {
        &self.ssirxd_ssi0
    }
    #[doc = "0x114 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN."]
    #[inline(always)]
    pub const fn ssifssin_ssi0(&self) -> &SsifssinSsi0 {
        &self.ssifssin_ssi0
    }
    #[doc = "0x118 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN."]
    #[inline(always)]
    pub const fn clk_ssiin_ssi0(&self) -> &ClkSsiinSsi0 {
        &self.clk_ssiin_ssi0
    }
    #[doc = "0x11c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK."]
    #[inline(always)]
    pub const fn clk_ssi_ssi1(&self) -> &ClkSsiSsi1 {
        &self.clk_ssi_ssi1
    }
    #[doc = "0x120 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX."]
    #[inline(always)]
    pub const fn ssirxd_ssi1(&self) -> &SsirxdSsi1 {
        &self.ssirxd_ssi1
    }
    #[doc = "0x124 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN."]
    #[inline(always)]
    pub const fn ssifssin_ssi1(&self) -> &SsifssinSsi1 {
        &self.ssifssin_ssi1
    }
    #[doc = "0x128 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN."]
    #[inline(always)]
    pub const fn clk_ssiin_ssi1(&self) -> &ClkSsiinSsi1 {
        &self.clk_ssiin_ssi1
    }
    #[doc = "0x12c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA."]
    #[inline(always)]
    pub const fn i2cmssda(&self) -> &I2cmssda {
        &self.i2cmssda
    }
    #[doc = "0x130 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL."]
    #[inline(always)]
    pub const fn i2cmsscl(&self) -> &I2cmsscl {
        &self.i2cmsscl
    }
    #[doc = "0x134 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1."]
    #[inline(always)]
    pub const fn gpt0ocp1(&self) -> &Gpt0ocp1 {
        &self.gpt0ocp1
    }
    #[doc = "0x138 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2."]
    #[inline(always)]
    pub const fn gpt0ocp2(&self) -> &Gpt0ocp2 {
        &self.gpt0ocp2
    }
    #[doc = "0x13c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1."]
    #[inline(always)]
    pub const fn gpt1ocp1(&self) -> &Gpt1ocp1 {
        &self.gpt1ocp1
    }
    #[doc = "0x140 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2."]
    #[inline(always)]
    pub const fn gpt1ocp2(&self) -> &Gpt1ocp2 {
        &self.gpt1ocp2
    }
    #[doc = "0x144 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1."]
    #[inline(always)]
    pub const fn gpt2ocp1(&self) -> &Gpt2ocp1 {
        &self.gpt2ocp1
    }
    #[doc = "0x148 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2."]
    #[inline(always)]
    pub const fn gpt2ocp2(&self) -> &Gpt2ocp2 {
        &self.gpt2ocp2
    }
    #[doc = "0x14c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1."]
    #[inline(always)]
    pub const fn gpt3ocp1(&self) -> &Gpt3ocp1 {
        &self.gpt3ocp1
    }
    #[doc = "0x150 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2."]
    #[inline(always)]
    pub const fn gpt3ocp2(&self) -> &Gpt3ocp2 {
        &self.gpt3ocp2
    }
}
#[doc = "PA0_SEL (rw) register accessor: Peripheral select control for PA0\n\nYou can [`read`](crate::Reg::read) this register and get [`pa0_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa0_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa0_sel`]
module"]
#[doc(alias = "PA0_SEL")]
pub type Pa0Sel = crate::Reg<pa0_sel::Pa0SelSpec>;
#[doc = "Peripheral select control for PA0"]
pub mod pa0_sel;
#[doc = "PA1_SEL (rw) register accessor: Peripheral select control for PA1\n\nYou can [`read`](crate::Reg::read) this register and get [`pa1_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa1_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa1_sel`]
module"]
#[doc(alias = "PA1_SEL")]
pub type Pa1Sel = crate::Reg<pa1_sel::Pa1SelSpec>;
#[doc = "Peripheral select control for PA1"]
pub mod pa1_sel;
#[doc = "PA2_SEL (rw) register accessor: Peripheral select control for PA2\n\nYou can [`read`](crate::Reg::read) this register and get [`pa2_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa2_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa2_sel`]
module"]
#[doc(alias = "PA2_SEL")]
pub type Pa2Sel = crate::Reg<pa2_sel::Pa2SelSpec>;
#[doc = "Peripheral select control for PA2"]
pub mod pa2_sel;
#[doc = "PA3_SEL (rw) register accessor: Peripheral select control for PA3\n\nYou can [`read`](crate::Reg::read) this register and get [`pa3_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa3_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa3_sel`]
module"]
#[doc(alias = "PA3_SEL")]
pub type Pa3Sel = crate::Reg<pa3_sel::Pa3SelSpec>;
#[doc = "Peripheral select control for PA3"]
pub mod pa3_sel;
#[doc = "PA4_SEL (rw) register accessor: Peripheral select control for PA4\n\nYou can [`read`](crate::Reg::read) this register and get [`pa4_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa4_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa4_sel`]
module"]
#[doc(alias = "PA4_SEL")]
pub type Pa4Sel = crate::Reg<pa4_sel::Pa4SelSpec>;
#[doc = "Peripheral select control for PA4"]
pub mod pa4_sel;
#[doc = "PA5_SEL (rw) register accessor: Peripheral select control for PA5\n\nYou can [`read`](crate::Reg::read) this register and get [`pa5_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa5_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa5_sel`]
module"]
#[doc(alias = "PA5_SEL")]
pub type Pa5Sel = crate::Reg<pa5_sel::Pa5SelSpec>;
#[doc = "Peripheral select control for PA5"]
pub mod pa5_sel;
#[doc = "PA6_SEL (rw) register accessor: Peripheral select control for PA6\n\nYou can [`read`](crate::Reg::read) this register and get [`pa6_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa6_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa6_sel`]
module"]
#[doc(alias = "PA6_SEL")]
pub type Pa6Sel = crate::Reg<pa6_sel::Pa6SelSpec>;
#[doc = "Peripheral select control for PA6"]
pub mod pa6_sel;
#[doc = "PA7_SEL (rw) register accessor: Peripheral select control for PA7\n\nYou can [`read`](crate::Reg::read) this register and get [`pa7_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa7_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa7_sel`]
module"]
#[doc(alias = "PA7_SEL")]
pub type Pa7Sel = crate::Reg<pa7_sel::Pa7SelSpec>;
#[doc = "Peripheral select control for PA7"]
pub mod pa7_sel;
#[doc = "PB0_SEL (rw) register accessor: Peripheral select control for PB0\n\nYou can [`read`](crate::Reg::read) this register and get [`pb0_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb0_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb0_sel`]
module"]
#[doc(alias = "PB0_SEL")]
pub type Pb0Sel = crate::Reg<pb0_sel::Pb0SelSpec>;
#[doc = "Peripheral select control for PB0"]
pub mod pb0_sel;
#[doc = "PB1_SEL (rw) register accessor: Peripheral select control for PB1\n\nYou can [`read`](crate::Reg::read) this register and get [`pb1_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb1_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb1_sel`]
module"]
#[doc(alias = "PB1_SEL")]
pub type Pb1Sel = crate::Reg<pb1_sel::Pb1SelSpec>;
#[doc = "Peripheral select control for PB1"]
pub mod pb1_sel;
#[doc = "PB2_SEL (rw) register accessor: Peripheral select control for PB2\n\nYou can [`read`](crate::Reg::read) this register and get [`pb2_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb2_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb2_sel`]
module"]
#[doc(alias = "PB2_SEL")]
pub type Pb2Sel = crate::Reg<pb2_sel::Pb2SelSpec>;
#[doc = "Peripheral select control for PB2"]
pub mod pb2_sel;
#[doc = "PB3_SEL (rw) register accessor: Peripheral select control for PB3\n\nYou can [`read`](crate::Reg::read) this register and get [`pb3_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb3_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb3_sel`]
module"]
#[doc(alias = "PB3_SEL")]
pub type Pb3Sel = crate::Reg<pb3_sel::Pb3SelSpec>;
#[doc = "Peripheral select control for PB3"]
pub mod pb3_sel;
#[doc = "PB4_SEL (rw) register accessor: Peripheral select control for PB4\n\nYou can [`read`](crate::Reg::read) this register and get [`pb4_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb4_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb4_sel`]
module"]
#[doc(alias = "PB4_SEL")]
pub type Pb4Sel = crate::Reg<pb4_sel::Pb4SelSpec>;
#[doc = "Peripheral select control for PB4"]
pub mod pb4_sel;
#[doc = "PB5_SEL (rw) register accessor: Peripheral select control for PB5\n\nYou can [`read`](crate::Reg::read) this register and get [`pb5_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb5_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb5_sel`]
module"]
#[doc(alias = "PB5_SEL")]
pub type Pb5Sel = crate::Reg<pb5_sel::Pb5SelSpec>;
#[doc = "Peripheral select control for PB5"]
pub mod pb5_sel;
#[doc = "PB6_SEL (rw) register accessor: Peripheral select control for PB6\n\nYou can [`read`](crate::Reg::read) this register and get [`pb6_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb6_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb6_sel`]
module"]
#[doc(alias = "PB6_SEL")]
pub type Pb6Sel = crate::Reg<pb6_sel::Pb6SelSpec>;
#[doc = "Peripheral select control for PB6"]
pub mod pb6_sel;
#[doc = "PB7_SEL (rw) register accessor: Peripheral select control for PB7\n\nYou can [`read`](crate::Reg::read) this register and get [`pb7_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb7_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb7_sel`]
module"]
#[doc(alias = "PB7_SEL")]
pub type Pb7Sel = crate::Reg<pb7_sel::Pb7SelSpec>;
#[doc = "Peripheral select control for PB7"]
pub mod pb7_sel;
#[doc = "PC0_SEL (rw) register accessor: Peripheral select control for PC0\n\nYou can [`read`](crate::Reg::read) this register and get [`pc0_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc0_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc0_sel`]
module"]
#[doc(alias = "PC0_SEL")]
pub type Pc0Sel = crate::Reg<pc0_sel::Pc0SelSpec>;
#[doc = "Peripheral select control for PC0"]
pub mod pc0_sel;
#[doc = "PC1_SEL (rw) register accessor: Peripheral select control for PC1\n\nYou can [`read`](crate::Reg::read) this register and get [`pc1_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc1_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc1_sel`]
module"]
#[doc(alias = "PC1_SEL")]
pub type Pc1Sel = crate::Reg<pc1_sel::Pc1SelSpec>;
#[doc = "Peripheral select control for PC1"]
pub mod pc1_sel;
#[doc = "PC2_SEL (rw) register accessor: Peripheral select control for PC2\n\nYou can [`read`](crate::Reg::read) this register and get [`pc2_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc2_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc2_sel`]
module"]
#[doc(alias = "PC2_SEL")]
pub type Pc2Sel = crate::Reg<pc2_sel::Pc2SelSpec>;
#[doc = "Peripheral select control for PC2"]
pub mod pc2_sel;
#[doc = "PC3_SEL (rw) register accessor: Peripheral select control for PC3\n\nYou can [`read`](crate::Reg::read) this register and get [`pc3_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc3_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc3_sel`]
module"]
#[doc(alias = "PC3_SEL")]
pub type Pc3Sel = crate::Reg<pc3_sel::Pc3SelSpec>;
#[doc = "Peripheral select control for PC3"]
pub mod pc3_sel;
#[doc = "PC4_SEL (rw) register accessor: Peripheral select control for PC4\n\nYou can [`read`](crate::Reg::read) this register and get [`pc4_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc4_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc4_sel`]
module"]
#[doc(alias = "PC4_SEL")]
pub type Pc4Sel = crate::Reg<pc4_sel::Pc4SelSpec>;
#[doc = "Peripheral select control for PC4"]
pub mod pc4_sel;
#[doc = "PC5_SEL (rw) register accessor: Peripheral select control for PC5\n\nYou can [`read`](crate::Reg::read) this register and get [`pc5_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc5_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc5_sel`]
module"]
#[doc(alias = "PC5_SEL")]
pub type Pc5Sel = crate::Reg<pc5_sel::Pc5SelSpec>;
#[doc = "Peripheral select control for PC5"]
pub mod pc5_sel;
#[doc = "PC6_SEL (rw) register accessor: Peripheral select control for PC6\n\nYou can [`read`](crate::Reg::read) this register and get [`pc6_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc6_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc6_sel`]
module"]
#[doc(alias = "PC6_SEL")]
pub type Pc6Sel = crate::Reg<pc6_sel::Pc6SelSpec>;
#[doc = "Peripheral select control for PC6"]
pub mod pc6_sel;
#[doc = "PC7_SEL (rw) register accessor: Peripheral select control for PC7\n\nYou can [`read`](crate::Reg::read) this register and get [`pc7_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc7_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc7_sel`]
module"]
#[doc(alias = "PC7_SEL")]
pub type Pc7Sel = crate::Reg<pc7_sel::Pc7SelSpec>;
#[doc = "Peripheral select control for PC7"]
pub mod pc7_sel;
#[doc = "PD0_SEL (rw) register accessor: Peripheral select control for PD0\n\nYou can [`read`](crate::Reg::read) this register and get [`pd0_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd0_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd0_sel`]
module"]
#[doc(alias = "PD0_SEL")]
pub type Pd0Sel = crate::Reg<pd0_sel::Pd0SelSpec>;
#[doc = "Peripheral select control for PD0"]
pub mod pd0_sel;
#[doc = "PD1_SEL (rw) register accessor: Peripheral select control for PD1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd1_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd1_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd1_sel`]
module"]
#[doc(alias = "PD1_SEL")]
pub type Pd1Sel = crate::Reg<pd1_sel::Pd1SelSpec>;
#[doc = "Peripheral select control for PD1"]
pub mod pd1_sel;
#[doc = "PD2_SEL (rw) register accessor: Peripheral select control for PD2\n\nYou can [`read`](crate::Reg::read) this register and get [`pd2_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd2_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd2_sel`]
module"]
#[doc(alias = "PD2_SEL")]
pub type Pd2Sel = crate::Reg<pd2_sel::Pd2SelSpec>;
#[doc = "Peripheral select control for PD2"]
pub mod pd2_sel;
#[doc = "PD3_SEL (rw) register accessor: Peripheral select control for PD3\n\nYou can [`read`](crate::Reg::read) this register and get [`pd3_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd3_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd3_sel`]
module"]
#[doc(alias = "PD3_SEL")]
pub type Pd3Sel = crate::Reg<pd3_sel::Pd3SelSpec>;
#[doc = "Peripheral select control for PD3"]
pub mod pd3_sel;
#[doc = "PD4_SEL (rw) register accessor: Peripheral select control for PD4\n\nYou can [`read`](crate::Reg::read) this register and get [`pd4_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd4_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd4_sel`]
module"]
#[doc(alias = "PD4_SEL")]
pub type Pd4Sel = crate::Reg<pd4_sel::Pd4SelSpec>;
#[doc = "Peripheral select control for PD4"]
pub mod pd4_sel;
#[doc = "PD5_SEL (rw) register accessor: Peripheral select control for PD5\n\nYou can [`read`](crate::Reg::read) this register and get [`pd5_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd5_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd5_sel`]
module"]
#[doc(alias = "PD5_SEL")]
pub type Pd5Sel = crate::Reg<pd5_sel::Pd5SelSpec>;
#[doc = "Peripheral select control for PD5"]
pub mod pd5_sel;
#[doc = "PD6_SEL (rw) register accessor: Peripheral select control for PD6\n\nYou can [`read`](crate::Reg::read) this register and get [`pd6_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd6_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd6_sel`]
module"]
#[doc(alias = "PD6_SEL")]
pub type Pd6Sel = crate::Reg<pd6_sel::Pd6SelSpec>;
#[doc = "Peripheral select control for PD6"]
pub mod pd6_sel;
#[doc = "PD7_SEL (rw) register accessor: Peripheral select control for PD7\n\nYou can [`read`](crate::Reg::read) this register and get [`pd7_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd7_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd7_sel`]
module"]
#[doc(alias = "PD7_SEL")]
pub type Pd7Sel = crate::Reg<pd7_sel::Pd7SelSpec>;
#[doc = "Peripheral select control for PD7"]
pub mod pd7_sel;
#[doc = "PA0_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa0_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa0_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa0_over`]
module"]
#[doc(alias = "PA0_OVER")]
pub type Pa0Over = crate::Reg<pa0_over::Pa0OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa0_over;
#[doc = "PA1_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa1_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa1_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa1_over`]
module"]
#[doc(alias = "PA1_OVER")]
pub type Pa1Over = crate::Reg<pa1_over::Pa1OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa1_over;
#[doc = "PA2_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa2_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa2_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa2_over`]
module"]
#[doc(alias = "PA2_OVER")]
pub type Pa2Over = crate::Reg<pa2_over::Pa2OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa2_over;
#[doc = "PA3_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa3_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa3_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa3_over`]
module"]
#[doc(alias = "PA3_OVER")]
pub type Pa3Over = crate::Reg<pa3_over::Pa3OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa3_over;
#[doc = "PA4_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa4_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa4_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa4_over`]
module"]
#[doc(alias = "PA4_OVER")]
pub type Pa4Over = crate::Reg<pa4_over::Pa4OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa4_over;
#[doc = "PA5_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa5_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa5_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa5_over`]
module"]
#[doc(alias = "PA5_OVER")]
pub type Pa5Over = crate::Reg<pa5_over::Pa5OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa5_over;
#[doc = "PA6_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa6_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa6_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa6_over`]
module"]
#[doc(alias = "PA6_OVER")]
pub type Pa6Over = crate::Reg<pa6_over::Pa6OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa6_over;
#[doc = "PA7_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa7_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa7_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa7_over`]
module"]
#[doc(alias = "PA7_OVER")]
pub type Pa7Over = crate::Reg<pa7_over::Pa7OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa7_over;
#[doc = "PB0_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb0_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb0_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb0_over`]
module"]
#[doc(alias = "PB0_OVER")]
pub type Pb0Over = crate::Reg<pb0_over::Pb0OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb0_over;
#[doc = "PB1_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb1_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb1_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb1_over`]
module"]
#[doc(alias = "PB1_OVER")]
pub type Pb1Over = crate::Reg<pb1_over::Pb1OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb1_over;
#[doc = "PB2_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb2_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb2_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb2_over`]
module"]
#[doc(alias = "PB2_OVER")]
pub type Pb2Over = crate::Reg<pb2_over::Pb2OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb2_over;
#[doc = "PB3_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb3_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb3_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb3_over`]
module"]
#[doc(alias = "PB3_OVER")]
pub type Pb3Over = crate::Reg<pb3_over::Pb3OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb3_over;
#[doc = "PB4_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb4_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb4_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb4_over`]
module"]
#[doc(alias = "PB4_OVER")]
pub type Pb4Over = crate::Reg<pb4_over::Pb4OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb4_over;
#[doc = "PB5_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb5_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb5_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb5_over`]
module"]
#[doc(alias = "PB5_OVER")]
pub type Pb5Over = crate::Reg<pb5_over::Pb5OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb5_over;
#[doc = "PB6_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb6_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb6_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb6_over`]
module"]
#[doc(alias = "PB6_OVER")]
pub type Pb6Over = crate::Reg<pb6_over::Pb6OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb6_over;
#[doc = "PB7_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb7_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb7_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb7_over`]
module"]
#[doc(alias = "PB7_OVER")]
pub type Pb7Over = crate::Reg<pb7_over::Pb7OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb7_over;
#[doc = "PC0_OVER (rw) register accessor: This is the overide configuration register for each pad. PC0 has high drive capability.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc0_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc0_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc0_over`]
module"]
#[doc(alias = "PC0_OVER")]
pub type Pc0Over = crate::Reg<pc0_over::Pc0OverSpec>;
#[doc = "This is the overide configuration register for each pad. PC0 has high drive capability."]
pub mod pc0_over;
#[doc = "PC1_OVER (rw) register accessor: This is the overide configuration register for each pad. PC1 has high drive capability.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc1_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc1_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc1_over`]
module"]
#[doc(alias = "PC1_OVER")]
pub type Pc1Over = crate::Reg<pc1_over::Pc1OverSpec>;
#[doc = "This is the overide configuration register for each pad. PC1 has high drive capability."]
pub mod pc1_over;
#[doc = "PC2_OVER (rw) register accessor: This is the overide configuration register for each pad. PC2 has high drive capability.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc2_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc2_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc2_over`]
module"]
#[doc(alias = "PC2_OVER")]
pub type Pc2Over = crate::Reg<pc2_over::Pc2OverSpec>;
#[doc = "This is the overide configuration register for each pad. PC2 has high drive capability."]
pub mod pc2_over;
#[doc = "PC3_OVER (rw) register accessor: This is the overide configuration register for each pad. PC3 has high drive capability.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc3_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc3_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc3_over`]
module"]
#[doc(alias = "PC3_OVER")]
pub type Pc3Over = crate::Reg<pc3_over::Pc3OverSpec>;
#[doc = "This is the overide configuration register for each pad. PC3 has high drive capability."]
pub mod pc3_over;
#[doc = "PC4_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc4_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc4_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc4_over`]
module"]
#[doc(alias = "PC4_OVER")]
pub type Pc4Over = crate::Reg<pc4_over::Pc4OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc4_over;
#[doc = "PC5_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc5_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc5_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc5_over`]
module"]
#[doc(alias = "PC5_OVER")]
pub type Pc5Over = crate::Reg<pc5_over::Pc5OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc5_over;
#[doc = "PC6_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc6_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc6_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc6_over`]
module"]
#[doc(alias = "PC6_OVER")]
pub type Pc6Over = crate::Reg<pc6_over::Pc6OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc6_over;
#[doc = "PC7_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc7_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc7_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc7_over`]
module"]
#[doc(alias = "PC7_OVER")]
pub type Pc7Over = crate::Reg<pc7_over::Pc7OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc7_over;
#[doc = "PD0_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd0_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd0_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd0_over`]
module"]
#[doc(alias = "PD0_OVER")]
pub type Pd0Over = crate::Reg<pd0_over::Pd0OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd0_over;
#[doc = "PD1_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd1_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd1_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd1_over`]
module"]
#[doc(alias = "PD1_OVER")]
pub type Pd1Over = crate::Reg<pd1_over::Pd1OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd1_over;
#[doc = "PD2_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd2_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd2_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd2_over`]
module"]
#[doc(alias = "PD2_OVER")]
pub type Pd2Over = crate::Reg<pd2_over::Pd2OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd2_over;
#[doc = "PD3_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd3_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd3_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd3_over`]
module"]
#[doc(alias = "PD3_OVER")]
pub type Pd3Over = crate::Reg<pd3_over::Pd3OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd3_over;
#[doc = "PD4_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd4_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd4_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd4_over`]
module"]
#[doc(alias = "PD4_OVER")]
pub type Pd4Over = crate::Reg<pd4_over::Pd4OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd4_over;
#[doc = "PD5_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd5_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd5_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd5_over`]
module"]
#[doc(alias = "PD5_OVER")]
pub type Pd5Over = crate::Reg<pd5_over::Pd5OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd5_over;
#[doc = "PD6_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd6_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd6_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd6_over`]
module"]
#[doc(alias = "PD6_OVER")]
pub type Pd6Over = crate::Reg<pd6_over::Pd6OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd6_over;
#[doc = "PD7_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd7_over::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd7_over::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd7_over`]
module"]
#[doc(alias = "PD7_OVER")]
pub type Pd7Over = crate::Reg<pd7_over::Pd7OverSpec>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd7_over;
#[doc = "UARTRXD_UART0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX.\n\nYou can [`read`](crate::Reg::read) this register and get [`uartrxd_uart0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartrxd_uart0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartrxd_uart0`]
module"]
#[doc(alias = "UARTRXD_UART0")]
pub type UartrxdUart0 = crate::Reg<uartrxd_uart0::UartrxdUart0Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX."]
pub mod uartrxd_uart0;
#[doc = "UARTCTS_UART1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS.\n\nYou can [`read`](crate::Reg::read) this register and get [`uartcts_uart1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartcts_uart1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartcts_uart1`]
module"]
#[doc(alias = "UARTCTS_UART1")]
pub type UartctsUart1 = crate::Reg<uartcts_uart1::UartctsUart1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS."]
pub mod uartcts_uart1;
#[doc = "UARTRXD_UART1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX.\n\nYou can [`read`](crate::Reg::read) this register and get [`uartrxd_uart1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartrxd_uart1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartrxd_uart1`]
module"]
#[doc(alias = "UARTRXD_UART1")]
pub type UartrxdUart1 = crate::Reg<uartrxd_uart1::UartrxdUart1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX."]
pub mod uartrxd_uart1;
#[doc = "CLK_SSI_SSI0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ssi_ssi0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ssi_ssi0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ssi_ssi0`]
module"]
#[doc(alias = "CLK_SSI_SSI0")]
pub type ClkSsiSsi0 = crate::Reg<clk_ssi_ssi0::ClkSsiSsi0Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK."]
pub mod clk_ssi_ssi0;
#[doc = "SSIRXD_SSI0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssirxd_ssi0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssirxd_ssi0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssirxd_ssi0`]
module"]
#[doc(alias = "SSIRXD_SSI0")]
pub type SsirxdSsi0 = crate::Reg<ssirxd_ssi0::SsirxdSsi0Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX."]
pub mod ssirxd_ssi0;
#[doc = "SSIFSSIN_SSI0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssifssin_ssi0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifssin_ssi0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifssin_ssi0`]
module"]
#[doc(alias = "SSIFSSIN_SSI0")]
pub type SsifssinSsi0 = crate::Reg<ssifssin_ssi0::SsifssinSsi0Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN."]
pub mod ssifssin_ssi0;
#[doc = "CLK_SSIIN_SSI0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ssiin_ssi0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ssiin_ssi0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ssiin_ssi0`]
module"]
#[doc(alias = "CLK_SSIIN_SSI0")]
pub type ClkSsiinSsi0 = crate::Reg<clk_ssiin_ssi0::ClkSsiinSsi0Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN."]
pub mod clk_ssiin_ssi0;
#[doc = "CLK_SSI_SSI1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ssi_ssi1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ssi_ssi1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ssi_ssi1`]
module"]
#[doc(alias = "CLK_SSI_SSI1")]
pub type ClkSsiSsi1 = crate::Reg<clk_ssi_ssi1::ClkSsiSsi1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK."]
pub mod clk_ssi_ssi1;
#[doc = "SSIRXD_SSI1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssirxd_ssi1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssirxd_ssi1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssirxd_ssi1`]
module"]
#[doc(alias = "SSIRXD_SSI1")]
pub type SsirxdSsi1 = crate::Reg<ssirxd_ssi1::SsirxdSsi1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX."]
pub mod ssirxd_ssi1;
#[doc = "SSIFSSIN_SSI1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssifssin_ssi1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifssin_ssi1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifssin_ssi1`]
module"]
#[doc(alias = "SSIFSSIN_SSI1")]
pub type SsifssinSsi1 = crate::Reg<ssifssin_ssi1::SsifssinSsi1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN."]
pub mod ssifssin_ssi1;
#[doc = "CLK_SSIIN_SSI1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ssiin_ssi1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ssiin_ssi1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ssiin_ssi1`]
module"]
#[doc(alias = "CLK_SSIIN_SSI1")]
pub type ClkSsiinSsi1 = crate::Reg<clk_ssiin_ssi1::ClkSsiinSsi1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN."]
pub mod clk_ssiin_ssi1;
#[doc = "I2CMSSDA (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cmssda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cmssda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cmssda`]
module"]
#[doc(alias = "I2CMSSDA")]
pub type I2cmssda = crate::Reg<i2cmssda::I2cmssdaSpec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA."]
pub mod i2cmssda;
#[doc = "I2CMSSCL (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cmsscl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cmsscl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cmsscl`]
module"]
#[doc(alias = "I2CMSSCL")]
pub type I2cmsscl = crate::Reg<i2cmsscl::I2cmssclSpec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL."]
pub mod i2cmsscl;
#[doc = "GPT0OCP1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpt0ocp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpt0ocp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt0ocp1`]
module"]
#[doc(alias = "GPT0OCP1")]
pub type Gpt0ocp1 = crate::Reg<gpt0ocp1::Gpt0ocp1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1."]
pub mod gpt0ocp1;
#[doc = "GPT0OCP2 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpt0ocp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpt0ocp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt0ocp2`]
module"]
#[doc(alias = "GPT0OCP2")]
pub type Gpt0ocp2 = crate::Reg<gpt0ocp2::Gpt0ocp2Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2."]
pub mod gpt0ocp2;
#[doc = "GPT1OCP1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpt1ocp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpt1ocp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt1ocp1`]
module"]
#[doc(alias = "GPT1OCP1")]
pub type Gpt1ocp1 = crate::Reg<gpt1ocp1::Gpt1ocp1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1."]
pub mod gpt1ocp1;
#[doc = "GPT1OCP2 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpt1ocp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpt1ocp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt1ocp2`]
module"]
#[doc(alias = "GPT1OCP2")]
pub type Gpt1ocp2 = crate::Reg<gpt1ocp2::Gpt1ocp2Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2."]
pub mod gpt1ocp2;
#[doc = "GPT2OCP1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpt2ocp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpt2ocp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt2ocp1`]
module"]
#[doc(alias = "GPT2OCP1")]
pub type Gpt2ocp1 = crate::Reg<gpt2ocp1::Gpt2ocp1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1."]
pub mod gpt2ocp1;
#[doc = "GPT2OCP2 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpt2ocp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpt2ocp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt2ocp2`]
module"]
#[doc(alias = "GPT2OCP2")]
pub type Gpt2ocp2 = crate::Reg<gpt2ocp2::Gpt2ocp2Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2."]
pub mod gpt2ocp2;
#[doc = "GPT3OCP1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpt3ocp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpt3ocp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt3ocp1`]
module"]
#[doc(alias = "GPT3OCP1")]
pub type Gpt3ocp1 = crate::Reg<gpt3ocp1::Gpt3ocp1Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1."]
pub mod gpt3ocp1;
#[doc = "GPT3OCP2 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpt3ocp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpt3ocp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt3ocp2`]
module"]
#[doc(alias = "GPT3OCP2")]
pub type Gpt3ocp2 = crate::Reg<gpt3ocp2::Gpt3ocp2Spec>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2."]
pub mod gpt3ocp2;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pa0_sel: PA0_SEL,
    pa1_sel: PA1_SEL,
    pa2_sel: PA2_SEL,
    pa3_sel: PA3_SEL,
    pa4_sel: PA4_SEL,
    pa5_sel: PA5_SEL,
    pa6_sel: PA6_SEL,
    pa7_sel: PA7_SEL,
    pb0_sel: PB0_SEL,
    pb1_sel: PB1_SEL,
    pb2_sel: PB2_SEL,
    pb3_sel: PB3_SEL,
    pb4_sel: PB4_SEL,
    pb5_sel: PB5_SEL,
    pb6_sel: PB6_SEL,
    pb7_sel: PB7_SEL,
    pc0_sel: PC0_SEL,
    pc1_sel: PC1_SEL,
    pc2_sel: PC2_SEL,
    pc3_sel: PC3_SEL,
    pc4_sel: PC4_SEL,
    pc5_sel: PC5_SEL,
    pc6_sel: PC6_SEL,
    pc7_sel: PC7_SEL,
    pd0_sel: PD0_SEL,
    pd1_sel: PD1_SEL,
    pd2_sel: PD2_SEL,
    pd3_sel: PD3_SEL,
    pd4_sel: PD4_SEL,
    pd5_sel: PD5_SEL,
    pd6_sel: PD6_SEL,
    pd7_sel: PD7_SEL,
    pa0_over: PA0_OVER,
    pa1_over: PA1_OVER,
    pa2_over: PA2_OVER,
    pa3_over: PA3_OVER,
    pa4_over: PA4_OVER,
    pa5_over: PA5_OVER,
    pa6_over: PA6_OVER,
    pa7_over: PA7_OVER,
    pb0_over: PB0_OVER,
    pb1_over: PB1_OVER,
    pb2_over: PB2_OVER,
    pb3_over: PB3_OVER,
    pb4_over: PB4_OVER,
    pb5_over: PB5_OVER,
    pb6_over: PB6_OVER,
    pb7_over: PB7_OVER,
    pc0_over: PC0_OVER,
    pc1_over: PC1_OVER,
    pc2_over: PC2_OVER,
    pc3_over: PC3_OVER,
    pc4_over: PC4_OVER,
    pc5_over: PC5_OVER,
    pc6_over: PC6_OVER,
    pc7_over: PC7_OVER,
    pd0_over: PD0_OVER,
    pd1_over: PD1_OVER,
    pd2_over: PD2_OVER,
    pd3_over: PD3_OVER,
    pd4_over: PD4_OVER,
    pd5_over: PD5_OVER,
    pd6_over: PD6_OVER,
    pd7_over: PD7_OVER,
    uartrxd_uart0: UARTRXD_UART0,
    uartcts_uart1: UARTCTS_UART1,
    uartrxd_uart1: UARTRXD_UART1,
    clk_ssi_ssi0: CLK_SSI_SSI0,
    ssirxd_ssi0: SSIRXD_SSI0,
    ssifssin_ssi0: SSIFSSIN_SSI0,
    clk_ssiin_ssi0: CLK_SSIIN_SSI0,
    clk_ssi_ssi1: CLK_SSI_SSI1,
    ssirxd_ssi1: SSIRXD_SSI1,
    ssifssin_ssi1: SSIFSSIN_SSI1,
    clk_ssiin_ssi1: CLK_SSIIN_SSI1,
    i2cmssda: I2CMSSDA,
    i2cmsscl: I2CMSSCL,
    gpt0ocp1: GPT0OCP1,
    gpt0ocp2: GPT0OCP2,
    gpt1ocp1: GPT1OCP1,
    gpt1ocp2: GPT1OCP2,
    gpt2ocp1: GPT2OCP1,
    gpt2ocp2: GPT2OCP2,
    gpt3ocp1: GPT3OCP1,
    gpt3ocp2: GPT3OCP2,
}
impl RegisterBlock {
    #[doc = "0x00 - Peripheral select control for PA0"]
    #[inline(always)]
    pub const fn pa0_sel(&self) -> &PA0_SEL {
        &self.pa0_sel
    }
    #[doc = "0x04 - Peripheral select control for PA1"]
    #[inline(always)]
    pub const fn pa1_sel(&self) -> &PA1_SEL {
        &self.pa1_sel
    }
    #[doc = "0x08 - Peripheral select control for PA2"]
    #[inline(always)]
    pub const fn pa2_sel(&self) -> &PA2_SEL {
        &self.pa2_sel
    }
    #[doc = "0x0c - Peripheral select control for PA3"]
    #[inline(always)]
    pub const fn pa3_sel(&self) -> &PA3_SEL {
        &self.pa3_sel
    }
    #[doc = "0x10 - Peripheral select control for PA4"]
    #[inline(always)]
    pub const fn pa4_sel(&self) -> &PA4_SEL {
        &self.pa4_sel
    }
    #[doc = "0x14 - Peripheral select control for PA5"]
    #[inline(always)]
    pub const fn pa5_sel(&self) -> &PA5_SEL {
        &self.pa5_sel
    }
    #[doc = "0x18 - Peripheral select control for PA6"]
    #[inline(always)]
    pub const fn pa6_sel(&self) -> &PA6_SEL {
        &self.pa6_sel
    }
    #[doc = "0x1c - Peripheral select control for PA7"]
    #[inline(always)]
    pub const fn pa7_sel(&self) -> &PA7_SEL {
        &self.pa7_sel
    }
    #[doc = "0x20 - Peripheral select control for PB0"]
    #[inline(always)]
    pub const fn pb0_sel(&self) -> &PB0_SEL {
        &self.pb0_sel
    }
    #[doc = "0x24 - Peripheral select control for PB1"]
    #[inline(always)]
    pub const fn pb1_sel(&self) -> &PB1_SEL {
        &self.pb1_sel
    }
    #[doc = "0x28 - Peripheral select control for PB2"]
    #[inline(always)]
    pub const fn pb2_sel(&self) -> &PB2_SEL {
        &self.pb2_sel
    }
    #[doc = "0x2c - Peripheral select control for PB3"]
    #[inline(always)]
    pub const fn pb3_sel(&self) -> &PB3_SEL {
        &self.pb3_sel
    }
    #[doc = "0x30 - Peripheral select control for PB4"]
    #[inline(always)]
    pub const fn pb4_sel(&self) -> &PB4_SEL {
        &self.pb4_sel
    }
    #[doc = "0x34 - Peripheral select control for PB5"]
    #[inline(always)]
    pub const fn pb5_sel(&self) -> &PB5_SEL {
        &self.pb5_sel
    }
    #[doc = "0x38 - Peripheral select control for PB6"]
    #[inline(always)]
    pub const fn pb6_sel(&self) -> &PB6_SEL {
        &self.pb6_sel
    }
    #[doc = "0x3c - Peripheral select control for PB7"]
    #[inline(always)]
    pub const fn pb7_sel(&self) -> &PB7_SEL {
        &self.pb7_sel
    }
    #[doc = "0x40 - Peripheral select control for PC0"]
    #[inline(always)]
    pub const fn pc0_sel(&self) -> &PC0_SEL {
        &self.pc0_sel
    }
    #[doc = "0x44 - Peripheral select control for PC1"]
    #[inline(always)]
    pub const fn pc1_sel(&self) -> &PC1_SEL {
        &self.pc1_sel
    }
    #[doc = "0x48 - Peripheral select control for PC2"]
    #[inline(always)]
    pub const fn pc2_sel(&self) -> &PC2_SEL {
        &self.pc2_sel
    }
    #[doc = "0x4c - Peripheral select control for PC3"]
    #[inline(always)]
    pub const fn pc3_sel(&self) -> &PC3_SEL {
        &self.pc3_sel
    }
    #[doc = "0x50 - Peripheral select control for PC4"]
    #[inline(always)]
    pub const fn pc4_sel(&self) -> &PC4_SEL {
        &self.pc4_sel
    }
    #[doc = "0x54 - Peripheral select control for PC5"]
    #[inline(always)]
    pub const fn pc5_sel(&self) -> &PC5_SEL {
        &self.pc5_sel
    }
    #[doc = "0x58 - Peripheral select control for PC6"]
    #[inline(always)]
    pub const fn pc6_sel(&self) -> &PC6_SEL {
        &self.pc6_sel
    }
    #[doc = "0x5c - Peripheral select control for PC7"]
    #[inline(always)]
    pub const fn pc7_sel(&self) -> &PC7_SEL {
        &self.pc7_sel
    }
    #[doc = "0x60 - Peripheral select control for PD0"]
    #[inline(always)]
    pub const fn pd0_sel(&self) -> &PD0_SEL {
        &self.pd0_sel
    }
    #[doc = "0x64 - Peripheral select control for PD1"]
    #[inline(always)]
    pub const fn pd1_sel(&self) -> &PD1_SEL {
        &self.pd1_sel
    }
    #[doc = "0x68 - Peripheral select control for PD2"]
    #[inline(always)]
    pub const fn pd2_sel(&self) -> &PD2_SEL {
        &self.pd2_sel
    }
    #[doc = "0x6c - Peripheral select control for PD3"]
    #[inline(always)]
    pub const fn pd3_sel(&self) -> &PD3_SEL {
        &self.pd3_sel
    }
    #[doc = "0x70 - Peripheral select control for PD4"]
    #[inline(always)]
    pub const fn pd4_sel(&self) -> &PD4_SEL {
        &self.pd4_sel
    }
    #[doc = "0x74 - Peripheral select control for PD5"]
    #[inline(always)]
    pub const fn pd5_sel(&self) -> &PD5_SEL {
        &self.pd5_sel
    }
    #[doc = "0x78 - Peripheral select control for PD6"]
    #[inline(always)]
    pub const fn pd6_sel(&self) -> &PD6_SEL {
        &self.pd6_sel
    }
    #[doc = "0x7c - Peripheral select control for PD7"]
    #[inline(always)]
    pub const fn pd7_sel(&self) -> &PD7_SEL {
        &self.pd7_sel
    }
    #[doc = "0x80 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa0_over(&self) -> &PA0_OVER {
        &self.pa0_over
    }
    #[doc = "0x84 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa1_over(&self) -> &PA1_OVER {
        &self.pa1_over
    }
    #[doc = "0x88 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa2_over(&self) -> &PA2_OVER {
        &self.pa2_over
    }
    #[doc = "0x8c - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa3_over(&self) -> &PA3_OVER {
        &self.pa3_over
    }
    #[doc = "0x90 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa4_over(&self) -> &PA4_OVER {
        &self.pa4_over
    }
    #[doc = "0x94 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa5_over(&self) -> &PA5_OVER {
        &self.pa5_over
    }
    #[doc = "0x98 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa6_over(&self) -> &PA6_OVER {
        &self.pa6_over
    }
    #[doc = "0x9c - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pa7_over(&self) -> &PA7_OVER {
        &self.pa7_over
    }
    #[doc = "0xa0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb0_over(&self) -> &PB0_OVER {
        &self.pb0_over
    }
    #[doc = "0xa4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb1_over(&self) -> &PB1_OVER {
        &self.pb1_over
    }
    #[doc = "0xa8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb2_over(&self) -> &PB2_OVER {
        &self.pb2_over
    }
    #[doc = "0xac - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb3_over(&self) -> &PB3_OVER {
        &self.pb3_over
    }
    #[doc = "0xb0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb4_over(&self) -> &PB4_OVER {
        &self.pb4_over
    }
    #[doc = "0xb4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb5_over(&self) -> &PB5_OVER {
        &self.pb5_over
    }
    #[doc = "0xb8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb6_over(&self) -> &PB6_OVER {
        &self.pb6_over
    }
    #[doc = "0xbc - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pb7_over(&self) -> &PB7_OVER {
        &self.pb7_over
    }
    #[doc = "0xc0 - This is the overide configuration register for each pad. PC0 has high drive capability."]
    #[inline(always)]
    pub const fn pc0_over(&self) -> &PC0_OVER {
        &self.pc0_over
    }
    #[doc = "0xc4 - This is the overide configuration register for each pad. PC1 has high drive capability."]
    #[inline(always)]
    pub const fn pc1_over(&self) -> &PC1_OVER {
        &self.pc1_over
    }
    #[doc = "0xc8 - This is the overide configuration register for each pad. PC2 has high drive capability."]
    #[inline(always)]
    pub const fn pc2_over(&self) -> &PC2_OVER {
        &self.pc2_over
    }
    #[doc = "0xcc - This is the overide configuration register for each pad. PC3 has high drive capability."]
    #[inline(always)]
    pub const fn pc3_over(&self) -> &PC3_OVER {
        &self.pc3_over
    }
    #[doc = "0xd0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pc4_over(&self) -> &PC4_OVER {
        &self.pc4_over
    }
    #[doc = "0xd4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pc5_over(&self) -> &PC5_OVER {
        &self.pc5_over
    }
    #[doc = "0xd8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pc6_over(&self) -> &PC6_OVER {
        &self.pc6_over
    }
    #[doc = "0xdc - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pc7_over(&self) -> &PC7_OVER {
        &self.pc7_over
    }
    #[doc = "0xe0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd0_over(&self) -> &PD0_OVER {
        &self.pd0_over
    }
    #[doc = "0xe4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd1_over(&self) -> &PD1_OVER {
        &self.pd1_over
    }
    #[doc = "0xe8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd2_over(&self) -> &PD2_OVER {
        &self.pd2_over
    }
    #[doc = "0xec - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd3_over(&self) -> &PD3_OVER {
        &self.pd3_over
    }
    #[doc = "0xf0 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd4_over(&self) -> &PD4_OVER {
        &self.pd4_over
    }
    #[doc = "0xf4 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd5_over(&self) -> &PD5_OVER {
        &self.pd5_over
    }
    #[doc = "0xf8 - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd6_over(&self) -> &PD6_OVER {
        &self.pd6_over
    }
    #[doc = "0xfc - This is the overide configuration register for each pad."]
    #[inline(always)]
    pub const fn pd7_over(&self) -> &PD7_OVER {
        &self.pd7_over
    }
    #[doc = "0x100 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX."]
    #[inline(always)]
    pub const fn uartrxd_uart0(&self) -> &UARTRXD_UART0 {
        &self.uartrxd_uart0
    }
    #[doc = "0x104 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS."]
    #[inline(always)]
    pub const fn uartcts_uart1(&self) -> &UARTCTS_UART1 {
        &self.uartcts_uart1
    }
    #[doc = "0x108 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX."]
    #[inline(always)]
    pub const fn uartrxd_uart1(&self) -> &UARTRXD_UART1 {
        &self.uartrxd_uart1
    }
    #[doc = "0x10c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK."]
    #[inline(always)]
    pub const fn clk_ssi_ssi0(&self) -> &CLK_SSI_SSI0 {
        &self.clk_ssi_ssi0
    }
    #[doc = "0x110 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX."]
    #[inline(always)]
    pub const fn ssirxd_ssi0(&self) -> &SSIRXD_SSI0 {
        &self.ssirxd_ssi0
    }
    #[doc = "0x114 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN."]
    #[inline(always)]
    pub const fn ssifssin_ssi0(&self) -> &SSIFSSIN_SSI0 {
        &self.ssifssin_ssi0
    }
    #[doc = "0x118 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN."]
    #[inline(always)]
    pub const fn clk_ssiin_ssi0(&self) -> &CLK_SSIIN_SSI0 {
        &self.clk_ssiin_ssi0
    }
    #[doc = "0x11c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK."]
    #[inline(always)]
    pub const fn clk_ssi_ssi1(&self) -> &CLK_SSI_SSI1 {
        &self.clk_ssi_ssi1
    }
    #[doc = "0x120 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX."]
    #[inline(always)]
    pub const fn ssirxd_ssi1(&self) -> &SSIRXD_SSI1 {
        &self.ssirxd_ssi1
    }
    #[doc = "0x124 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN."]
    #[inline(always)]
    pub const fn ssifssin_ssi1(&self) -> &SSIFSSIN_SSI1 {
        &self.ssifssin_ssi1
    }
    #[doc = "0x128 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN."]
    #[inline(always)]
    pub const fn clk_ssiin_ssi1(&self) -> &CLK_SSIIN_SSI1 {
        &self.clk_ssiin_ssi1
    }
    #[doc = "0x12c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA."]
    #[inline(always)]
    pub const fn i2cmssda(&self) -> &I2CMSSDA {
        &self.i2cmssda
    }
    #[doc = "0x130 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL."]
    #[inline(always)]
    pub const fn i2cmsscl(&self) -> &I2CMSSCL {
        &self.i2cmsscl
    }
    #[doc = "0x134 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1."]
    #[inline(always)]
    pub const fn gpt0ocp1(&self) -> &GPT0OCP1 {
        &self.gpt0ocp1
    }
    #[doc = "0x138 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2."]
    #[inline(always)]
    pub const fn gpt0ocp2(&self) -> &GPT0OCP2 {
        &self.gpt0ocp2
    }
    #[doc = "0x13c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1."]
    #[inline(always)]
    pub const fn gpt1ocp1(&self) -> &GPT1OCP1 {
        &self.gpt1ocp1
    }
    #[doc = "0x140 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2."]
    #[inline(always)]
    pub const fn gpt1ocp2(&self) -> &GPT1OCP2 {
        &self.gpt1ocp2
    }
    #[doc = "0x144 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1."]
    #[inline(always)]
    pub const fn gpt2ocp1(&self) -> &GPT2OCP1 {
        &self.gpt2ocp1
    }
    #[doc = "0x148 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2."]
    #[inline(always)]
    pub const fn gpt2ocp2(&self) -> &GPT2OCP2 {
        &self.gpt2ocp2
    }
    #[doc = "0x14c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1."]
    #[inline(always)]
    pub const fn gpt3ocp1(&self) -> &GPT3OCP1 {
        &self.gpt3ocp1
    }
    #[doc = "0x150 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2."]
    #[inline(always)]
    pub const fn gpt3ocp2(&self) -> &GPT3OCP2 {
        &self.gpt3ocp2
    }
}
#[doc = "PA0_SEL (rw) register accessor: Peripheral select control for PA0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa0_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa0_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa0_sel`]
module"]
pub type PA0_SEL = crate::Reg<pa0_sel::PA0_SEL_SPEC>;
#[doc = "Peripheral select control for PA0"]
pub mod pa0_sel;
#[doc = "PA1_SEL (rw) register accessor: Peripheral select control for PA1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa1_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa1_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa1_sel`]
module"]
pub type PA1_SEL = crate::Reg<pa1_sel::PA1_SEL_SPEC>;
#[doc = "Peripheral select control for PA1"]
pub mod pa1_sel;
#[doc = "PA2_SEL (rw) register accessor: Peripheral select control for PA2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa2_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa2_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa2_sel`]
module"]
pub type PA2_SEL = crate::Reg<pa2_sel::PA2_SEL_SPEC>;
#[doc = "Peripheral select control for PA2"]
pub mod pa2_sel;
#[doc = "PA3_SEL (rw) register accessor: Peripheral select control for PA3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa3_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa3_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa3_sel`]
module"]
pub type PA3_SEL = crate::Reg<pa3_sel::PA3_SEL_SPEC>;
#[doc = "Peripheral select control for PA3"]
pub mod pa3_sel;
#[doc = "PA4_SEL (rw) register accessor: Peripheral select control for PA4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa4_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa4_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa4_sel`]
module"]
pub type PA4_SEL = crate::Reg<pa4_sel::PA4_SEL_SPEC>;
#[doc = "Peripheral select control for PA4"]
pub mod pa4_sel;
#[doc = "PA5_SEL (rw) register accessor: Peripheral select control for PA5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa5_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa5_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa5_sel`]
module"]
pub type PA5_SEL = crate::Reg<pa5_sel::PA5_SEL_SPEC>;
#[doc = "Peripheral select control for PA5"]
pub mod pa5_sel;
#[doc = "PA6_SEL (rw) register accessor: Peripheral select control for PA6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa6_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa6_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa6_sel`]
module"]
pub type PA6_SEL = crate::Reg<pa6_sel::PA6_SEL_SPEC>;
#[doc = "Peripheral select control for PA6"]
pub mod pa6_sel;
#[doc = "PA7_SEL (rw) register accessor: Peripheral select control for PA7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa7_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa7_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa7_sel`]
module"]
pub type PA7_SEL = crate::Reg<pa7_sel::PA7_SEL_SPEC>;
#[doc = "Peripheral select control for PA7"]
pub mod pa7_sel;
#[doc = "PB0_SEL (rw) register accessor: Peripheral select control for PB0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb0_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb0_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb0_sel`]
module"]
pub type PB0_SEL = crate::Reg<pb0_sel::PB0_SEL_SPEC>;
#[doc = "Peripheral select control for PB0"]
pub mod pb0_sel;
#[doc = "PB1_SEL (rw) register accessor: Peripheral select control for PB1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb1_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb1_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb1_sel`]
module"]
pub type PB1_SEL = crate::Reg<pb1_sel::PB1_SEL_SPEC>;
#[doc = "Peripheral select control for PB1"]
pub mod pb1_sel;
#[doc = "PB2_SEL (rw) register accessor: Peripheral select control for PB2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb2_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb2_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb2_sel`]
module"]
pub type PB2_SEL = crate::Reg<pb2_sel::PB2_SEL_SPEC>;
#[doc = "Peripheral select control for PB2"]
pub mod pb2_sel;
#[doc = "PB3_SEL (rw) register accessor: Peripheral select control for PB3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb3_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb3_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb3_sel`]
module"]
pub type PB3_SEL = crate::Reg<pb3_sel::PB3_SEL_SPEC>;
#[doc = "Peripheral select control for PB3"]
pub mod pb3_sel;
#[doc = "PB4_SEL (rw) register accessor: Peripheral select control for PB4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb4_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb4_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb4_sel`]
module"]
pub type PB4_SEL = crate::Reg<pb4_sel::PB4_SEL_SPEC>;
#[doc = "Peripheral select control for PB4"]
pub mod pb4_sel;
#[doc = "PB5_SEL (rw) register accessor: Peripheral select control for PB5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb5_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb5_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb5_sel`]
module"]
pub type PB5_SEL = crate::Reg<pb5_sel::PB5_SEL_SPEC>;
#[doc = "Peripheral select control for PB5"]
pub mod pb5_sel;
#[doc = "PB6_SEL (rw) register accessor: Peripheral select control for PB6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb6_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb6_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb6_sel`]
module"]
pub type PB6_SEL = crate::Reg<pb6_sel::PB6_SEL_SPEC>;
#[doc = "Peripheral select control for PB6"]
pub mod pb6_sel;
#[doc = "PB7_SEL (rw) register accessor: Peripheral select control for PB7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb7_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb7_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb7_sel`]
module"]
pub type PB7_SEL = crate::Reg<pb7_sel::PB7_SEL_SPEC>;
#[doc = "Peripheral select control for PB7"]
pub mod pb7_sel;
#[doc = "PC0_SEL (rw) register accessor: Peripheral select control for PC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc0_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc0_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc0_sel`]
module"]
pub type PC0_SEL = crate::Reg<pc0_sel::PC0_SEL_SPEC>;
#[doc = "Peripheral select control for PC0"]
pub mod pc0_sel;
#[doc = "PC1_SEL (rw) register accessor: Peripheral select control for PC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc1_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc1_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc1_sel`]
module"]
pub type PC1_SEL = crate::Reg<pc1_sel::PC1_SEL_SPEC>;
#[doc = "Peripheral select control for PC1"]
pub mod pc1_sel;
#[doc = "PC2_SEL (rw) register accessor: Peripheral select control for PC2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc2_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc2_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc2_sel`]
module"]
pub type PC2_SEL = crate::Reg<pc2_sel::PC2_SEL_SPEC>;
#[doc = "Peripheral select control for PC2"]
pub mod pc2_sel;
#[doc = "PC3_SEL (rw) register accessor: Peripheral select control for PC3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc3_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc3_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc3_sel`]
module"]
pub type PC3_SEL = crate::Reg<pc3_sel::PC3_SEL_SPEC>;
#[doc = "Peripheral select control for PC3"]
pub mod pc3_sel;
#[doc = "PC4_SEL (rw) register accessor: Peripheral select control for PC4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc4_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc4_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc4_sel`]
module"]
pub type PC4_SEL = crate::Reg<pc4_sel::PC4_SEL_SPEC>;
#[doc = "Peripheral select control for PC4"]
pub mod pc4_sel;
#[doc = "PC5_SEL (rw) register accessor: Peripheral select control for PC5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc5_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc5_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc5_sel`]
module"]
pub type PC5_SEL = crate::Reg<pc5_sel::PC5_SEL_SPEC>;
#[doc = "Peripheral select control for PC5"]
pub mod pc5_sel;
#[doc = "PC6_SEL (rw) register accessor: Peripheral select control for PC6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc6_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc6_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc6_sel`]
module"]
pub type PC6_SEL = crate::Reg<pc6_sel::PC6_SEL_SPEC>;
#[doc = "Peripheral select control for PC6"]
pub mod pc6_sel;
#[doc = "PC7_SEL (rw) register accessor: Peripheral select control for PC7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc7_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc7_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc7_sel`]
module"]
pub type PC7_SEL = crate::Reg<pc7_sel::PC7_SEL_SPEC>;
#[doc = "Peripheral select control for PC7"]
pub mod pc7_sel;
#[doc = "PD0_SEL (rw) register accessor: Peripheral select control for PD0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd0_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd0_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd0_sel`]
module"]
pub type PD0_SEL = crate::Reg<pd0_sel::PD0_SEL_SPEC>;
#[doc = "Peripheral select control for PD0"]
pub mod pd0_sel;
#[doc = "PD1_SEL (rw) register accessor: Peripheral select control for PD1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd1_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd1_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd1_sel`]
module"]
pub type PD1_SEL = crate::Reg<pd1_sel::PD1_SEL_SPEC>;
#[doc = "Peripheral select control for PD1"]
pub mod pd1_sel;
#[doc = "PD2_SEL (rw) register accessor: Peripheral select control for PD2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd2_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd2_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd2_sel`]
module"]
pub type PD2_SEL = crate::Reg<pd2_sel::PD2_SEL_SPEC>;
#[doc = "Peripheral select control for PD2"]
pub mod pd2_sel;
#[doc = "PD3_SEL (rw) register accessor: Peripheral select control for PD3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd3_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd3_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd3_sel`]
module"]
pub type PD3_SEL = crate::Reg<pd3_sel::PD3_SEL_SPEC>;
#[doc = "Peripheral select control for PD3"]
pub mod pd3_sel;
#[doc = "PD4_SEL (rw) register accessor: Peripheral select control for PD4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd4_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd4_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd4_sel`]
module"]
pub type PD4_SEL = crate::Reg<pd4_sel::PD4_SEL_SPEC>;
#[doc = "Peripheral select control for PD4"]
pub mod pd4_sel;
#[doc = "PD5_SEL (rw) register accessor: Peripheral select control for PD5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd5_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd5_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd5_sel`]
module"]
pub type PD5_SEL = crate::Reg<pd5_sel::PD5_SEL_SPEC>;
#[doc = "Peripheral select control for PD5"]
pub mod pd5_sel;
#[doc = "PD6_SEL (rw) register accessor: Peripheral select control for PD6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd6_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd6_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd6_sel`]
module"]
pub type PD6_SEL = crate::Reg<pd6_sel::PD6_SEL_SPEC>;
#[doc = "Peripheral select control for PD6"]
pub mod pd6_sel;
#[doc = "PD7_SEL (rw) register accessor: Peripheral select control for PD7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd7_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd7_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd7_sel`]
module"]
pub type PD7_SEL = crate::Reg<pd7_sel::PD7_SEL_SPEC>;
#[doc = "Peripheral select control for PD7"]
pub mod pd7_sel;
#[doc = "PA0_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa0_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa0_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa0_over`]
module"]
pub type PA0_OVER = crate::Reg<pa0_over::PA0_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa0_over;
#[doc = "PA1_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa1_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa1_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa1_over`]
module"]
pub type PA1_OVER = crate::Reg<pa1_over::PA1_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa1_over;
#[doc = "PA2_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa2_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa2_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa2_over`]
module"]
pub type PA2_OVER = crate::Reg<pa2_over::PA2_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa2_over;
#[doc = "PA3_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa3_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa3_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa3_over`]
module"]
pub type PA3_OVER = crate::Reg<pa3_over::PA3_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa3_over;
#[doc = "PA4_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa4_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa4_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa4_over`]
module"]
pub type PA4_OVER = crate::Reg<pa4_over::PA4_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa4_over;
#[doc = "PA5_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa5_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa5_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa5_over`]
module"]
pub type PA5_OVER = crate::Reg<pa5_over::PA5_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa5_over;
#[doc = "PA6_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa6_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa6_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa6_over`]
module"]
pub type PA6_OVER = crate::Reg<pa6_over::PA6_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa6_over;
#[doc = "PA7_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa7_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa7_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa7_over`]
module"]
pub type PA7_OVER = crate::Reg<pa7_over::PA7_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa7_over;
#[doc = "PB0_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb0_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb0_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb0_over`]
module"]
pub type PB0_OVER = crate::Reg<pb0_over::PB0_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb0_over;
#[doc = "PB1_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb1_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb1_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb1_over`]
module"]
pub type PB1_OVER = crate::Reg<pb1_over::PB1_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb1_over;
#[doc = "PB2_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb2_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb2_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb2_over`]
module"]
pub type PB2_OVER = crate::Reg<pb2_over::PB2_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb2_over;
#[doc = "PB3_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb3_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb3_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb3_over`]
module"]
pub type PB3_OVER = crate::Reg<pb3_over::PB3_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb3_over;
#[doc = "PB4_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb4_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb4_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb4_over`]
module"]
pub type PB4_OVER = crate::Reg<pb4_over::PB4_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb4_over;
#[doc = "PB5_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb5_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb5_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb5_over`]
module"]
pub type PB5_OVER = crate::Reg<pb5_over::PB5_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb5_over;
#[doc = "PB6_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb6_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb6_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb6_over`]
module"]
pub type PB6_OVER = crate::Reg<pb6_over::PB6_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb6_over;
#[doc = "PB7_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb7_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb7_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb7_over`]
module"]
pub type PB7_OVER = crate::Reg<pb7_over::PB7_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb7_over;
#[doc = "PC0_OVER (rw) register accessor: This is the overide configuration register for each pad. PC0 has high drive capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc0_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc0_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc0_over`]
module"]
pub type PC0_OVER = crate::Reg<pc0_over::PC0_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad. PC0 has high drive capability."]
pub mod pc0_over;
#[doc = "PC1_OVER (rw) register accessor: This is the overide configuration register for each pad. PC1 has high drive capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc1_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc1_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc1_over`]
module"]
pub type PC1_OVER = crate::Reg<pc1_over::PC1_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad. PC1 has high drive capability."]
pub mod pc1_over;
#[doc = "PC2_OVER (rw) register accessor: This is the overide configuration register for each pad. PC2 has high drive capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc2_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc2_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc2_over`]
module"]
pub type PC2_OVER = crate::Reg<pc2_over::PC2_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad. PC2 has high drive capability."]
pub mod pc2_over;
#[doc = "PC3_OVER (rw) register accessor: This is the overide configuration register for each pad. PC3 has high drive capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc3_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc3_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc3_over`]
module"]
pub type PC3_OVER = crate::Reg<pc3_over::PC3_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad. PC3 has high drive capability."]
pub mod pc3_over;
#[doc = "PC4_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc4_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc4_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc4_over`]
module"]
pub type PC4_OVER = crate::Reg<pc4_over::PC4_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc4_over;
#[doc = "PC5_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc5_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc5_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc5_over`]
module"]
pub type PC5_OVER = crate::Reg<pc5_over::PC5_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc5_over;
#[doc = "PC6_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc6_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc6_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc6_over`]
module"]
pub type PC6_OVER = crate::Reg<pc6_over::PC6_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc6_over;
#[doc = "PC7_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc7_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc7_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc7_over`]
module"]
pub type PC7_OVER = crate::Reg<pc7_over::PC7_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc7_over;
#[doc = "PD0_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd0_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd0_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd0_over`]
module"]
pub type PD0_OVER = crate::Reg<pd0_over::PD0_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd0_over;
#[doc = "PD1_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd1_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd1_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd1_over`]
module"]
pub type PD1_OVER = crate::Reg<pd1_over::PD1_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd1_over;
#[doc = "PD2_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd2_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd2_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd2_over`]
module"]
pub type PD2_OVER = crate::Reg<pd2_over::PD2_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd2_over;
#[doc = "PD3_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd3_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd3_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd3_over`]
module"]
pub type PD3_OVER = crate::Reg<pd3_over::PD3_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd3_over;
#[doc = "PD4_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd4_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd4_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd4_over`]
module"]
pub type PD4_OVER = crate::Reg<pd4_over::PD4_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd4_over;
#[doc = "PD5_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd5_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd5_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd5_over`]
module"]
pub type PD5_OVER = crate::Reg<pd5_over::PD5_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd5_over;
#[doc = "PD6_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd6_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd6_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd6_over`]
module"]
pub type PD6_OVER = crate::Reg<pd6_over::PD6_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd6_over;
#[doc = "PD7_OVER (rw) register accessor: This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd7_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd7_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd7_over`]
module"]
pub type PD7_OVER = crate::Reg<pd7_over::PD7_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd7_over;
#[doc = "UARTRXD_UART0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartrxd_uart0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartrxd_uart0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartrxd_uart0`]
module"]
pub type UARTRXD_UART0 = crate::Reg<uartrxd_uart0::UARTRXD_UART0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX."]
pub mod uartrxd_uart0;
#[doc = "UARTCTS_UART1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartcts_uart1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartcts_uart1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartcts_uart1`]
module"]
pub type UARTCTS_UART1 = crate::Reg<uartcts_uart1::UARTCTS_UART1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS."]
pub mod uartcts_uart1;
#[doc = "UARTRXD_UART1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartrxd_uart1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartrxd_uart1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartrxd_uart1`]
module"]
pub type UARTRXD_UART1 = crate::Reg<uartrxd_uart1::UARTRXD_UART1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX."]
pub mod uartrxd_uart1;
#[doc = "CLK_SSI_SSI0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ssi_ssi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ssi_ssi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ssi_ssi0`]
module"]
pub type CLK_SSI_SSI0 = crate::Reg<clk_ssi_ssi0::CLK_SSI_SSI0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK."]
pub mod clk_ssi_ssi0;
#[doc = "SSIRXD_SSI0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssirxd_ssi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssirxd_ssi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssirxd_ssi0`]
module"]
pub type SSIRXD_SSI0 = crate::Reg<ssirxd_ssi0::SSIRXD_SSI0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX."]
pub mod ssirxd_ssi0;
#[doc = "SSIFSSIN_SSI0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssifssin_ssi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssifssin_ssi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifssin_ssi0`]
module"]
pub type SSIFSSIN_SSI0 = crate::Reg<ssifssin_ssi0::SSIFSSIN_SSI0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN."]
pub mod ssifssin_ssi0;
#[doc = "CLK_SSIIN_SSI0 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ssiin_ssi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ssiin_ssi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ssiin_ssi0`]
module"]
pub type CLK_SSIIN_SSI0 = crate::Reg<clk_ssiin_ssi0::CLK_SSIIN_SSI0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN."]
pub mod clk_ssiin_ssi0;
#[doc = "CLK_SSI_SSI1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ssi_ssi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ssi_ssi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ssi_ssi1`]
module"]
pub type CLK_SSI_SSI1 = crate::Reg<clk_ssi_ssi1::CLK_SSI_SSI1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK."]
pub mod clk_ssi_ssi1;
#[doc = "SSIRXD_SSI1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssirxd_ssi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssirxd_ssi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssirxd_ssi1`]
module"]
pub type SSIRXD_SSI1 = crate::Reg<ssirxd_ssi1::SSIRXD_SSI1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX."]
pub mod ssirxd_ssi1;
#[doc = "SSIFSSIN_SSI1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssifssin_ssi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssifssin_ssi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssifssin_ssi1`]
module"]
pub type SSIFSSIN_SSI1 = crate::Reg<ssifssin_ssi1::SSIFSSIN_SSI1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN."]
pub mod ssifssin_ssi1;
#[doc = "CLK_SSIIN_SSI1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ssiin_ssi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ssiin_ssi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ssiin_ssi1`]
module"]
pub type CLK_SSIIN_SSI1 = crate::Reg<clk_ssiin_ssi1::CLK_SSIIN_SSI1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN."]
pub mod clk_ssiin_ssi1;
#[doc = "I2CMSSDA (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cmssda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cmssda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cmssda`]
module"]
pub type I2CMSSDA = crate::Reg<i2cmssda::I2CMSSDA_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA."]
pub mod i2cmssda;
#[doc = "I2CMSSCL (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cmsscl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cmsscl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cmsscl`]
module"]
pub type I2CMSSCL = crate::Reg<i2cmsscl::I2CMSSCL_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL."]
pub mod i2cmsscl;
#[doc = "GPT0OCP1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt0ocp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt0ocp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt0ocp1`]
module"]
pub type GPT0OCP1 = crate::Reg<gpt0ocp1::GPT0OCP1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1."]
pub mod gpt0ocp1;
#[doc = "GPT0OCP2 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt0ocp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt0ocp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt0ocp2`]
module"]
pub type GPT0OCP2 = crate::Reg<gpt0ocp2::GPT0OCP2_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2."]
pub mod gpt0ocp2;
#[doc = "GPT1OCP1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt1ocp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt1ocp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt1ocp1`]
module"]
pub type GPT1OCP1 = crate::Reg<gpt1ocp1::GPT1OCP1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1."]
pub mod gpt1ocp1;
#[doc = "GPT1OCP2 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt1ocp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt1ocp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt1ocp2`]
module"]
pub type GPT1OCP2 = crate::Reg<gpt1ocp2::GPT1OCP2_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2."]
pub mod gpt1ocp2;
#[doc = "GPT2OCP1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt2ocp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt2ocp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt2ocp1`]
module"]
pub type GPT2OCP1 = crate::Reg<gpt2ocp1::GPT2OCP1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1."]
pub mod gpt2ocp1;
#[doc = "GPT2OCP2 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt2ocp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt2ocp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt2ocp2`]
module"]
pub type GPT2OCP2 = crate::Reg<gpt2ocp2::GPT2OCP2_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2."]
pub mod gpt2ocp2;
#[doc = "GPT3OCP1 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt3ocp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt3ocp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt3ocp1`]
module"]
pub type GPT3OCP1 = crate::Reg<gpt3ocp1::GPT3OCP1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1."]
pub mod gpt3ocp1;
#[doc = "GPT3OCP2 (rw) register accessor: Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt3ocp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt3ocp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt3ocp2`]
module"]
pub type GPT3OCP2 = crate::Reg<gpt3ocp2::GPT3OCP2_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2."]
pub mod gpt3ocp2;

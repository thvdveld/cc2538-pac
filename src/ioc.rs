#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral select control for PA0"]
    pub pa0_sel: crate::Reg<pa0_sel::PA0_SEL_SPEC>,
    #[doc = "0x04 - Peripheral select control for PA1"]
    pub pa1_sel: crate::Reg<pa1_sel::PA1_SEL_SPEC>,
    #[doc = "0x08 - Peripheral select control for PA2"]
    pub pa2_sel: crate::Reg<pa2_sel::PA2_SEL_SPEC>,
    #[doc = "0x0c - Peripheral select control for PA3"]
    pub pa3_sel: crate::Reg<pa3_sel::PA3_SEL_SPEC>,
    #[doc = "0x10 - Peripheral select control for PA4"]
    pub pa4_sel: crate::Reg<pa4_sel::PA4_SEL_SPEC>,
    #[doc = "0x14 - Peripheral select control for PA5"]
    pub pa5_sel: crate::Reg<pa5_sel::PA5_SEL_SPEC>,
    #[doc = "0x18 - Peripheral select control for PA6"]
    pub pa6_sel: crate::Reg<pa6_sel::PA6_SEL_SPEC>,
    #[doc = "0x1c - Peripheral select control for PA7"]
    pub pa7_sel: crate::Reg<pa7_sel::PA7_SEL_SPEC>,
    #[doc = "0x20 - Peripheral select control for PB0"]
    pub pb0_sel: crate::Reg<pb0_sel::PB0_SEL_SPEC>,
    #[doc = "0x24 - Peripheral select control for PB1"]
    pub pb1_sel: crate::Reg<pb1_sel::PB1_SEL_SPEC>,
    #[doc = "0x28 - Peripheral select control for PB2"]
    pub pb2_sel: crate::Reg<pb2_sel::PB2_SEL_SPEC>,
    #[doc = "0x2c - Peripheral select control for PB3"]
    pub pb3_sel: crate::Reg<pb3_sel::PB3_SEL_SPEC>,
    #[doc = "0x30 - Peripheral select control for PB4"]
    pub pb4_sel: crate::Reg<pb4_sel::PB4_SEL_SPEC>,
    #[doc = "0x34 - Peripheral select control for PB5"]
    pub pb5_sel: crate::Reg<pb5_sel::PB5_SEL_SPEC>,
    #[doc = "0x38 - Peripheral select control for PB6"]
    pub pb6_sel: crate::Reg<pb6_sel::PB6_SEL_SPEC>,
    #[doc = "0x3c - Peripheral select control for PB7"]
    pub pb7_sel: crate::Reg<pb7_sel::PB7_SEL_SPEC>,
    #[doc = "0x40 - Peripheral select control for PC0"]
    pub pc0_sel: crate::Reg<pc0_sel::PC0_SEL_SPEC>,
    #[doc = "0x44 - Peripheral select control for PC1"]
    pub pc1_sel: crate::Reg<pc1_sel::PC1_SEL_SPEC>,
    #[doc = "0x48 - Peripheral select control for PC2"]
    pub pc2_sel: crate::Reg<pc2_sel::PC2_SEL_SPEC>,
    #[doc = "0x4c - Peripheral select control for PC3"]
    pub pc3_sel: crate::Reg<pc3_sel::PC3_SEL_SPEC>,
    #[doc = "0x50 - Peripheral select control for PC4"]
    pub pc4_sel: crate::Reg<pc4_sel::PC4_SEL_SPEC>,
    #[doc = "0x54 - Peripheral select control for PC5"]
    pub pc5_sel: crate::Reg<pc5_sel::PC5_SEL_SPEC>,
    #[doc = "0x58 - Peripheral select control for PC6"]
    pub pc6_sel: crate::Reg<pc6_sel::PC6_SEL_SPEC>,
    #[doc = "0x5c - Peripheral select control for PC7"]
    pub pc7_sel: crate::Reg<pc7_sel::PC7_SEL_SPEC>,
    #[doc = "0x60 - Peripheral select control for PD0"]
    pub pd0_sel: crate::Reg<pd0_sel::PD0_SEL_SPEC>,
    #[doc = "0x64 - Peripheral select control for PD1"]
    pub pd1_sel: crate::Reg<pd1_sel::PD1_SEL_SPEC>,
    #[doc = "0x68 - Peripheral select control for PD2"]
    pub pd2_sel: crate::Reg<pd2_sel::PD2_SEL_SPEC>,
    #[doc = "0x6c - Peripheral select control for PD3"]
    pub pd3_sel: crate::Reg<pd3_sel::PD3_SEL_SPEC>,
    #[doc = "0x70 - Peripheral select control for PD4"]
    pub pd4_sel: crate::Reg<pd4_sel::PD4_SEL_SPEC>,
    #[doc = "0x74 - Peripheral select control for PD5"]
    pub pd5_sel: crate::Reg<pd5_sel::PD5_SEL_SPEC>,
    #[doc = "0x78 - Peripheral select control for PD6"]
    pub pd6_sel: crate::Reg<pd6_sel::PD6_SEL_SPEC>,
    #[doc = "0x7c - Peripheral select control for PD7"]
    pub pd7_sel: crate::Reg<pd7_sel::PD7_SEL_SPEC>,
    #[doc = "0x80 - This is the overide configuration register for each pad."]
    pub pa0_over: crate::Reg<pa0_over::PA0_OVER_SPEC>,
    #[doc = "0x84 - This is the overide configuration register for each pad."]
    pub pa1_over: crate::Reg<pa1_over::PA1_OVER_SPEC>,
    #[doc = "0x88 - This is the overide configuration register for each pad."]
    pub pa2_over: crate::Reg<pa2_over::PA2_OVER_SPEC>,
    #[doc = "0x8c - This is the overide configuration register for each pad."]
    pub pa3_over: crate::Reg<pa3_over::PA3_OVER_SPEC>,
    #[doc = "0x90 - This is the overide configuration register for each pad."]
    pub pa4_over: crate::Reg<pa4_over::PA4_OVER_SPEC>,
    #[doc = "0x94 - This is the overide configuration register for each pad."]
    pub pa5_over: crate::Reg<pa5_over::PA5_OVER_SPEC>,
    #[doc = "0x98 - This is the overide configuration register for each pad."]
    pub pa6_over: crate::Reg<pa6_over::PA6_OVER_SPEC>,
    #[doc = "0x9c - This is the overide configuration register for each pad."]
    pub pa7_over: crate::Reg<pa7_over::PA7_OVER_SPEC>,
    #[doc = "0xa0 - This is the overide configuration register for each pad."]
    pub pb0_over: crate::Reg<pb0_over::PB0_OVER_SPEC>,
    #[doc = "0xa4 - This is the overide configuration register for each pad."]
    pub pb1_over: crate::Reg<pb1_over::PB1_OVER_SPEC>,
    #[doc = "0xa8 - This is the overide configuration register for each pad."]
    pub pb2_over: crate::Reg<pb2_over::PB2_OVER_SPEC>,
    #[doc = "0xac - This is the overide configuration register for each pad."]
    pub pb3_over: crate::Reg<pb3_over::PB3_OVER_SPEC>,
    #[doc = "0xb0 - This is the overide configuration register for each pad."]
    pub pb4_over: crate::Reg<pb4_over::PB4_OVER_SPEC>,
    #[doc = "0xb4 - This is the overide configuration register for each pad."]
    pub pb5_over: crate::Reg<pb5_over::PB5_OVER_SPEC>,
    #[doc = "0xb8 - This is the overide configuration register for each pad."]
    pub pb6_over: crate::Reg<pb6_over::PB6_OVER_SPEC>,
    #[doc = "0xbc - This is the overide configuration register for each pad."]
    pub pb7_over: crate::Reg<pb7_over::PB7_OVER_SPEC>,
    #[doc = "0xc0 - This is the overide configuration register for each pad. PC0 has high drive capability."]
    pub pc0_over: crate::Reg<pc0_over::PC0_OVER_SPEC>,
    #[doc = "0xc4 - This is the overide configuration register for each pad. PC1 has high drive capability."]
    pub pc1_over: crate::Reg<pc1_over::PC1_OVER_SPEC>,
    #[doc = "0xc8 - This is the overide configuration register for each pad. PC2 has high drive capability."]
    pub pc2_over: crate::Reg<pc2_over::PC2_OVER_SPEC>,
    #[doc = "0xcc - This is the overide configuration register for each pad. PC3 has high drive capability."]
    pub pc3_over: crate::Reg<pc3_over::PC3_OVER_SPEC>,
    #[doc = "0xd0 - This is the overide configuration register for each pad."]
    pub pc4_over: crate::Reg<pc4_over::PC4_OVER_SPEC>,
    #[doc = "0xd4 - This is the overide configuration register for each pad."]
    pub pc5_over: crate::Reg<pc5_over::PC5_OVER_SPEC>,
    #[doc = "0xd8 - This is the overide configuration register for each pad."]
    pub pc6_over: crate::Reg<pc6_over::PC6_OVER_SPEC>,
    #[doc = "0xdc - This is the overide configuration register for each pad."]
    pub pc7_over: crate::Reg<pc7_over::PC7_OVER_SPEC>,
    #[doc = "0xe0 - This is the overide configuration register for each pad."]
    pub pd0_over: crate::Reg<pd0_over::PD0_OVER_SPEC>,
    #[doc = "0xe4 - This is the overide configuration register for each pad."]
    pub pd1_over: crate::Reg<pd1_over::PD1_OVER_SPEC>,
    #[doc = "0xe8 - This is the overide configuration register for each pad."]
    pub pd2_over: crate::Reg<pd2_over::PD2_OVER_SPEC>,
    #[doc = "0xec - This is the overide configuration register for each pad."]
    pub pd3_over: crate::Reg<pd3_over::PD3_OVER_SPEC>,
    #[doc = "0xf0 - This is the overide configuration register for each pad."]
    pub pd4_over: crate::Reg<pd4_over::PD4_OVER_SPEC>,
    #[doc = "0xf4 - This is the overide configuration register for each pad."]
    pub pd5_over: crate::Reg<pd5_over::PD5_OVER_SPEC>,
    #[doc = "0xf8 - This is the overide configuration register for each pad."]
    pub pd6_over: crate::Reg<pd6_over::PD6_OVER_SPEC>,
    #[doc = "0xfc - This is the overide configuration register for each pad."]
    pub pd7_over: crate::Reg<pd7_over::PD7_OVER_SPEC>,
    #[doc = "0x100 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX."]
    pub uartrxd_uart0: crate::Reg<uartrxd_uart0::UARTRXD_UART0_SPEC>,
    #[doc = "0x104 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS."]
    pub uartcts_uart1: crate::Reg<uartcts_uart1::UARTCTS_UART1_SPEC>,
    #[doc = "0x108 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX."]
    pub uartrxd_uart1: crate::Reg<uartrxd_uart1::UARTRXD_UART1_SPEC>,
    #[doc = "0x10c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK."]
    pub clk_ssi_ssi0: crate::Reg<clk_ssi_ssi0::CLK_SSI_SSI0_SPEC>,
    #[doc = "0x110 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX."]
    pub ssirxd_ssi0: crate::Reg<ssirxd_ssi0::SSIRXD_SSI0_SPEC>,
    #[doc = "0x114 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN."]
    pub ssifssin_ssi0: crate::Reg<ssifssin_ssi0::SSIFSSIN_SSI0_SPEC>,
    #[doc = "0x118 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN."]
    pub clk_ssiin_ssi0: crate::Reg<clk_ssiin_ssi0::CLK_SSIIN_SSI0_SPEC>,
    #[doc = "0x11c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK."]
    pub clk_ssi_ssi1: crate::Reg<clk_ssi_ssi1::CLK_SSI_SSI1_SPEC>,
    #[doc = "0x120 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX."]
    pub ssirxd_ssi1: crate::Reg<ssirxd_ssi1::SSIRXD_SSI1_SPEC>,
    #[doc = "0x124 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN."]
    pub ssifssin_ssi1: crate::Reg<ssifssin_ssi1::SSIFSSIN_SSI1_SPEC>,
    #[doc = "0x128 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN."]
    pub clk_ssiin_ssi1: crate::Reg<clk_ssiin_ssi1::CLK_SSIIN_SSI1_SPEC>,
    #[doc = "0x12c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA."]
    pub i2cmssda: crate::Reg<i2cmssda::I2CMSSDA_SPEC>,
    #[doc = "0x130 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL."]
    pub i2cmsscl: crate::Reg<i2cmsscl::I2CMSSCL_SPEC>,
    #[doc = "0x134 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1."]
    pub gpt0ocp1: crate::Reg<gpt0ocp1::GPT0OCP1_SPEC>,
    #[doc = "0x138 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2."]
    pub gpt0ocp2: crate::Reg<gpt0ocp2::GPT0OCP2_SPEC>,
    #[doc = "0x13c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1."]
    pub gpt1ocp1: crate::Reg<gpt1ocp1::GPT1OCP1_SPEC>,
    #[doc = "0x140 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2."]
    pub gpt1ocp2: crate::Reg<gpt1ocp2::GPT1OCP2_SPEC>,
    #[doc = "0x144 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1."]
    pub gpt2ocp1: crate::Reg<gpt2ocp1::GPT2OCP1_SPEC>,
    #[doc = "0x148 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2."]
    pub gpt2ocp2: crate::Reg<gpt2ocp2::GPT2OCP2_SPEC>,
    #[doc = "0x14c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1."]
    pub gpt3ocp1: crate::Reg<gpt3ocp1::GPT3OCP1_SPEC>,
    #[doc = "0x150 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2."]
    pub gpt3ocp2: crate::Reg<gpt3ocp2::GPT3OCP2_SPEC>,
}
#[doc = "PA0_SEL register accessor: an alias for `Reg<PA0_SEL_SPEC>`"]
pub type PA0_SEL = crate::Reg<pa0_sel::PA0_SEL_SPEC>;
#[doc = "Peripheral select control for PA0"]
pub mod pa0_sel;
#[doc = "PA1_SEL register accessor: an alias for `Reg<PA1_SEL_SPEC>`"]
pub type PA1_SEL = crate::Reg<pa1_sel::PA1_SEL_SPEC>;
#[doc = "Peripheral select control for PA1"]
pub mod pa1_sel;
#[doc = "PA2_SEL register accessor: an alias for `Reg<PA2_SEL_SPEC>`"]
pub type PA2_SEL = crate::Reg<pa2_sel::PA2_SEL_SPEC>;
#[doc = "Peripheral select control for PA2"]
pub mod pa2_sel;
#[doc = "PA3_SEL register accessor: an alias for `Reg<PA3_SEL_SPEC>`"]
pub type PA3_SEL = crate::Reg<pa3_sel::PA3_SEL_SPEC>;
#[doc = "Peripheral select control for PA3"]
pub mod pa3_sel;
#[doc = "PA4_SEL register accessor: an alias for `Reg<PA4_SEL_SPEC>`"]
pub type PA4_SEL = crate::Reg<pa4_sel::PA4_SEL_SPEC>;
#[doc = "Peripheral select control for PA4"]
pub mod pa4_sel;
#[doc = "PA5_SEL register accessor: an alias for `Reg<PA5_SEL_SPEC>`"]
pub type PA5_SEL = crate::Reg<pa5_sel::PA5_SEL_SPEC>;
#[doc = "Peripheral select control for PA5"]
pub mod pa5_sel;
#[doc = "PA6_SEL register accessor: an alias for `Reg<PA6_SEL_SPEC>`"]
pub type PA6_SEL = crate::Reg<pa6_sel::PA6_SEL_SPEC>;
#[doc = "Peripheral select control for PA6"]
pub mod pa6_sel;
#[doc = "PA7_SEL register accessor: an alias for `Reg<PA7_SEL_SPEC>`"]
pub type PA7_SEL = crate::Reg<pa7_sel::PA7_SEL_SPEC>;
#[doc = "Peripheral select control for PA7"]
pub mod pa7_sel;
#[doc = "PB0_SEL register accessor: an alias for `Reg<PB0_SEL_SPEC>`"]
pub type PB0_SEL = crate::Reg<pb0_sel::PB0_SEL_SPEC>;
#[doc = "Peripheral select control for PB0"]
pub mod pb0_sel;
#[doc = "PB1_SEL register accessor: an alias for `Reg<PB1_SEL_SPEC>`"]
pub type PB1_SEL = crate::Reg<pb1_sel::PB1_SEL_SPEC>;
#[doc = "Peripheral select control for PB1"]
pub mod pb1_sel;
#[doc = "PB2_SEL register accessor: an alias for `Reg<PB2_SEL_SPEC>`"]
pub type PB2_SEL = crate::Reg<pb2_sel::PB2_SEL_SPEC>;
#[doc = "Peripheral select control for PB2"]
pub mod pb2_sel;
#[doc = "PB3_SEL register accessor: an alias for `Reg<PB3_SEL_SPEC>`"]
pub type PB3_SEL = crate::Reg<pb3_sel::PB3_SEL_SPEC>;
#[doc = "Peripheral select control for PB3"]
pub mod pb3_sel;
#[doc = "PB4_SEL register accessor: an alias for `Reg<PB4_SEL_SPEC>`"]
pub type PB4_SEL = crate::Reg<pb4_sel::PB4_SEL_SPEC>;
#[doc = "Peripheral select control for PB4"]
pub mod pb4_sel;
#[doc = "PB5_SEL register accessor: an alias for `Reg<PB5_SEL_SPEC>`"]
pub type PB5_SEL = crate::Reg<pb5_sel::PB5_SEL_SPEC>;
#[doc = "Peripheral select control for PB5"]
pub mod pb5_sel;
#[doc = "PB6_SEL register accessor: an alias for `Reg<PB6_SEL_SPEC>`"]
pub type PB6_SEL = crate::Reg<pb6_sel::PB6_SEL_SPEC>;
#[doc = "Peripheral select control for PB6"]
pub mod pb6_sel;
#[doc = "PB7_SEL register accessor: an alias for `Reg<PB7_SEL_SPEC>`"]
pub type PB7_SEL = crate::Reg<pb7_sel::PB7_SEL_SPEC>;
#[doc = "Peripheral select control for PB7"]
pub mod pb7_sel;
#[doc = "PC0_SEL register accessor: an alias for `Reg<PC0_SEL_SPEC>`"]
pub type PC0_SEL = crate::Reg<pc0_sel::PC0_SEL_SPEC>;
#[doc = "Peripheral select control for PC0"]
pub mod pc0_sel;
#[doc = "PC1_SEL register accessor: an alias for `Reg<PC1_SEL_SPEC>`"]
pub type PC1_SEL = crate::Reg<pc1_sel::PC1_SEL_SPEC>;
#[doc = "Peripheral select control for PC1"]
pub mod pc1_sel;
#[doc = "PC2_SEL register accessor: an alias for `Reg<PC2_SEL_SPEC>`"]
pub type PC2_SEL = crate::Reg<pc2_sel::PC2_SEL_SPEC>;
#[doc = "Peripheral select control for PC2"]
pub mod pc2_sel;
#[doc = "PC3_SEL register accessor: an alias for `Reg<PC3_SEL_SPEC>`"]
pub type PC3_SEL = crate::Reg<pc3_sel::PC3_SEL_SPEC>;
#[doc = "Peripheral select control for PC3"]
pub mod pc3_sel;
#[doc = "PC4_SEL register accessor: an alias for `Reg<PC4_SEL_SPEC>`"]
pub type PC4_SEL = crate::Reg<pc4_sel::PC4_SEL_SPEC>;
#[doc = "Peripheral select control for PC4"]
pub mod pc4_sel;
#[doc = "PC5_SEL register accessor: an alias for `Reg<PC5_SEL_SPEC>`"]
pub type PC5_SEL = crate::Reg<pc5_sel::PC5_SEL_SPEC>;
#[doc = "Peripheral select control for PC5"]
pub mod pc5_sel;
#[doc = "PC6_SEL register accessor: an alias for `Reg<PC6_SEL_SPEC>`"]
pub type PC6_SEL = crate::Reg<pc6_sel::PC6_SEL_SPEC>;
#[doc = "Peripheral select control for PC6"]
pub mod pc6_sel;
#[doc = "PC7_SEL register accessor: an alias for `Reg<PC7_SEL_SPEC>`"]
pub type PC7_SEL = crate::Reg<pc7_sel::PC7_SEL_SPEC>;
#[doc = "Peripheral select control for PC7"]
pub mod pc7_sel;
#[doc = "PD0_SEL register accessor: an alias for `Reg<PD0_SEL_SPEC>`"]
pub type PD0_SEL = crate::Reg<pd0_sel::PD0_SEL_SPEC>;
#[doc = "Peripheral select control for PD0"]
pub mod pd0_sel;
#[doc = "PD1_SEL register accessor: an alias for `Reg<PD1_SEL_SPEC>`"]
pub type PD1_SEL = crate::Reg<pd1_sel::PD1_SEL_SPEC>;
#[doc = "Peripheral select control for PD1"]
pub mod pd1_sel;
#[doc = "PD2_SEL register accessor: an alias for `Reg<PD2_SEL_SPEC>`"]
pub type PD2_SEL = crate::Reg<pd2_sel::PD2_SEL_SPEC>;
#[doc = "Peripheral select control for PD2"]
pub mod pd2_sel;
#[doc = "PD3_SEL register accessor: an alias for `Reg<PD3_SEL_SPEC>`"]
pub type PD3_SEL = crate::Reg<pd3_sel::PD3_SEL_SPEC>;
#[doc = "Peripheral select control for PD3"]
pub mod pd3_sel;
#[doc = "PD4_SEL register accessor: an alias for `Reg<PD4_SEL_SPEC>`"]
pub type PD4_SEL = crate::Reg<pd4_sel::PD4_SEL_SPEC>;
#[doc = "Peripheral select control for PD4"]
pub mod pd4_sel;
#[doc = "PD5_SEL register accessor: an alias for `Reg<PD5_SEL_SPEC>`"]
pub type PD5_SEL = crate::Reg<pd5_sel::PD5_SEL_SPEC>;
#[doc = "Peripheral select control for PD5"]
pub mod pd5_sel;
#[doc = "PD6_SEL register accessor: an alias for `Reg<PD6_SEL_SPEC>`"]
pub type PD6_SEL = crate::Reg<pd6_sel::PD6_SEL_SPEC>;
#[doc = "Peripheral select control for PD6"]
pub mod pd6_sel;
#[doc = "PD7_SEL register accessor: an alias for `Reg<PD7_SEL_SPEC>`"]
pub type PD7_SEL = crate::Reg<pd7_sel::PD7_SEL_SPEC>;
#[doc = "Peripheral select control for PD7"]
pub mod pd7_sel;
#[doc = "PA0_OVER register accessor: an alias for `Reg<PA0_OVER_SPEC>`"]
pub type PA0_OVER = crate::Reg<pa0_over::PA0_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa0_over;
#[doc = "PA1_OVER register accessor: an alias for `Reg<PA1_OVER_SPEC>`"]
pub type PA1_OVER = crate::Reg<pa1_over::PA1_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa1_over;
#[doc = "PA2_OVER register accessor: an alias for `Reg<PA2_OVER_SPEC>`"]
pub type PA2_OVER = crate::Reg<pa2_over::PA2_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa2_over;
#[doc = "PA3_OVER register accessor: an alias for `Reg<PA3_OVER_SPEC>`"]
pub type PA3_OVER = crate::Reg<pa3_over::PA3_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa3_over;
#[doc = "PA4_OVER register accessor: an alias for `Reg<PA4_OVER_SPEC>`"]
pub type PA4_OVER = crate::Reg<pa4_over::PA4_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa4_over;
#[doc = "PA5_OVER register accessor: an alias for `Reg<PA5_OVER_SPEC>`"]
pub type PA5_OVER = crate::Reg<pa5_over::PA5_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa5_over;
#[doc = "PA6_OVER register accessor: an alias for `Reg<PA6_OVER_SPEC>`"]
pub type PA6_OVER = crate::Reg<pa6_over::PA6_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa6_over;
#[doc = "PA7_OVER register accessor: an alias for `Reg<PA7_OVER_SPEC>`"]
pub type PA7_OVER = crate::Reg<pa7_over::PA7_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pa7_over;
#[doc = "PB0_OVER register accessor: an alias for `Reg<PB0_OVER_SPEC>`"]
pub type PB0_OVER = crate::Reg<pb0_over::PB0_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb0_over;
#[doc = "PB1_OVER register accessor: an alias for `Reg<PB1_OVER_SPEC>`"]
pub type PB1_OVER = crate::Reg<pb1_over::PB1_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb1_over;
#[doc = "PB2_OVER register accessor: an alias for `Reg<PB2_OVER_SPEC>`"]
pub type PB2_OVER = crate::Reg<pb2_over::PB2_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb2_over;
#[doc = "PB3_OVER register accessor: an alias for `Reg<PB3_OVER_SPEC>`"]
pub type PB3_OVER = crate::Reg<pb3_over::PB3_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb3_over;
#[doc = "PB4_OVER register accessor: an alias for `Reg<PB4_OVER_SPEC>`"]
pub type PB4_OVER = crate::Reg<pb4_over::PB4_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb4_over;
#[doc = "PB5_OVER register accessor: an alias for `Reg<PB5_OVER_SPEC>`"]
pub type PB5_OVER = crate::Reg<pb5_over::PB5_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb5_over;
#[doc = "PB6_OVER register accessor: an alias for `Reg<PB6_OVER_SPEC>`"]
pub type PB6_OVER = crate::Reg<pb6_over::PB6_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb6_over;
#[doc = "PB7_OVER register accessor: an alias for `Reg<PB7_OVER_SPEC>`"]
pub type PB7_OVER = crate::Reg<pb7_over::PB7_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pb7_over;
#[doc = "PC0_OVER register accessor: an alias for `Reg<PC0_OVER_SPEC>`"]
pub type PC0_OVER = crate::Reg<pc0_over::PC0_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad. PC0 has high drive capability."]
pub mod pc0_over;
#[doc = "PC1_OVER register accessor: an alias for `Reg<PC1_OVER_SPEC>`"]
pub type PC1_OVER = crate::Reg<pc1_over::PC1_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad. PC1 has high drive capability."]
pub mod pc1_over;
#[doc = "PC2_OVER register accessor: an alias for `Reg<PC2_OVER_SPEC>`"]
pub type PC2_OVER = crate::Reg<pc2_over::PC2_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad. PC2 has high drive capability."]
pub mod pc2_over;
#[doc = "PC3_OVER register accessor: an alias for `Reg<PC3_OVER_SPEC>`"]
pub type PC3_OVER = crate::Reg<pc3_over::PC3_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad. PC3 has high drive capability."]
pub mod pc3_over;
#[doc = "PC4_OVER register accessor: an alias for `Reg<PC4_OVER_SPEC>`"]
pub type PC4_OVER = crate::Reg<pc4_over::PC4_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc4_over;
#[doc = "PC5_OVER register accessor: an alias for `Reg<PC5_OVER_SPEC>`"]
pub type PC5_OVER = crate::Reg<pc5_over::PC5_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc5_over;
#[doc = "PC6_OVER register accessor: an alias for `Reg<PC6_OVER_SPEC>`"]
pub type PC6_OVER = crate::Reg<pc6_over::PC6_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc6_over;
#[doc = "PC7_OVER register accessor: an alias for `Reg<PC7_OVER_SPEC>`"]
pub type PC7_OVER = crate::Reg<pc7_over::PC7_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pc7_over;
#[doc = "PD0_OVER register accessor: an alias for `Reg<PD0_OVER_SPEC>`"]
pub type PD0_OVER = crate::Reg<pd0_over::PD0_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd0_over;
#[doc = "PD1_OVER register accessor: an alias for `Reg<PD1_OVER_SPEC>`"]
pub type PD1_OVER = crate::Reg<pd1_over::PD1_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd1_over;
#[doc = "PD2_OVER register accessor: an alias for `Reg<PD2_OVER_SPEC>`"]
pub type PD2_OVER = crate::Reg<pd2_over::PD2_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd2_over;
#[doc = "PD3_OVER register accessor: an alias for `Reg<PD3_OVER_SPEC>`"]
pub type PD3_OVER = crate::Reg<pd3_over::PD3_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd3_over;
#[doc = "PD4_OVER register accessor: an alias for `Reg<PD4_OVER_SPEC>`"]
pub type PD4_OVER = crate::Reg<pd4_over::PD4_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd4_over;
#[doc = "PD5_OVER register accessor: an alias for `Reg<PD5_OVER_SPEC>`"]
pub type PD5_OVER = crate::Reg<pd5_over::PD5_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd5_over;
#[doc = "PD6_OVER register accessor: an alias for `Reg<PD6_OVER_SPEC>`"]
pub type PD6_OVER = crate::Reg<pd6_over::PD6_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd6_over;
#[doc = "PD7_OVER register accessor: an alias for `Reg<PD7_OVER_SPEC>`"]
pub type PD7_OVER = crate::Reg<pd7_over::PD7_OVER_SPEC>;
#[doc = "This is the overide configuration register for each pad."]
pub mod pd7_over;
#[doc = "UARTRXD_UART0 register accessor: an alias for `Reg<UARTRXD_UART0_SPEC>`"]
pub type UARTRXD_UART0 = crate::Reg<uartrxd_uart0::UARTRXD_UART0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX."]
pub mod uartrxd_uart0;
#[doc = "UARTCTS_UART1 register accessor: an alias for `Reg<UARTCTS_UART1_SPEC>`"]
pub type UARTCTS_UART1 = crate::Reg<uartcts_uart1::UARTCTS_UART1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS."]
pub mod uartcts_uart1;
#[doc = "UARTRXD_UART1 register accessor: an alias for `Reg<UARTRXD_UART1_SPEC>`"]
pub type UARTRXD_UART1 = crate::Reg<uartrxd_uart1::UARTRXD_UART1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX."]
pub mod uartrxd_uart1;
#[doc = "CLK_SSI_SSI0 register accessor: an alias for `Reg<CLK_SSI_SSI0_SPEC>`"]
pub type CLK_SSI_SSI0 = crate::Reg<clk_ssi_ssi0::CLK_SSI_SSI0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK."]
pub mod clk_ssi_ssi0;
#[doc = "SSIRXD_SSI0 register accessor: an alias for `Reg<SSIRXD_SSI0_SPEC>`"]
pub type SSIRXD_SSI0 = crate::Reg<ssirxd_ssi0::SSIRXD_SSI0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX."]
pub mod ssirxd_ssi0;
#[doc = "SSIFSSIN_SSI0 register accessor: an alias for `Reg<SSIFSSIN_SSI0_SPEC>`"]
pub type SSIFSSIN_SSI0 = crate::Reg<ssifssin_ssi0::SSIFSSIN_SSI0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN."]
pub mod ssifssin_ssi0;
#[doc = "CLK_SSIIN_SSI0 register accessor: an alias for `Reg<CLK_SSIIN_SSI0_SPEC>`"]
pub type CLK_SSIIN_SSI0 = crate::Reg<clk_ssiin_ssi0::CLK_SSIIN_SSI0_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN."]
pub mod clk_ssiin_ssi0;
#[doc = "CLK_SSI_SSI1 register accessor: an alias for `Reg<CLK_SSI_SSI1_SPEC>`"]
pub type CLK_SSI_SSI1 = crate::Reg<clk_ssi_ssi1::CLK_SSI_SSI1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK."]
pub mod clk_ssi_ssi1;
#[doc = "SSIRXD_SSI1 register accessor: an alias for `Reg<SSIRXD_SSI1_SPEC>`"]
pub type SSIRXD_SSI1 = crate::Reg<ssirxd_ssi1::SSIRXD_SSI1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX."]
pub mod ssirxd_ssi1;
#[doc = "SSIFSSIN_SSI1 register accessor: an alias for `Reg<SSIFSSIN_SSI1_SPEC>`"]
pub type SSIFSSIN_SSI1 = crate::Reg<ssifssin_ssi1::SSIFSSIN_SSI1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN."]
pub mod ssifssin_ssi1;
#[doc = "CLK_SSIIN_SSI1 register accessor: an alias for `Reg<CLK_SSIIN_SSI1_SPEC>`"]
pub type CLK_SSIIN_SSI1 = crate::Reg<clk_ssiin_ssi1::CLK_SSIIN_SSI1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN."]
pub mod clk_ssiin_ssi1;
#[doc = "I2CMSSDA register accessor: an alias for `Reg<I2CMSSDA_SPEC>`"]
pub type I2CMSSDA = crate::Reg<i2cmssda::I2CMSSDA_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA."]
pub mod i2cmssda;
#[doc = "I2CMSSCL register accessor: an alias for `Reg<I2CMSSCL_SPEC>`"]
pub type I2CMSSCL = crate::Reg<i2cmsscl::I2CMSSCL_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL."]
pub mod i2cmsscl;
#[doc = "GPT0OCP1 register accessor: an alias for `Reg<GPT0OCP1_SPEC>`"]
pub type GPT0OCP1 = crate::Reg<gpt0ocp1::GPT0OCP1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1."]
pub mod gpt0ocp1;
#[doc = "GPT0OCP2 register accessor: an alias for `Reg<GPT0OCP2_SPEC>`"]
pub type GPT0OCP2 = crate::Reg<gpt0ocp2::GPT0OCP2_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2."]
pub mod gpt0ocp2;
#[doc = "GPT1OCP1 register accessor: an alias for `Reg<GPT1OCP1_SPEC>`"]
pub type GPT1OCP1 = crate::Reg<gpt1ocp1::GPT1OCP1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1."]
pub mod gpt1ocp1;
#[doc = "GPT1OCP2 register accessor: an alias for `Reg<GPT1OCP2_SPEC>`"]
pub type GPT1OCP2 = crate::Reg<gpt1ocp2::GPT1OCP2_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2."]
pub mod gpt1ocp2;
#[doc = "GPT2OCP1 register accessor: an alias for `Reg<GPT2OCP1_SPEC>`"]
pub type GPT2OCP1 = crate::Reg<gpt2ocp1::GPT2OCP1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1."]
pub mod gpt2ocp1;
#[doc = "GPT2OCP2 register accessor: an alias for `Reg<GPT2OCP2_SPEC>`"]
pub type GPT2OCP2 = crate::Reg<gpt2ocp2::GPT2OCP2_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2."]
pub mod gpt2ocp2;
#[doc = "GPT3OCP1 register accessor: an alias for `Reg<GPT3OCP1_SPEC>`"]
pub type GPT3OCP1 = crate::Reg<gpt3ocp1::GPT3OCP1_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1."]
pub mod gpt3ocp1;
#[doc = "GPT3OCP2 register accessor: an alias for `Reg<GPT3OCP2_SPEC>`"]
pub type GPT3OCP2 = crate::Reg<gpt3ocp2::GPT3OCP2_SPEC>;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2."]
pub mod gpt3ocp2;

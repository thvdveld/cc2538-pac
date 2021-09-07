#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
    pub clock_ctrl: crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>,
    #[doc = "0x04 - Clock status register This register reflects the current chip status."]
    pub clock_sta: crate::Reg<clock_sta::CLOCK_STA_SPEC>,
    #[doc = "0x08 - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcgpt: crate::Reg<rcgcgpt::RCGCGPT_SPEC>,
    #[doc = "0x0c - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcgpt: crate::Reg<scgcgpt::SCGCGPT_SPEC>,
    #[doc = "0x10 - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcgpt: crate::Reg<dcgcgpt::DCGCGPT_SPEC>,
    #[doc = "0x14 - This register controls the reset for GPT\\[3:0\\]."]
    pub srgpt: crate::Reg<srgpt::SRGPT_SPEC>,
    #[doc = "0x18 - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcssi: crate::Reg<rcgcssi::RCGCSSI_SPEC>,
    #[doc = "0x1c - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcssi: crate::Reg<scgcssi::SCGCSSI_SPEC>,
    #[doc = "0x20 - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcssi: crate::Reg<dcgcssi::DCGCSSI_SPEC>,
    #[doc = "0x24 - This register controls the reset for SSI\\[1:0\\]."]
    pub srssi: crate::Reg<srssi::SRSSI_SPEC>,
    #[doc = "0x28 - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcuart: crate::Reg<rcgcuart::RCGCUART_SPEC>,
    #[doc = "0x2c - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcuart: crate::Reg<scgcuart::SCGCUART_SPEC>,
    #[doc = "0x30 - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcuart: crate::Reg<dcgcuart::DCGCUART_SPEC>,
    #[doc = "0x34 - This register controls the reset for UART\\[1:0\\]."]
    pub sruart: crate::Reg<sruart::SRUART_SPEC>,
    #[doc = "0x38 - This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgci2c: crate::Reg<rcgci2c::RCGCI2C_SPEC>,
    #[doc = "0x3c - This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgci2c: crate::Reg<scgci2c::SCGCI2C_SPEC>,
    #[doc = "0x40 - This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgci2c: crate::Reg<dcgci2c::DCGCI2C_SPEC>,
    #[doc = "0x44 - This register controls the reset for I2C."]
    pub sri2c: crate::Reg<sri2c::SRI2C_SPEC>,
    #[doc = "0x48 - This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcsec: crate::Reg<rcgcsec::RCGCSEC_SPEC>,
    #[doc = "0x4c - This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcsec: crate::Reg<scgcsec::SCGCSEC_SPEC>,
    #[doc = "0x50 - This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcsec: crate::Reg<dcgcsec::DCGCSEC_SPEC>,
    #[doc = "0x54 - This register controls the reset for the security module."]
    pub srsec: crate::Reg<srsec::SRSEC_SPEC>,
    #[doc = "0x58 - This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
    pub pmctl: crate::Reg<pmctl::PMCTL_SPEC>,
    #[doc = "0x5c - This register controls CRC on state retention."]
    pub srcrc: crate::Reg<srcrc::SRCRC_SPEC>,
    _reserved24: [u8; 0x14],
    #[doc = "0x74 - Power debug register"]
    pub pwrdbg: crate::Reg<pwrdbg::PWRDBG_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x80 - This register controls the clock loss detection feature."]
    pub cld: crate::Reg<cld::CLD_SPEC>,
    _reserved26: [u8; 0x10],
    #[doc = "0x94 - This register controls interrupt wake-up."]
    pub iwe: crate::Reg<iwe::IWE_SPEC>,
    #[doc = "0x98 - This register selects which interrupt map to be used."]
    pub i_map: crate::Reg<i_map::I_MAP_SPEC>,
    _reserved28: [u8; 0x0c],
    #[doc = "0xa8 - This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcrfc: crate::Reg<rcgcrfc::RCGCRFC_SPEC>,
    #[doc = "0xac - This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcrfc: crate::Reg<scgcrfc::SCGCRFC_SPEC>,
    #[doc = "0xb0 - This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcrfc: crate::Reg<dcgcrfc::DCGCRFC_SPEC>,
    #[doc = "0xb4 - This register defines the emulator override controls for power mode and peripheral clock gate."]
    pub emuovr: crate::Reg<emuovr::EMUOVR_SPEC>,
}
#[doc = "CLOCK_CTRL register accessor: an alias for `Reg<CLOCK_CTRL_SPEC>`"]
pub type CLOCK_CTRL = crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>;
#[doc = "The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
pub mod clock_ctrl;
#[doc = "CLOCK_STA register accessor: an alias for `Reg<CLOCK_STA_SPEC>`"]
pub type CLOCK_STA = crate::Reg<clock_sta::CLOCK_STA_SPEC>;
#[doc = "Clock status register This register reflects the current chip status."]
pub mod clock_sta;
#[doc = "RCGCGPT register accessor: an alias for `Reg<RCGCGPT_SPEC>`"]
pub type RCGCGPT = crate::Reg<rcgcgpt::RCGCGPT_SPEC>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcgpt;
#[doc = "SCGCGPT register accessor: an alias for `Reg<SCGCGPT_SPEC>`"]
pub type SCGCGPT = crate::Reg<scgcgpt::SCGCGPT_SPEC>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcgpt;
#[doc = "DCGCGPT register accessor: an alias for `Reg<DCGCGPT_SPEC>`"]
pub type DCGCGPT = crate::Reg<dcgcgpt::DCGCGPT_SPEC>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcgpt;
#[doc = "SRGPT register accessor: an alias for `Reg<SRGPT_SPEC>`"]
pub type SRGPT = crate::Reg<srgpt::SRGPT_SPEC>;
#[doc = "This register controls the reset for GPT\\[3:0\\]."]
pub mod srgpt;
#[doc = "RCGCSSI register accessor: an alias for `Reg<RCGCSSI_SPEC>`"]
pub type RCGCSSI = crate::Reg<rcgcssi::RCGCSSI_SPEC>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcssi;
#[doc = "SCGCSSI register accessor: an alias for `Reg<SCGCSSI_SPEC>`"]
pub type SCGCSSI = crate::Reg<scgcssi::SCGCSSI_SPEC>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcssi;
#[doc = "DCGCSSI register accessor: an alias for `Reg<DCGCSSI_SPEC>`"]
pub type DCGCSSI = crate::Reg<dcgcssi::DCGCSSI_SPEC>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcssi;
#[doc = "SRSSI register accessor: an alias for `Reg<SRSSI_SPEC>`"]
pub type SRSSI = crate::Reg<srssi::SRSSI_SPEC>;
#[doc = "This register controls the reset for SSI\\[1:0\\]."]
pub mod srssi;
#[doc = "RCGCUART register accessor: an alias for `Reg<RCGCUART_SPEC>`"]
pub type RCGCUART = crate::Reg<rcgcuart::RCGCUART_SPEC>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcuart;
#[doc = "SCGCUART register accessor: an alias for `Reg<SCGCUART_SPEC>`"]
pub type SCGCUART = crate::Reg<scgcuart::SCGCUART_SPEC>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcuart;
#[doc = "DCGCUART register accessor: an alias for `Reg<DCGCUART_SPEC>`"]
pub type DCGCUART = crate::Reg<dcgcuart::DCGCUART_SPEC>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcuart;
#[doc = "SRUART register accessor: an alias for `Reg<SRUART_SPEC>`"]
pub type SRUART = crate::Reg<sruart::SRUART_SPEC>;
#[doc = "This register controls the reset for UART\\[1:0\\]."]
pub mod sruart;
#[doc = "RCGCI2C register accessor: an alias for `Reg<RCGCI2C_SPEC>`"]
pub type RCGCI2C = crate::Reg<rcgci2c::RCGCI2C_SPEC>;
#[doc = "This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgci2c;
#[doc = "SCGCI2C register accessor: an alias for `Reg<SCGCI2C_SPEC>`"]
pub type SCGCI2C = crate::Reg<scgci2c::SCGCI2C_SPEC>;
#[doc = "This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgci2c;
#[doc = "DCGCI2C register accessor: an alias for `Reg<DCGCI2C_SPEC>`"]
pub type DCGCI2C = crate::Reg<dcgci2c::DCGCI2C_SPEC>;
#[doc = "This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgci2c;
#[doc = "SRI2C register accessor: an alias for `Reg<SRI2C_SPEC>`"]
pub type SRI2C = crate::Reg<sri2c::SRI2C_SPEC>;
#[doc = "This register controls the reset for I2C."]
pub mod sri2c;
#[doc = "RCGCSEC register accessor: an alias for `Reg<RCGCSEC_SPEC>`"]
pub type RCGCSEC = crate::Reg<rcgcsec::RCGCSEC_SPEC>;
#[doc = "This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcsec;
#[doc = "SCGCSEC register accessor: an alias for `Reg<SCGCSEC_SPEC>`"]
pub type SCGCSEC = crate::Reg<scgcsec::SCGCSEC_SPEC>;
#[doc = "This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcsec;
#[doc = "DCGCSEC register accessor: an alias for `Reg<DCGCSEC_SPEC>`"]
pub type DCGCSEC = crate::Reg<dcgcsec::DCGCSEC_SPEC>;
#[doc = "This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcsec;
#[doc = "SRSEC register accessor: an alias for `Reg<SRSEC_SPEC>`"]
pub type SRSEC = crate::Reg<srsec::SRSEC_SPEC>;
#[doc = "This register controls the reset for the security module."]
pub mod srsec;
#[doc = "PMCTL register accessor: an alias for `Reg<PMCTL_SPEC>`"]
pub type PMCTL = crate::Reg<pmctl::PMCTL_SPEC>;
#[doc = "This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
pub mod pmctl;
#[doc = "SRCRC register accessor: an alias for `Reg<SRCRC_SPEC>`"]
pub type SRCRC = crate::Reg<srcrc::SRCRC_SPEC>;
#[doc = "This register controls CRC on state retention."]
pub mod srcrc;
#[doc = "PWRDBG register accessor: an alias for `Reg<PWRDBG_SPEC>`"]
pub type PWRDBG = crate::Reg<pwrdbg::PWRDBG_SPEC>;
#[doc = "Power debug register"]
pub mod pwrdbg;
#[doc = "CLD register accessor: an alias for `Reg<CLD_SPEC>`"]
pub type CLD = crate::Reg<cld::CLD_SPEC>;
#[doc = "This register controls the clock loss detection feature."]
pub mod cld;
#[doc = "IWE register accessor: an alias for `Reg<IWE_SPEC>`"]
pub type IWE = crate::Reg<iwe::IWE_SPEC>;
#[doc = "This register controls interrupt wake-up."]
pub mod iwe;
#[doc = "I_MAP register accessor: an alias for `Reg<I_MAP_SPEC>`"]
pub type I_MAP = crate::Reg<i_map::I_MAP_SPEC>;
#[doc = "This register selects which interrupt map to be used."]
pub mod i_map;
#[doc = "RCGCRFC register accessor: an alias for `Reg<RCGCRFC_SPEC>`"]
pub type RCGCRFC = crate::Reg<rcgcrfc::RCGCRFC_SPEC>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcrfc;
#[doc = "SCGCRFC register accessor: an alias for `Reg<SCGCRFC_SPEC>`"]
pub type SCGCRFC = crate::Reg<scgcrfc::SCGCRFC_SPEC>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcrfc;
#[doc = "DCGCRFC register accessor: an alias for `Reg<DCGCRFC_SPEC>`"]
pub type DCGCRFC = crate::Reg<dcgcrfc::DCGCRFC_SPEC>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcrfc;
#[doc = "EMUOVR register accessor: an alias for `Reg<EMUOVR_SPEC>`"]
pub type EMUOVR = crate::Reg<emuovr::EMUOVR_SPEC>;
#[doc = "This register defines the emulator override controls for power mode and peripheral clock gate."]
pub mod emuovr;

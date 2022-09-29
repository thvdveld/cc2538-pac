#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
    pub clock_ctrl: CLOCK_CTRL,
    #[doc = "0x04 - Clock status register This register reflects the current chip status."]
    pub clock_sta: CLOCK_STA,
    #[doc = "0x08 - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcgpt: RCGCGPT,
    #[doc = "0x0c - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcgpt: SCGCGPT,
    #[doc = "0x10 - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcgpt: DCGCGPT,
    #[doc = "0x14 - This register controls the reset for GPT\\[3:0\\]."]
    pub srgpt: SRGPT,
    #[doc = "0x18 - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcssi: RCGCSSI,
    #[doc = "0x1c - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcssi: SCGCSSI,
    #[doc = "0x20 - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcssi: DCGCSSI,
    #[doc = "0x24 - This register controls the reset for SSI\\[1:0\\]."]
    pub srssi: SRSSI,
    #[doc = "0x28 - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcuart: RCGCUART,
    #[doc = "0x2c - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcuart: SCGCUART,
    #[doc = "0x30 - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcuart: DCGCUART,
    #[doc = "0x34 - This register controls the reset for UART\\[1:0\\]."]
    pub sruart: SRUART,
    #[doc = "0x38 - This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgci2c: RCGCI2C,
    #[doc = "0x3c - This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgci2c: SCGCI2C,
    #[doc = "0x40 - This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgci2c: DCGCI2C,
    #[doc = "0x44 - This register controls the reset for I2C."]
    pub sri2c: SRI2C,
    #[doc = "0x48 - This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcsec: RCGCSEC,
    #[doc = "0x4c - This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcsec: SCGCSEC,
    #[doc = "0x50 - This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcsec: DCGCSEC,
    #[doc = "0x54 - This register controls the reset for the security module."]
    pub srsec: SRSEC,
    #[doc = "0x58 - This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
    pub pmctl: PMCTL,
    #[doc = "0x5c - This register controls CRC on state retention."]
    pub srcrc: SRCRC,
    _reserved24: [u8; 0x14],
    #[doc = "0x74 - Power debug register"]
    pub pwrdbg: PWRDBG,
    _reserved25: [u8; 0x08],
    #[doc = "0x80 - This register controls the clock loss detection feature."]
    pub cld: CLD,
    _reserved26: [u8; 0x10],
    #[doc = "0x94 - This register controls interrupt wake-up."]
    pub iwe: IWE,
    #[doc = "0x98 - This register selects which interrupt map to be used."]
    pub i_map: I_MAP,
    _reserved28: [u8; 0x0c],
    #[doc = "0xa8 - This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcrfc: RCGCRFC,
    #[doc = "0xac - This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcrfc: SCGCRFC,
    #[doc = "0xb0 - This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcrfc: DCGCRFC,
    #[doc = "0xb4 - This register defines the emulator override controls for power mode and peripheral clock gate."]
    pub emuovr: EMUOVR,
}
#[doc = "CLOCK_CTRL (rw) register accessor: an alias for `Reg<CLOCK_CTRL_SPEC>`"]
pub type CLOCK_CTRL = crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>;
#[doc = "The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
pub mod clock_ctrl;
#[doc = "CLOCK_STA (r) register accessor: an alias for `Reg<CLOCK_STA_SPEC>`"]
pub type CLOCK_STA = crate::Reg<clock_sta::CLOCK_STA_SPEC>;
#[doc = "Clock status register This register reflects the current chip status."]
pub mod clock_sta;
#[doc = "RCGCGPT (rw) register accessor: an alias for `Reg<RCGCGPT_SPEC>`"]
pub type RCGCGPT = crate::Reg<rcgcgpt::RCGCGPT_SPEC>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcgpt;
#[doc = "SCGCGPT (rw) register accessor: an alias for `Reg<SCGCGPT_SPEC>`"]
pub type SCGCGPT = crate::Reg<scgcgpt::SCGCGPT_SPEC>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcgpt;
#[doc = "DCGCGPT (rw) register accessor: an alias for `Reg<DCGCGPT_SPEC>`"]
pub type DCGCGPT = crate::Reg<dcgcgpt::DCGCGPT_SPEC>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcgpt;
#[doc = "SRGPT (rw) register accessor: an alias for `Reg<SRGPT_SPEC>`"]
pub type SRGPT = crate::Reg<srgpt::SRGPT_SPEC>;
#[doc = "This register controls the reset for GPT\\[3:0\\]."]
pub mod srgpt;
#[doc = "RCGCSSI (rw) register accessor: an alias for `Reg<RCGCSSI_SPEC>`"]
pub type RCGCSSI = crate::Reg<rcgcssi::RCGCSSI_SPEC>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcssi;
#[doc = "SCGCSSI (rw) register accessor: an alias for `Reg<SCGCSSI_SPEC>`"]
pub type SCGCSSI = crate::Reg<scgcssi::SCGCSSI_SPEC>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcssi;
#[doc = "DCGCSSI (rw) register accessor: an alias for `Reg<DCGCSSI_SPEC>`"]
pub type DCGCSSI = crate::Reg<dcgcssi::DCGCSSI_SPEC>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcssi;
#[doc = "SRSSI (rw) register accessor: an alias for `Reg<SRSSI_SPEC>`"]
pub type SRSSI = crate::Reg<srssi::SRSSI_SPEC>;
#[doc = "This register controls the reset for SSI\\[1:0\\]."]
pub mod srssi;
#[doc = "RCGCUART (rw) register accessor: an alias for `Reg<RCGCUART_SPEC>`"]
pub type RCGCUART = crate::Reg<rcgcuart::RCGCUART_SPEC>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcuart;
#[doc = "SCGCUART (rw) register accessor: an alias for `Reg<SCGCUART_SPEC>`"]
pub type SCGCUART = crate::Reg<scgcuart::SCGCUART_SPEC>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcuart;
#[doc = "DCGCUART (rw) register accessor: an alias for `Reg<DCGCUART_SPEC>`"]
pub type DCGCUART = crate::Reg<dcgcuart::DCGCUART_SPEC>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcuart;
#[doc = "SRUART (rw) register accessor: an alias for `Reg<SRUART_SPEC>`"]
pub type SRUART = crate::Reg<sruart::SRUART_SPEC>;
#[doc = "This register controls the reset for UART\\[1:0\\]."]
pub mod sruart;
#[doc = "RCGCI2C (rw) register accessor: an alias for `Reg<RCGCI2C_SPEC>`"]
pub type RCGCI2C = crate::Reg<rcgci2c::RCGCI2C_SPEC>;
#[doc = "This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgci2c;
#[doc = "SCGCI2C (rw) register accessor: an alias for `Reg<SCGCI2C_SPEC>`"]
pub type SCGCI2C = crate::Reg<scgci2c::SCGCI2C_SPEC>;
#[doc = "This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgci2c;
#[doc = "DCGCI2C (rw) register accessor: an alias for `Reg<DCGCI2C_SPEC>`"]
pub type DCGCI2C = crate::Reg<dcgci2c::DCGCI2C_SPEC>;
#[doc = "This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgci2c;
#[doc = "SRI2C (rw) register accessor: an alias for `Reg<SRI2C_SPEC>`"]
pub type SRI2C = crate::Reg<sri2c::SRI2C_SPEC>;
#[doc = "This register controls the reset for I2C."]
pub mod sri2c;
#[doc = "RCGCSEC (rw) register accessor: an alias for `Reg<RCGCSEC_SPEC>`"]
pub type RCGCSEC = crate::Reg<rcgcsec::RCGCSEC_SPEC>;
#[doc = "This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcsec;
#[doc = "SCGCSEC (rw) register accessor: an alias for `Reg<SCGCSEC_SPEC>`"]
pub type SCGCSEC = crate::Reg<scgcsec::SCGCSEC_SPEC>;
#[doc = "This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcsec;
#[doc = "DCGCSEC (rw) register accessor: an alias for `Reg<DCGCSEC_SPEC>`"]
pub type DCGCSEC = crate::Reg<dcgcsec::DCGCSEC_SPEC>;
#[doc = "This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcsec;
#[doc = "SRSEC (rw) register accessor: an alias for `Reg<SRSEC_SPEC>`"]
pub type SRSEC = crate::Reg<srsec::SRSEC_SPEC>;
#[doc = "This register controls the reset for the security module."]
pub mod srsec;
#[doc = "PMCTL (rw) register accessor: an alias for `Reg<PMCTL_SPEC>`"]
pub type PMCTL = crate::Reg<pmctl::PMCTL_SPEC>;
#[doc = "This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
pub mod pmctl;
#[doc = "SRCRC (rw) register accessor: an alias for `Reg<SRCRC_SPEC>`"]
pub type SRCRC = crate::Reg<srcrc::SRCRC_SPEC>;
#[doc = "This register controls CRC on state retention."]
pub mod srcrc;
#[doc = "PWRDBG (rw) register accessor: an alias for `Reg<PWRDBG_SPEC>`"]
pub type PWRDBG = crate::Reg<pwrdbg::PWRDBG_SPEC>;
#[doc = "Power debug register"]
pub mod pwrdbg;
#[doc = "CLD (rw) register accessor: an alias for `Reg<CLD_SPEC>`"]
pub type CLD = crate::Reg<cld::CLD_SPEC>;
#[doc = "This register controls the clock loss detection feature."]
pub mod cld;
#[doc = "IWE (rw) register accessor: an alias for `Reg<IWE_SPEC>`"]
pub type IWE = crate::Reg<iwe::IWE_SPEC>;
#[doc = "This register controls interrupt wake-up."]
pub mod iwe;
#[doc = "I_MAP (rw) register accessor: an alias for `Reg<I_MAP_SPEC>`"]
pub type I_MAP = crate::Reg<i_map::I_MAP_SPEC>;
#[doc = "This register selects which interrupt map to be used."]
pub mod i_map;
#[doc = "RCGCRFC (rw) register accessor: an alias for `Reg<RCGCRFC_SPEC>`"]
pub type RCGCRFC = crate::Reg<rcgcrfc::RCGCRFC_SPEC>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcrfc;
#[doc = "SCGCRFC (rw) register accessor: an alias for `Reg<SCGCRFC_SPEC>`"]
pub type SCGCRFC = crate::Reg<scgcrfc::SCGCRFC_SPEC>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcrfc;
#[doc = "DCGCRFC (rw) register accessor: an alias for `Reg<DCGCRFC_SPEC>`"]
pub type DCGCRFC = crate::Reg<dcgcrfc::DCGCRFC_SPEC>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcrfc;
#[doc = "EMUOVR (rw) register accessor: an alias for `Reg<EMUOVR_SPEC>`"]
pub type EMUOVR = crate::Reg<emuovr::EMUOVR_SPEC>;
#[doc = "This register defines the emulator override controls for power mode and peripheral clock gate."]
pub mod emuovr;

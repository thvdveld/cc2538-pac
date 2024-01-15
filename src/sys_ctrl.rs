#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    clock_ctrl: CLOCK_CTRL,
    clock_sta: CLOCK_STA,
    rcgcgpt: RCGCGPT,
    scgcgpt: SCGCGPT,
    dcgcgpt: DCGCGPT,
    srgpt: SRGPT,
    rcgcssi: RCGCSSI,
    scgcssi: SCGCSSI,
    dcgcssi: DCGCSSI,
    srssi: SRSSI,
    rcgcuart: RCGCUART,
    scgcuart: SCGCUART,
    dcgcuart: DCGCUART,
    sruart: SRUART,
    rcgci2c: RCGCI2C,
    scgci2c: SCGCI2C,
    dcgci2c: DCGCI2C,
    sri2c: SRI2C,
    rcgcsec: RCGCSEC,
    scgcsec: SCGCSEC,
    dcgcsec: DCGCSEC,
    srsec: SRSEC,
    pmctl: PMCTL,
    srcrc: SRCRC,
    _reserved24: [u8; 0x14],
    pwrdbg: PWRDBG,
    _reserved25: [u8; 0x08],
    cld: CLD,
    _reserved26: [u8; 0x10],
    iwe: IWE,
    i_map: I_MAP,
    _reserved28: [u8; 0x0c],
    rcgcrfc: RCGCRFC,
    scgcrfc: SCGCRFC,
    dcgcrfc: DCGCRFC,
    emuovr: EMUOVR,
}
impl RegisterBlock {
    #[doc = "0x00 - The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
    #[inline(always)]
    pub const fn clock_ctrl(&self) -> &CLOCK_CTRL {
        &self.clock_ctrl
    }
    #[doc = "0x04 - Clock status register This register reflects the current chip status."]
    #[inline(always)]
    pub const fn clock_sta(&self) -> &CLOCK_STA {
        &self.clock_sta
    }
    #[doc = "0x08 - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcgpt(&self) -> &RCGCGPT {
        &self.rcgcgpt
    }
    #[doc = "0x0c - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcgpt(&self) -> &SCGCGPT {
        &self.scgcgpt
    }
    #[doc = "0x10 - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcgpt(&self) -> &DCGCGPT {
        &self.dcgcgpt
    }
    #[doc = "0x14 - This register controls the reset for GPT\\[3:0\\]."]
    #[inline(always)]
    pub const fn srgpt(&self) -> &SRGPT {
        &self.srgpt
    }
    #[doc = "0x18 - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcssi(&self) -> &RCGCSSI {
        &self.rcgcssi
    }
    #[doc = "0x1c - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcssi(&self) -> &SCGCSSI {
        &self.scgcssi
    }
    #[doc = "0x20 - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcssi(&self) -> &DCGCSSI {
        &self.dcgcssi
    }
    #[doc = "0x24 - This register controls the reset for SSI\\[1:0\\]."]
    #[inline(always)]
    pub const fn srssi(&self) -> &SRSSI {
        &self.srssi
    }
    #[doc = "0x28 - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcuart(&self) -> &RCGCUART {
        &self.rcgcuart
    }
    #[doc = "0x2c - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcuart(&self) -> &SCGCUART {
        &self.scgcuart
    }
    #[doc = "0x30 - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcuart(&self) -> &DCGCUART {
        &self.dcgcuart
    }
    #[doc = "0x34 - This register controls the reset for UART\\[1:0\\]."]
    #[inline(always)]
    pub const fn sruart(&self) -> &SRUART {
        &self.sruart
    }
    #[doc = "0x38 - This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgci2c(&self) -> &RCGCI2C {
        &self.rcgci2c
    }
    #[doc = "0x3c - This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgci2c(&self) -> &SCGCI2C {
        &self.scgci2c
    }
    #[doc = "0x40 - This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgci2c(&self) -> &DCGCI2C {
        &self.dcgci2c
    }
    #[doc = "0x44 - This register controls the reset for I2C."]
    #[inline(always)]
    pub const fn sri2c(&self) -> &SRI2C {
        &self.sri2c
    }
    #[doc = "0x48 - This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcsec(&self) -> &RCGCSEC {
        &self.rcgcsec
    }
    #[doc = "0x4c - This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcsec(&self) -> &SCGCSEC {
        &self.scgcsec
    }
    #[doc = "0x50 - This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcsec(&self) -> &DCGCSEC {
        &self.dcgcsec
    }
    #[doc = "0x54 - This register controls the reset for the security module."]
    #[inline(always)]
    pub const fn srsec(&self) -> &SRSEC {
        &self.srsec
    }
    #[doc = "0x58 - This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
    #[inline(always)]
    pub const fn pmctl(&self) -> &PMCTL {
        &self.pmctl
    }
    #[doc = "0x5c - This register controls CRC on state retention."]
    #[inline(always)]
    pub const fn srcrc(&self) -> &SRCRC {
        &self.srcrc
    }
    #[doc = "0x74 - Power debug register"]
    #[inline(always)]
    pub const fn pwrdbg(&self) -> &PWRDBG {
        &self.pwrdbg
    }
    #[doc = "0x80 - This register controls the clock loss detection feature."]
    #[inline(always)]
    pub const fn cld(&self) -> &CLD {
        &self.cld
    }
    #[doc = "0x94 - This register controls interrupt wake-up."]
    #[inline(always)]
    pub const fn iwe(&self) -> &IWE {
        &self.iwe
    }
    #[doc = "0x98 - This register selects which interrupt map to be used."]
    #[inline(always)]
    pub const fn i_map(&self) -> &I_MAP {
        &self.i_map
    }
    #[doc = "0xa8 - This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcrfc(&self) -> &RCGCRFC {
        &self.rcgcrfc
    }
    #[doc = "0xac - This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcrfc(&self) -> &SCGCRFC {
        &self.scgcrfc
    }
    #[doc = "0xb0 - This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcrfc(&self) -> &DCGCRFC {
        &self.dcgcrfc
    }
    #[doc = "0xb4 - This register defines the emulator override controls for power mode and peripheral clock gate."]
    #[inline(always)]
    pub const fn emuovr(&self) -> &EMUOVR {
        &self.emuovr
    }
}
#[doc = "CLOCK_CTRL (rw) register accessor: The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctrl`]
module"]
pub type CLOCK_CTRL = crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>;
#[doc = "The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
pub mod clock_ctrl;
#[doc = "CLOCK_STA (r) register accessor: Clock status register This register reflects the current chip status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_sta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_sta`]
module"]
pub type CLOCK_STA = crate::Reg<clock_sta::CLOCK_STA_SPEC>;
#[doc = "Clock status register This register reflects the current chip status."]
pub mod clock_sta;
#[doc = "RCGCGPT (rw) register accessor: This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcgpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcgpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcgpt`]
module"]
pub type RCGCGPT = crate::Reg<rcgcgpt::RCGCGPT_SPEC>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcgpt;
#[doc = "SCGCGPT (rw) register accessor: This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcgpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcgpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcgpt`]
module"]
pub type SCGCGPT = crate::Reg<scgcgpt::SCGCGPT_SPEC>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcgpt;
#[doc = "DCGCGPT (rw) register accessor: This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcgpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcgpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcgpt`]
module"]
pub type DCGCGPT = crate::Reg<dcgcgpt::DCGCGPT_SPEC>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcgpt;
#[doc = "SRGPT (rw) register accessor: This register controls the reset for GPT\\[3:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srgpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srgpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srgpt`]
module"]
pub type SRGPT = crate::Reg<srgpt::SRGPT_SPEC>;
#[doc = "This register controls the reset for GPT\\[3:0\\]."]
pub mod srgpt;
#[doc = "RCGCSSI (rw) register accessor: This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcssi`]
module"]
pub type RCGCSSI = crate::Reg<rcgcssi::RCGCSSI_SPEC>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcssi;
#[doc = "SCGCSSI (rw) register accessor: This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcssi`]
module"]
pub type SCGCSSI = crate::Reg<scgcssi::SCGCSSI_SPEC>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcssi;
#[doc = "DCGCSSI (rw) register accessor: This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcssi`]
module"]
pub type DCGCSSI = crate::Reg<dcgcssi::DCGCSSI_SPEC>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcssi;
#[doc = "SRSSI (rw) register accessor: This register controls the reset for SSI\\[1:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srssi`]
module"]
pub type SRSSI = crate::Reg<srssi::SRSSI_SPEC>;
#[doc = "This register controls the reset for SSI\\[1:0\\]."]
pub mod srssi;
#[doc = "RCGCUART (rw) register accessor: This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcuart`]
module"]
pub type RCGCUART = crate::Reg<rcgcuart::RCGCUART_SPEC>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcuart;
#[doc = "SCGCUART (rw) register accessor: This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcuart`]
module"]
pub type SCGCUART = crate::Reg<scgcuart::SCGCUART_SPEC>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcuart;
#[doc = "DCGCUART (rw) register accessor: This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcuart`]
module"]
pub type DCGCUART = crate::Reg<dcgcuart::DCGCUART_SPEC>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcuart;
#[doc = "SRUART (rw) register accessor: This register controls the reset for UART\\[1:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sruart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sruart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sruart`]
module"]
pub type SRUART = crate::Reg<sruart::SRUART_SPEC>;
#[doc = "This register controls the reset for UART\\[1:0\\]."]
pub mod sruart;
#[doc = "RCGCI2C (rw) register accessor: This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgci2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgci2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgci2c`]
module"]
pub type RCGCI2C = crate::Reg<rcgci2c::RCGCI2C_SPEC>;
#[doc = "This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgci2c;
#[doc = "SCGCI2C (rw) register accessor: This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgci2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgci2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgci2c`]
module"]
pub type SCGCI2C = crate::Reg<scgci2c::SCGCI2C_SPEC>;
#[doc = "This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgci2c;
#[doc = "DCGCI2C (rw) register accessor: This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgci2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgci2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgci2c`]
module"]
pub type DCGCI2C = crate::Reg<dcgci2c::DCGCI2C_SPEC>;
#[doc = "This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgci2c;
#[doc = "SRI2C (rw) register accessor: This register controls the reset for I2C.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sri2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sri2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sri2c`]
module"]
pub type SRI2C = crate::Reg<sri2c::SRI2C_SPEC>;
#[doc = "This register controls the reset for I2C."]
pub mod sri2c;
#[doc = "RCGCSEC (rw) register accessor: This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcsec`]
module"]
pub type RCGCSEC = crate::Reg<rcgcsec::RCGCSEC_SPEC>;
#[doc = "This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcsec;
#[doc = "SCGCSEC (rw) register accessor: This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcsec`]
module"]
pub type SCGCSEC = crate::Reg<scgcsec::SCGCSEC_SPEC>;
#[doc = "This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcsec;
#[doc = "DCGCSEC (rw) register accessor: This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcsec`]
module"]
pub type DCGCSEC = crate::Reg<dcgcsec::DCGCSEC_SPEC>;
#[doc = "This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcsec;
#[doc = "SRSEC (rw) register accessor: This register controls the reset for the security module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsec`]
module"]
pub type SRSEC = crate::Reg<srsec::SRSEC_SPEC>;
#[doc = "This register controls the reset for the security module."]
pub mod srsec;
#[doc = "PMCTL (rw) register accessor: This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmctl`]
module"]
pub type PMCTL = crate::Reg<pmctl::PMCTL_SPEC>;
#[doc = "This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
pub mod pmctl;
#[doc = "SRCRC (rw) register accessor: This register controls CRC on state retention.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcrc`]
module"]
pub type SRCRC = crate::Reg<srcrc::SRCRC_SPEC>;
#[doc = "This register controls CRC on state retention."]
pub mod srcrc;
#[doc = "PWRDBG (rw) register accessor: Power debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdbg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdbg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrdbg`]
module"]
pub type PWRDBG = crate::Reg<pwrdbg::PWRDBG_SPEC>;
#[doc = "Power debug register"]
pub mod pwrdbg;
#[doc = "CLD (rw) register accessor: This register controls the clock loss detection feature.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cld`]
module"]
pub type CLD = crate::Reg<cld::CLD_SPEC>;
#[doc = "This register controls the clock loss detection feature."]
pub mod cld;
#[doc = "IWE (rw) register accessor: This register controls interrupt wake-up.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwe`]
module"]
pub type IWE = crate::Reg<iwe::IWE_SPEC>;
#[doc = "This register controls interrupt wake-up."]
pub mod iwe;
#[doc = "I_MAP (rw) register accessor: This register selects which interrupt map to be used.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i_map`]
module"]
pub type I_MAP = crate::Reg<i_map::I_MAP_SPEC>;
#[doc = "This register selects which interrupt map to be used."]
pub mod i_map;
#[doc = "RCGCRFC (rw) register accessor: This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgcrfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgcrfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcrfc`]
module"]
pub type RCGCRFC = crate::Reg<rcgcrfc::RCGCRFC_SPEC>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcrfc;
#[doc = "SCGCRFC (rw) register accessor: This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcrfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcrfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcrfc`]
module"]
pub type SCGCRFC = crate::Reg<scgcrfc::SCGCRFC_SPEC>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcrfc;
#[doc = "DCGCRFC (rw) register accessor: This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcrfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcrfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcrfc`]
module"]
pub type DCGCRFC = crate::Reg<dcgcrfc::DCGCRFC_SPEC>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcrfc;
#[doc = "EMUOVR (rw) register accessor: This register defines the emulator override controls for power mode and peripheral clock gate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emuovr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emuovr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emuovr`]
module"]
pub type EMUOVR = crate::Reg<emuovr::EMUOVR_SPEC>;
#[doc = "This register defines the emulator override controls for power mode and peripheral clock gate."]
pub mod emuovr;

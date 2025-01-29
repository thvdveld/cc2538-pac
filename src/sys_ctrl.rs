#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clock_ctrl: ClockCtrl,
    clock_sta: ClockSta,
    rcgcgpt: Rcgcgpt,
    scgcgpt: Scgcgpt,
    dcgcgpt: Dcgcgpt,
    srgpt: Srgpt,
    rcgcssi: Rcgcssi,
    scgcssi: Scgcssi,
    dcgcssi: Dcgcssi,
    srssi: Srssi,
    rcgcuart: Rcgcuart,
    scgcuart: Scgcuart,
    dcgcuart: Dcgcuart,
    sruart: Sruart,
    rcgci2c: Rcgci2c,
    scgci2c: Scgci2c,
    dcgci2c: Dcgci2c,
    sri2c: Sri2c,
    rcgcsec: Rcgcsec,
    scgcsec: Scgcsec,
    dcgcsec: Dcgcsec,
    srsec: Srsec,
    pmctl: Pmctl,
    srcrc: Srcrc,
    _reserved24: [u8; 0x14],
    pwrdbg: Pwrdbg,
    _reserved25: [u8; 0x08],
    cld: Cld,
    _reserved26: [u8; 0x10],
    iwe: Iwe,
    i_map: IMap,
    _reserved28: [u8; 0x0c],
    rcgcrfc: Rcgcrfc,
    scgcrfc: Scgcrfc,
    dcgcrfc: Dcgcrfc,
    emuovr: Emuovr,
}
impl RegisterBlock {
    #[doc = "0x00 - The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
    #[inline(always)]
    pub const fn clock_ctrl(&self) -> &ClockCtrl {
        &self.clock_ctrl
    }
    #[doc = "0x04 - Clock status register This register reflects the current chip status."]
    #[inline(always)]
    pub const fn clock_sta(&self) -> &ClockSta {
        &self.clock_sta
    }
    #[doc = "0x08 - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcgpt(&self) -> &Rcgcgpt {
        &self.rcgcgpt
    }
    #[doc = "0x0c - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcgpt(&self) -> &Scgcgpt {
        &self.scgcgpt
    }
    #[doc = "0x10 - This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcgpt(&self) -> &Dcgcgpt {
        &self.dcgcgpt
    }
    #[doc = "0x14 - This register controls the reset for GPT\\[3:0\\]."]
    #[inline(always)]
    pub const fn srgpt(&self) -> &Srgpt {
        &self.srgpt
    }
    #[doc = "0x18 - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcssi(&self) -> &Rcgcssi {
        &self.rcgcssi
    }
    #[doc = "0x1c - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcssi(&self) -> &Scgcssi {
        &self.scgcssi
    }
    #[doc = "0x20 - This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcssi(&self) -> &Dcgcssi {
        &self.dcgcssi
    }
    #[doc = "0x24 - This register controls the reset for SSI\\[1:0\\]."]
    #[inline(always)]
    pub const fn srssi(&self) -> &Srssi {
        &self.srssi
    }
    #[doc = "0x28 - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcuart(&self) -> &Rcgcuart {
        &self.rcgcuart
    }
    #[doc = "0x2c - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcuart(&self) -> &Scgcuart {
        &self.scgcuart
    }
    #[doc = "0x30 - This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcuart(&self) -> &Dcgcuart {
        &self.dcgcuart
    }
    #[doc = "0x34 - This register controls the reset for UART\\[1:0\\]."]
    #[inline(always)]
    pub const fn sruart(&self) -> &Sruart {
        &self.sruart
    }
    #[doc = "0x38 - This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgci2c(&self) -> &Rcgci2c {
        &self.rcgci2c
    }
    #[doc = "0x3c - This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgci2c(&self) -> &Scgci2c {
        &self.scgci2c
    }
    #[doc = "0x40 - This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgci2c(&self) -> &Dcgci2c {
        &self.dcgci2c
    }
    #[doc = "0x44 - This register controls the reset for I2C."]
    #[inline(always)]
    pub const fn sri2c(&self) -> &Sri2c {
        &self.sri2c
    }
    #[doc = "0x48 - This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcsec(&self) -> &Rcgcsec {
        &self.rcgcsec
    }
    #[doc = "0x4c - This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcsec(&self) -> &Scgcsec {
        &self.scgcsec
    }
    #[doc = "0x50 - This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcsec(&self) -> &Dcgcsec {
        &self.dcgcsec
    }
    #[doc = "0x54 - This register controls the reset for the security module."]
    #[inline(always)]
    pub const fn srsec(&self) -> &Srsec {
        &self.srsec
    }
    #[doc = "0x58 - This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
    #[inline(always)]
    pub const fn pmctl(&self) -> &Pmctl {
        &self.pmctl
    }
    #[doc = "0x5c - This register controls CRC on state retention."]
    #[inline(always)]
    pub const fn srcrc(&self) -> &Srcrc {
        &self.srcrc
    }
    #[doc = "0x74 - Power debug register"]
    #[inline(always)]
    pub const fn pwrdbg(&self) -> &Pwrdbg {
        &self.pwrdbg
    }
    #[doc = "0x80 - This register controls the clock loss detection feature."]
    #[inline(always)]
    pub const fn cld(&self) -> &Cld {
        &self.cld
    }
    #[doc = "0x94 - This register controls interrupt wake-up."]
    #[inline(always)]
    pub const fn iwe(&self) -> &Iwe {
        &self.iwe
    }
    #[doc = "0x98 - This register selects which interrupt map to be used."]
    #[inline(always)]
    pub const fn i_map(&self) -> &IMap {
        &self.i_map
    }
    #[doc = "0xa8 - This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn rcgcrfc(&self) -> &Rcgcrfc {
        &self.rcgcrfc
    }
    #[doc = "0xac - This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn scgcrfc(&self) -> &Scgcrfc {
        &self.scgcrfc
    }
    #[doc = "0xb0 - This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    #[inline(always)]
    pub const fn dcgcrfc(&self) -> &Dcgcrfc {
        &self.dcgcrfc
    }
    #[doc = "0xb4 - This register defines the emulator override controls for power mode and peripheral clock gate."]
    #[inline(always)]
    pub const fn emuovr(&self) -> &Emuovr {
        &self.emuovr
    }
}
#[doc = "CLOCK_CTRL (rw) register accessor: The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctrl`]
module"]
#[doc(alias = "CLOCK_CTRL")]
pub type ClockCtrl = crate::Reg<clock_ctrl::ClockCtrlSpec>;
#[doc = "The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
pub mod clock_ctrl;
#[doc = "CLOCK_STA (r) register accessor: Clock status register This register reflects the current chip status.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_sta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_sta`]
module"]
#[doc(alias = "CLOCK_STA")]
pub type ClockSta = crate::Reg<clock_sta::ClockStaSpec>;
#[doc = "Clock status register This register reflects the current chip status."]
pub mod clock_sta;
#[doc = "RCGCGPT (rw) register accessor: This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`rcgcgpt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcgcgpt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcgpt`]
module"]
#[doc(alias = "RCGCGPT")]
pub type Rcgcgpt = crate::Reg<rcgcgpt::RcgcgptSpec>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcgpt;
#[doc = "SCGCGPT (rw) register accessor: This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`scgcgpt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgcgpt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcgpt`]
module"]
#[doc(alias = "SCGCGPT")]
pub type Scgcgpt = crate::Reg<scgcgpt::ScgcgptSpec>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcgpt;
#[doc = "DCGCGPT (rw) register accessor: This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcgcgpt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcgcgpt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcgpt`]
module"]
#[doc(alias = "DCGCGPT")]
pub type Dcgcgpt = crate::Reg<dcgcgpt::DcgcgptSpec>;
#[doc = "This register defines the module clocks for GPT\\[3:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcgpt;
#[doc = "SRGPT (rw) register accessor: This register controls the reset for GPT\\[3:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`srgpt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srgpt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srgpt`]
module"]
#[doc(alias = "SRGPT")]
pub type Srgpt = crate::Reg<srgpt::SrgptSpec>;
#[doc = "This register controls the reset for GPT\\[3:0\\]."]
pub mod srgpt;
#[doc = "RCGCSSI (rw) register accessor: This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`rcgcssi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcgcssi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcssi`]
module"]
#[doc(alias = "RCGCSSI")]
pub type Rcgcssi = crate::Reg<rcgcssi::RcgcssiSpec>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcssi;
#[doc = "SCGCSSI (rw) register accessor: This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`scgcssi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgcssi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcssi`]
module"]
#[doc(alias = "SCGCSSI")]
pub type Scgcssi = crate::Reg<scgcssi::ScgcssiSpec>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcssi;
#[doc = "DCGCSSI (rw) register accessor: This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcgcssi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcgcssi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcssi`]
module"]
#[doc(alias = "DCGCSSI")]
pub type Dcgcssi = crate::Reg<dcgcssi::DcgcssiSpec>;
#[doc = "This register defines the module clocks for SSI\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcssi;
#[doc = "SRSSI (rw) register accessor: This register controls the reset for SSI\\[1:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`srssi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srssi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srssi`]
module"]
#[doc(alias = "SRSSI")]
pub type Srssi = crate::Reg<srssi::SrssiSpec>;
#[doc = "This register controls the reset for SSI\\[1:0\\]."]
pub mod srssi;
#[doc = "RCGCUART (rw) register accessor: This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`rcgcuart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcgcuart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcuart`]
module"]
#[doc(alias = "RCGCUART")]
pub type Rcgcuart = crate::Reg<rcgcuart::RcgcuartSpec>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcuart;
#[doc = "SCGCUART (rw) register accessor: This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`scgcuart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgcuart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcuart`]
module"]
#[doc(alias = "SCGCUART")]
pub type Scgcuart = crate::Reg<scgcuart::ScgcuartSpec>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcuart;
#[doc = "DCGCUART (rw) register accessor: This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcgcuart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcgcuart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcuart`]
module"]
#[doc(alias = "DCGCUART")]
pub type Dcgcuart = crate::Reg<dcgcuart::DcgcuartSpec>;
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcuart;
#[doc = "SRUART (rw) register accessor: This register controls the reset for UART\\[1:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`sruart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sruart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sruart`]
module"]
#[doc(alias = "SRUART")]
pub type Sruart = crate::Reg<sruart::SruartSpec>;
#[doc = "This register controls the reset for UART\\[1:0\\]."]
pub mod sruart;
#[doc = "RCGCI2C (rw) register accessor: This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`rcgci2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcgci2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgci2c`]
module"]
#[doc(alias = "RCGCI2C")]
pub type Rcgci2c = crate::Reg<rcgci2c::Rcgci2cSpec>;
#[doc = "This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgci2c;
#[doc = "SCGCI2C (rw) register accessor: This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`scgci2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgci2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgci2c`]
module"]
#[doc(alias = "SCGCI2C")]
pub type Scgci2c = crate::Reg<scgci2c::Scgci2cSpec>;
#[doc = "This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgci2c;
#[doc = "DCGCI2C (rw) register accessor: This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcgci2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcgci2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgci2c`]
module"]
#[doc(alias = "DCGCI2C")]
pub type Dcgci2c = crate::Reg<dcgci2c::Dcgci2cSpec>;
#[doc = "This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgci2c;
#[doc = "SRI2C (rw) register accessor: This register controls the reset for I2C.\n\nYou can [`read`](crate::Reg::read) this register and get [`sri2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sri2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sri2c`]
module"]
#[doc(alias = "SRI2C")]
pub type Sri2c = crate::Reg<sri2c::Sri2cSpec>;
#[doc = "This register controls the reset for I2C."]
pub mod sri2c;
#[doc = "RCGCSEC (rw) register accessor: This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`rcgcsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcgcsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcsec`]
module"]
#[doc(alias = "RCGCSEC")]
pub type Rcgcsec = crate::Reg<rcgcsec::RcgcsecSpec>;
#[doc = "This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcsec;
#[doc = "SCGCSEC (rw) register accessor: This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`scgcsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgcsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcsec`]
module"]
#[doc(alias = "SCGCSEC")]
pub type Scgcsec = crate::Reg<scgcsec::ScgcsecSpec>;
#[doc = "This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcsec;
#[doc = "DCGCSEC (rw) register accessor: This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcgcsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcgcsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcsec`]
module"]
#[doc(alias = "DCGCSEC")]
pub type Dcgcsec = crate::Reg<dcgcsec::DcgcsecSpec>;
#[doc = "This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcsec;
#[doc = "SRSEC (rw) register accessor: This register controls the reset for the security module.\n\nYou can [`read`](crate::Reg::read) this register and get [`srsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsec`]
module"]
#[doc(alias = "SRSEC")]
pub type Srsec = crate::Reg<srsec::SrsecSpec>;
#[doc = "This register controls the reset for the security module."]
pub mod srsec;
#[doc = "PMCTL (rw) register accessor: This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pmctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmctl`]
module"]
#[doc(alias = "PMCTL")]
pub type Pmctl = crate::Reg<pmctl::PmctlSpec>;
#[doc = "This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
pub mod pmctl;
#[doc = "SRCRC (rw) register accessor: This register controls CRC on state retention.\n\nYou can [`read`](crate::Reg::read) this register and get [`srcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcrc`]
module"]
#[doc(alias = "SRCRC")]
pub type Srcrc = crate::Reg<srcrc::SrcrcSpec>;
#[doc = "This register controls CRC on state retention."]
pub mod srcrc;
#[doc = "PWRDBG (rw) register accessor: Power debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrdbg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrdbg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrdbg`]
module"]
#[doc(alias = "PWRDBG")]
pub type Pwrdbg = crate::Reg<pwrdbg::PwrdbgSpec>;
#[doc = "Power debug register"]
pub mod pwrdbg;
#[doc = "CLD (rw) register accessor: This register controls the clock loss detection feature.\n\nYou can [`read`](crate::Reg::read) this register and get [`cld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cld`]
module"]
#[doc(alias = "CLD")]
pub type Cld = crate::Reg<cld::CldSpec>;
#[doc = "This register controls the clock loss detection feature."]
pub mod cld;
#[doc = "IWE (rw) register accessor: This register controls interrupt wake-up.\n\nYou can [`read`](crate::Reg::read) this register and get [`iwe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwe`]
module"]
#[doc(alias = "IWE")]
pub type Iwe = crate::Reg<iwe::IweSpec>;
#[doc = "This register controls interrupt wake-up."]
pub mod iwe;
#[doc = "I_MAP (rw) register accessor: This register selects which interrupt map to be used.\n\nYou can [`read`](crate::Reg::read) this register and get [`i_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i_map`]
module"]
#[doc(alias = "I_MAP")]
pub type IMap = crate::Reg<i_map::IMapSpec>;
#[doc = "This register selects which interrupt map to be used."]
pub mod i_map;
#[doc = "RCGCRFC (rw) register accessor: This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`rcgcrfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcgcrfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcgcrfc`]
module"]
#[doc(alias = "RCGCRFC")]
pub type Rcgcrfc = crate::Reg<rcgcrfc::RcgcrfcSpec>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcrfc;
#[doc = "SCGCRFC (rw) register accessor: This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`scgcrfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgcrfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scgcrfc`]
module"]
#[doc(alias = "SCGCRFC")]
pub type Scgcrfc = crate::Reg<scgcrfc::ScgcrfcSpec>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcrfc;
#[doc = "DCGCRFC (rw) register accessor: This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcgcrfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcgcrfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgcrfc`]
module"]
#[doc(alias = "DCGCRFC")]
pub type Dcgcrfc = crate::Reg<dcgcrfc::DcgcrfcSpec>;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcrfc;
#[doc = "EMUOVR (rw) register accessor: This register defines the emulator override controls for power mode and peripheral clock gate.\n\nYou can [`read`](crate::Reg::read) this register and get [`emuovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emuovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emuovr`]
module"]
#[doc(alias = "EMUOVR")]
pub type Emuovr = crate::Reg<emuovr::EmuovrSpec>;
#[doc = "This register defines the emulator override controls for power mode and peripheral clock gate."]
pub mod emuovr;

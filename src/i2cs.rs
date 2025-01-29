#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    oar: Oar,
    _reserved_1_ctrl: [u8; 0x04],
    dr: Dr,
    imr: Imr,
    ris: Ris,
    mis: Mis,
    icr: Icr,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus."]
    #[inline(always)]
    pub const fn oar(&self) -> &Oar {
        &self.oar
    }
    #[doc = "0x04 - I2C slave control and status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - I2C slave control and status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - I2C slave data This register contains the data to be transmitted when in the slave transmit state, and the data received when in the slave receive state."]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x0c - I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x10 - I2C slave raw interrupt status This register specifies whether an interrupt is pending."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x14 - I2C slave masked interrupt status This register specifies whether an interrupt was signaled."]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x18 - I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data."]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
}
#[doc = "OAR (rw) register accessor: I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus.\n\nYou can [`read`](crate::Reg::read) this register and get [`oar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar`]
module"]
#[doc(alias = "OAR")]
pub type Oar = crate::Reg<oar::OarSpec>;
#[doc = "I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus."]
pub mod oar;
#[doc = "STAT (r) register accessor: I2C slave control and status This register functions as a control register when written, and a status register when read.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "I2C slave control and status This register functions as a control register when written, and a status register when read."]
pub mod stat;
#[doc = "CTRL (w) register accessor: I2C slave control and status This register functions as a control register when written, and a status register when read.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "I2C slave control and status This register functions as a control register when written, and a status register when read."]
pub mod ctrl;
#[doc = "DR (rw) register accessor: I2C slave data This register contains the data to be transmitted when in the slave transmit state, and the data received when in the slave receive state.\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "I2C slave data This register contains the data to be transmitted when in the slave transmit state, and the data received when in the slave receive state."]
pub mod dr;
#[doc = "IMR (rw) register accessor: I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod imr;
#[doc = "RIS (r) register accessor: I2C slave raw interrupt status This register specifies whether an interrupt is pending.\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "I2C slave raw interrupt status This register specifies whether an interrupt is pending."]
pub mod ris;
#[doc = "MIS (r) register accessor: I2C slave masked interrupt status This register specifies whether an interrupt was signaled.\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "I2C slave masked interrupt status This register specifies whether an interrupt was signaled."]
pub mod mis;
#[doc = "ICR (w) register accessor: I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data."]
pub mod icr;

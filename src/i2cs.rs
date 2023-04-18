#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus."]
    pub oar: OAR,
    _reserved_1_ctrl: [u8; 0x04],
    #[doc = "0x08 - I2C slave data This register contains the data to be transmitted when in the slave transmit state, and the data received when in the slave receive state."]
    pub dr: DR,
    #[doc = "0x0c - I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    pub imr: IMR,
    #[doc = "0x10 - I2C slave raw interrupt status This register specifies whether an interrupt is pending."]
    pub ris: RIS,
    #[doc = "0x14 - I2C slave masked interrupt status This register specifies whether an interrupt was signaled."]
    pub mis: MIS,
    #[doc = "0x18 - I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data."]
    pub icr: ICR,
}
impl RegisterBlock {
    #[doc = "0x04 - I2C slave control and status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - I2C slave control and status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "OAR (rw) register accessor: an alias for `Reg<OAR_SPEC>`"]
pub type OAR = crate::Reg<oar::OAR_SPEC>;
#[doc = "I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus."]
pub mod oar;
#[doc = "STAT (r) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "I2C slave control and status This register functions as a control register when written, and a status register when read."]
pub mod stat;
#[doc = "CTRL (w) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "I2C slave control and status This register functions as a control register when written, and a status register when read."]
pub mod ctrl;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "I2C slave data This register contains the data to be transmitted when in the slave transmit state, and the data received when in the slave receive state."]
pub mod dr;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod imr;
#[doc = "RIS (r) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "I2C slave raw interrupt status This register specifies whether an interrupt is pending."]
pub mod ris;
#[doc = "MIS (r) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "I2C slave masked interrupt status This register specifies whether an interrupt was signaled."]
pub mod mis;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data."]
pub mod icr;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C master slave address This register consists of eight bits, seven address bits (A6-A0), and a receive and send bit, which determines if the next operation is a receive (high) or transmit (low)."]
    pub sa: SA,
    _reserved_1_ctrl: [u8; 0x04],
    #[doc = "0x08 - I2C master data This register contains the data to be transmitted when in the master transmit state and the data received when in the master receive state."]
    pub dr: DR,
    #[doc = "0x0c - I2C master timer period This register specifies the period of the SCL clock."]
    pub tpr: TPR,
    #[doc = "0x10 - I2C master interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    pub imr: IMR,
    #[doc = "0x14 - I2C master raw interrupt status This register specifies whether an interrupt is pending."]
    pub ris: RIS,
    #[doc = "0x18 - I2C master masked interrupt status This register specifies whether an interrupt was signaled."]
    pub mis: MIS,
    #[doc = "0x1c - I2C master interrupt clear This register clears the raw and masked interrupts."]
    pub icr: ICR,
    #[doc = "0x20 - I2C master configuration This register configures the mode (master or slave) and sets the interface for test mode loopback."]
    pub cr: CR,
}
impl RegisterBlock {
    #[doc = "0x04 - I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "SA (rw) register accessor: I2C master slave address This register consists of eight bits, seven address bits (A6-A0), and a receive and send bit, which determines if the next operation is a receive (high) or transmit (low).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sa`]
module"]
pub type SA = crate::Reg<sa::SA_SPEC>;
#[doc = "I2C master slave address This register consists of eight bits, seven address bits (A6-A0), and a receive and send bit, which determines if the next operation is a receive (high) or transmit (low)."]
pub mod sa;
#[doc = "CTRL (w) register accessor: I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
pub mod ctrl;
#[doc = "STAT (r) register accessor: I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
pub mod stat;
#[doc = "DR (rw) register accessor: I2C master data This register contains the data to be transmitted when in the master transmit state and the data received when in the master receive state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "I2C master data This register contains the data to be transmitted when in the master transmit state and the data received when in the master receive state."]
pub mod dr;
#[doc = "TPR (rw) register accessor: I2C master timer period This register specifies the period of the SCL clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tpr`]
module"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "I2C master timer period This register specifies the period of the SCL clock."]
pub mod tpr;
#[doc = "IMR (rw) register accessor: I2C master interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`imr`]
module"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "I2C master interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod imr;
#[doc = "RIS (r) register accessor: I2C master raw interrupt status This register specifies whether an interrupt is pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "I2C master raw interrupt status This register specifies whether an interrupt is pending."]
pub mod ris;
#[doc = "MIS (r) register accessor: I2C master masked interrupt status This register specifies whether an interrupt was signaled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "I2C master masked interrupt status This register specifies whether an interrupt was signaled."]
pub mod mis;
#[doc = "ICR (w) register accessor: I2C master interrupt clear This register clears the raw and masked interrupts.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "I2C master interrupt clear This register clears the raw and masked interrupts."]
pub mod icr;
#[doc = "CR (rw) register accessor: I2C master configuration This register configures the mode (master or slave) and sets the interface for test mode loopback.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "I2C master configuration This register configures the mode (master or slave) and sets the interface for test mode loopback."]
pub mod cr;

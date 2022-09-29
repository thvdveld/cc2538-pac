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
    pub fn stat(&self) -> &STAT {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const STAT) }
    }
    #[doc = "0x04 - I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    pub fn ctrl(&self) -> &CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const CTRL) }
    }
}
#[doc = "SA (rw) register accessor: an alias for `Reg<SA_SPEC>`"]
pub type SA = crate::Reg<sa::SA_SPEC>;
#[doc = "I2C master slave address This register consists of eight bits, seven address bits (A6-A0), and a receive and send bit, which determines if the next operation is a receive (high) or transmit (low)."]
pub mod sa;
#[doc = "CTRL (w) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
pub mod ctrl;
#[doc = "STAT (r) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "I2C master control and status This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller. When written, the control register configures the I2C controller operation. The START bit generates the START or REPEATED START condition. The STOP bit determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. To generate a single transmit cycle, the I2C master slave address (I2CMSA) register is written with the desired address, the R/S bit is cleared, and this register is written with ACK = X (0 or 1), STOP = 1, START = 1, and RUN = 1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the I2CMDR register. When the I2C module operates in master receiver mode, the ACK bit is normally set, causing the I2C bus controller to automatically transmit an acknowledge after each byte. This bit must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
pub mod stat;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "I2C master data This register contains the data to be transmitted when in the master transmit state and the data received when in the master receive state."]
pub mod dr;
#[doc = "TPR (rw) register accessor: an alias for `Reg<TPR_SPEC>`"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "I2C master timer period This register specifies the period of the SCL clock."]
pub mod tpr;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "I2C master interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod imr;
#[doc = "RIS (r) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "I2C master raw interrupt status This register specifies whether an interrupt is pending."]
pub mod ris;
#[doc = "MIS (r) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "I2C master masked interrupt status This register specifies whether an interrupt was signaled."]
pub mod mis;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "I2C master interrupt clear This register clears the raw and masked interrupts."]
pub mod icr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "I2C master configuration This register configures the mode (master or slave) and sets the interface for test mode loopback."]
pub mod cr;

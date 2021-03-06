#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The CR0 register contains bit fields that control various functions within the SSI module. Functionality such as protocol mode, clock rate, and data size are configured in this register."]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x04 - The CR1 register contains bit fields that control various functions within the SSI module. Master and slave mode functionality is controlled by this register."]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x08 - The DR register is 16 bits wide. When the SSIDR register is read, the entry in the receive FIFO that is pointed to by the current FIFO read pointer is accessed. When a data value is removed by the SSI receive logic from the incoming data frame, it is placed into the entry in the receive FIFO pointed to by the current FIFO write pointer. When the DR register is written to, the entry in the transmit FIFO that is pointed to by the write pointer is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. Each data value is loaded into the transmit serial shifter, then serially shifted out onto the SSITx pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer. When the SSI is programmed for MICROWIRE frame format, the default size for transmit data is eight bits (the most significant byte is ignored). The receive data size is controlled by the programmer. The transmit FIFO and the receive FIFO are not cleared even when the SSE bit in the SSICR1 register is cleared, allowing the software to fill the transmit FIFO before enabling the SSI."]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x0c - The SR register contains bits that indicate the FIFO fill status and the SSI busy status."]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - The CPSR register specifies the division factor which is used to derive the SSIClk from the system clock. The clock is further divided by a value from 1 to 256, which is 1 + SCR. SCR is programmed in the SSICR0 register. The frequency of the SSIClk is defined by: SSIClk = SysClk / (CPSDVSR x (1 + SCR)) The value programmed into this register must be an even number between 2 and 254. The least-significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least-significant bit as zero."]
    pub cpsr: crate::Reg<cpsr::CPSR_SPEC>,
    #[doc = "0x14 - The IM register is the interrupt mask set or clear register. It is a read/write register and all bits are cleared on reset. On a read, this register gives the current value of the mask on the corresponding interrupt. Setting a bit sets the mask, preventing the interrupt from being signaled to the interrupt controller. Clearing a bit clears the corresponding mask, enabling the interrupt to be sent to the interrupt controller."]
    pub im: crate::Reg<im::IM_SPEC>,
    #[doc = "0x18 - The RIS register is the raw interrupt status register. On a read, this register gives the current raw status value of the corresponding interrupt before masking. A write has no effect."]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x1c - The MIS register is the masked interrupt status register. On a read, this register gives the current masked status value of the corresponding interrupt. A write has no effect."]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x20 - The ICR register is the interrupt clear register. On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x24 - The DMACTL register is the uDMA control register."]
    pub dmactl: crate::Reg<dmactl::DMACTL_SPEC>,
    _reserved10: [u8; 0x0fa0],
    #[doc = "0xfc8 - SSI clock configuration The CC register controls the baud clock and system clocks sources for the SSI module. Note: If the PIOSC is used for the SSI baud clock, the system clock frequency must be at least 16 MHz in run mode."]
    pub cc: crate::Reg<cc::CC_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "The CR0 register contains bit fields that control various functions within the SSI module. Functionality such as protocol mode, clock rate, and data size are configured in this register."]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "The CR1 register contains bit fields that control various functions within the SSI module. Master and slave mode functionality is controlled by this register."]
pub mod cr1;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "The DR register is 16 bits wide. When the SSIDR register is read, the entry in the receive FIFO that is pointed to by the current FIFO read pointer is accessed. When a data value is removed by the SSI receive logic from the incoming data frame, it is placed into the entry in the receive FIFO pointed to by the current FIFO write pointer. When the DR register is written to, the entry in the transmit FIFO that is pointed to by the write pointer is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. Each data value is loaded into the transmit serial shifter, then serially shifted out onto the SSITx pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer. When the SSI is programmed for MICROWIRE frame format, the default size for transmit data is eight bits (the most significant byte is ignored). The receive data size is controlled by the programmer. The transmit FIFO and the receive FIFO are not cleared even when the SSE bit in the SSICR1 register is cleared, allowing the software to fill the transmit FIFO before enabling the SSI."]
pub mod dr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "The SR register contains bits that indicate the FIFO fill status and the SSI busy status."]
pub mod sr;
#[doc = "CPSR register accessor: an alias for `Reg<CPSR_SPEC>`"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "The CPSR register specifies the division factor which is used to derive the SSIClk from the system clock. The clock is further divided by a value from 1 to 256, which is 1 + SCR. SCR is programmed in the SSICR0 register. The frequency of the SSIClk is defined by: SSIClk = SysClk / (CPSDVSR x (1 + SCR)) The value programmed into this register must be an even number between 2 and 254. The least-significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least-significant bit as zero."]
pub mod cpsr;
#[doc = "IM register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "The IM register is the interrupt mask set or clear register. It is a read/write register and all bits are cleared on reset. On a read, this register gives the current value of the mask on the corresponding interrupt. Setting a bit sets the mask, preventing the interrupt from being signaled to the interrupt controller. Clearing a bit clears the corresponding mask, enabling the interrupt to be sent to the interrupt controller."]
pub mod im;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "The RIS register is the raw interrupt status register. On a read, this register gives the current raw status value of the corresponding interrupt before masking. A write has no effect."]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "The MIS register is the masked interrupt status register. On a read, this register gives the current masked status value of the corresponding interrupt. A write has no effect."]
pub mod mis;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "The ICR register is the interrupt clear register. On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
pub mod icr;
#[doc = "DMACTL register accessor: an alias for `Reg<DMACTL_SPEC>`"]
pub type DMACTL = crate::Reg<dmactl::DMACTL_SPEC>;
#[doc = "The DMACTL register is the uDMA control register."]
pub mod dmactl;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "SSI clock configuration The CC register controls the baud clock and system clocks sources for the SSI module. Note: If the PIOSC is used for the SSI baud clock, the system clock frequency must be at least 16 MHz in run mode."]
pub mod cc;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This is the data register. In software control mode, values written in the GPIODATA register are transferred onto the GPOUT pins if the respective pins have been configured as outputs through the GPIODIR register. A read from GPIODATA returns the last bit value written if the respective pins are configured as output, or it returns the value on the corresponding input GPIN bit when these are configured as inputs."]
    pub data: crate::Reg<data::DATA_SPEC>,
    _reserved1: [u8; 0x03fc],
    #[doc = "0x400 - The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default."]
    pub dir: crate::Reg<dir::DIR_SPEC>,
    #[doc = "0x404 - The IS register is the interrupt sense register."]
    pub is: crate::Reg<is::IS_SPEC>,
    #[doc = "0x408 - The IBE register is the interrupt both-edges register. When the corresponding bit in IS is set to detect edges, bits set to high in IBE configure the corresponding pin to detect both rising and falling edges, regardless of the corresponding bit in the IEV (interrupt event register). Clearing a bit configures the pin to be controlled by IEV."]
    pub ibe: crate::Reg<ibe::IBE_SPEC>,
    #[doc = "0x40c - The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS."]
    pub iev: crate::Reg<iev::IEV_SPEC>,
    #[doc = "0x410 - The IE register is the interrupt mask register. Bits set to high in IE allow the corresponding pins to trigger their individual interrupts and the combined GPIOINTR line. Clearing a bit disables interrupt triggering on that pin."]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x414 - The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt."]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x418 - The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking."]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x41c - The IC register is the interrupt clear register. Writing 1 to a bit in this register clears the corresponding interrupt edge detection logic register. Writing 0 has no effect."]
    pub ic: crate::Reg<ic::IC_SPEC>,
    #[doc = "0x420 - The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default."]
    pub afsel: crate::Reg<afsel::AFSEL_SPEC>,
    _reserved10: [u8; 0xfc],
    #[doc = "0x520 - A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked."]
    pub gpiolock: crate::Reg<gpiolock::GPIOLOCK_SPEC>,
    #[doc = "0x524 - The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked."]
    pub gpiocr: crate::Reg<gpiocr::GPIOCR_SPEC>,
    _reserved12: [u8; 0x01d8],
    #[doc = "0x700 - The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled."]
    pub pmux: crate::Reg<pmux::PMUX_SPEC>,
    #[doc = "0x704 - The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system."]
    pub p_edge_ctrl: crate::Reg<p_edge_ctrl::P_EDGE_CTRL_SPEC>,
    #[doc = "0x708 - This register is used to control which edge of the USB controller input generates a power-up interrupt to the system."]
    pub usb_ctrl: crate::Reg<usb_ctrl::USB_CTRL_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x710 - The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled."]
    pub pi_ien: crate::Reg<pi_ien::PI_IEN_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x718 - If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1."]
    pub irq_detect_ack: crate::Reg<irq_detect_ack::IRQ_DETECT_ACK_SPEC>,
    #[doc = "0x71c - Same functionality as IRQ_DETECT_ACK, but for USB"]
    pub usb_irq_ack: crate::Reg<usb_irq_ack::USB_IRQ_ACK_SPEC>,
    #[doc = "0x720 - Same functionality as IRQ_DETECT_ACK, but this register handles masked interrupts"]
    pub irq_detect_unmask: crate::Reg<irq_detect_unmask::IRQ_DETECT_UNMASK_SPEC>,
}
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "This is the data register. In software control mode, values written in the GPIODATA register are transferred onto the GPOUT pins if the respective pins have been configured as outputs through the GPIODIR register. A read from GPIODATA returns the last bit value written if the respective pins are configured as output, or it returns the value on the corresponding input GPIN bit when these are configured as inputs."]
pub mod data;
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default."]
pub mod dir;
#[doc = "IS register accessor: an alias for `Reg<IS_SPEC>`"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "The IS register is the interrupt sense register."]
pub mod is;
#[doc = "IBE register accessor: an alias for `Reg<IBE_SPEC>`"]
pub type IBE = crate::Reg<ibe::IBE_SPEC>;
#[doc = "The IBE register is the interrupt both-edges register. When the corresponding bit in IS is set to detect edges, bits set to high in IBE configure the corresponding pin to detect both rising and falling edges, regardless of the corresponding bit in the IEV (interrupt event register). Clearing a bit configures the pin to be controlled by IEV."]
pub mod ibe;
#[doc = "IEV register accessor: an alias for `Reg<IEV_SPEC>`"]
pub type IEV = crate::Reg<iev::IEV_SPEC>;
#[doc = "The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS."]
pub mod iev;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "The IE register is the interrupt mask register. Bits set to high in IE allow the corresponding pins to trigger their individual interrupts and the combined GPIOINTR line. Clearing a bit disables interrupt triggering on that pin."]
pub mod ie;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt."]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking."]
pub mod mis;
#[doc = "IC register accessor: an alias for `Reg<IC_SPEC>`"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "The IC register is the interrupt clear register. Writing 1 to a bit in this register clears the corresponding interrupt edge detection logic register. Writing 0 has no effect."]
pub mod ic;
#[doc = "AFSEL register accessor: an alias for `Reg<AFSEL_SPEC>`"]
pub type AFSEL = crate::Reg<afsel::AFSEL_SPEC>;
#[doc = "The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default."]
pub mod afsel;
#[doc = "GPIOLOCK register accessor: an alias for `Reg<GPIOLOCK_SPEC>`"]
pub type GPIOLOCK = crate::Reg<gpiolock::GPIOLOCK_SPEC>;
#[doc = "A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked."]
pub mod gpiolock;
#[doc = "GPIOCR register accessor: an alias for `Reg<GPIOCR_SPEC>`"]
pub type GPIOCR = crate::Reg<gpiocr::GPIOCR_SPEC>;
#[doc = "The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked."]
pub mod gpiocr;
#[doc = "PMUX register accessor: an alias for `Reg<PMUX_SPEC>`"]
pub type PMUX = crate::Reg<pmux::PMUX_SPEC>;
#[doc = "The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled."]
pub mod pmux;
#[doc = "P_EDGE_CTRL register accessor: an alias for `Reg<P_EDGE_CTRL_SPEC>`"]
pub type P_EDGE_CTRL = crate::Reg<p_edge_ctrl::P_EDGE_CTRL_SPEC>;
#[doc = "The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system."]
pub mod p_edge_ctrl;
#[doc = "USB_CTRL register accessor: an alias for `Reg<USB_CTRL_SPEC>`"]
pub type USB_CTRL = crate::Reg<usb_ctrl::USB_CTRL_SPEC>;
#[doc = "This register is used to control which edge of the USB controller input generates a power-up interrupt to the system."]
pub mod usb_ctrl;
#[doc = "PI_IEN register accessor: an alias for `Reg<PI_IEN_SPEC>`"]
pub type PI_IEN = crate::Reg<pi_ien::PI_IEN_SPEC>;
#[doc = "The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled."]
pub mod pi_ien;
#[doc = "IRQ_DETECT_ACK register accessor: an alias for `Reg<IRQ_DETECT_ACK_SPEC>`"]
pub type IRQ_DETECT_ACK = crate::Reg<irq_detect_ack::IRQ_DETECT_ACK_SPEC>;
#[doc = "If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1."]
pub mod irq_detect_ack;
#[doc = "USB_IRQ_ACK register accessor: an alias for `Reg<USB_IRQ_ACK_SPEC>`"]
pub type USB_IRQ_ACK = crate::Reg<usb_irq_ack::USB_IRQ_ACK_SPEC>;
#[doc = "Same functionality as IRQ_DETECT_ACK, but for USB"]
pub mod usb_irq_ack;
#[doc = "IRQ_DETECT_UNMASK register accessor: an alias for `Reg<IRQ_DETECT_UNMASK_SPEC>`"]
pub type IRQ_DETECT_UNMASK = crate::Reg<irq_detect_unmask::IRQ_DETECT_UNMASK_SPEC>;
#[doc = "Same functionality as IRQ_DETECT_ACK, but this register handles masked interrupts"]
pub mod irq_detect_unmask;

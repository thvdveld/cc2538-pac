#[doc = "Register `FR` reader"]
pub struct R(crate::R<FR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXFE` reader - UART transmit FIFO empty The meaning of this bit depends on the state of the FEN bit in the UARTLCRH register. 1: If the FIFO is disabled (FEN is 0), the transmit holding register is empty. If the FIFO is enabled (FEN is 1), the transmit FIFO is empty. 0: The transmitter has data to transmit."]
pub type TXFE_R = crate::BitReader<bool>;
#[doc = "Field `RXFF` reader - UART receive FIFO full The meaning of this bit depends on the state of the FEN bit in the UARTLCRH register. 1: If the FIFO is disabled (FEN is 0), the receive holding register is full. If the FIFO is enabled (FEN is 1), the receive FIFO is full. 0: The receiver can receive data."]
pub type RXFF_R = crate::BitReader<bool>;
#[doc = "Field `TXFF` reader - UART transmit FIFO full The meaning of this bit depends on the state of the FEN bit in the UARTLCRH register. 1: If the FIFO is disabled (FEN is 0), the transmit holding register is full. If the FIFO is enabled (FEN is 1), the transmit FIFO is full. 0: The transmitter is not full."]
pub type TXFF_R = crate::BitReader<bool>;
#[doc = "Field `RXFE` reader - UART receive FIFO empty The meaning of this bit depends on the state of the FEN bit in the UARTLCRH register. 1: If the FIFO is disabled (FEN is 0), the receive holding register is empty. If the FIFO is enabled (FEN is 1), the receive FIFO is empty. 0: The receiver is not empty."]
pub type RXFE_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - UART busy 1: The UART is busy transmitting data. This bit remains set until the complete byte, including all stop bits, has been sent from the shift register. 0: The UART is not busy. This bit is set as soon as the transmit FIFO becomes non-empty (regardless of whether UART is enabled)."]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `CTS` reader - Clear to send (UART1 only, reserved for UART0). 1: The U1CTS signal is asserted. 0: The U1CTS signal is not asserted."]
pub type CTS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 7 - UART transmit FIFO empty The meaning of this bit depends on the state of the FEN bit in the UARTLCRH register. 1: If the FIFO is disabled (FEN is 0), the transmit holding register is empty. If the FIFO is enabled (FEN is 1), the transmit FIFO is empty. 0: The transmitter has data to transmit."]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - UART receive FIFO full The meaning of this bit depends on the state of the FEN bit in the UARTLCRH register. 1: If the FIFO is disabled (FEN is 0), the receive holding register is full. If the FIFO is enabled (FEN is 1), the receive FIFO is full. 0: The receiver can receive data."]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - UART transmit FIFO full The meaning of this bit depends on the state of the FEN bit in the UARTLCRH register. 1: If the FIFO is disabled (FEN is 0), the transmit holding register is full. If the FIFO is enabled (FEN is 1), the transmit FIFO is full. 0: The transmitter is not full."]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - UART receive FIFO empty The meaning of this bit depends on the state of the FEN bit in the UARTLCRH register. 1: If the FIFO is disabled (FEN is 0), the receive holding register is empty. If the FIFO is enabled (FEN is 1), the receive FIFO is empty. 0: The receiver is not empty."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - UART busy 1: The UART is busy transmitting data. This bit remains set until the complete byte, including all stop bits, has been sent from the shift register. 0: The UART is not busy. This bit is set as soon as the transmit FIFO becomes non-empty (regardless of whether UART is enabled)."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 0 - Clear to send (UART1 only, reserved for UART0). 1: The U1CTS signal is asserted. 0: The U1CTS signal is not asserted."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "UART flag The FR register is the flag register. After reset, the TXFF, RXFF, and BUSY bits are 0, and TXFE and RXFE bits are 1. The CTS bit indicate the modem flow control. Note that the modem bits are only implemented on UART1 and are tied inactive on UART0. Due to this difference, the reset state of the UART0 FR register is 0x90, while UART1 FR register reset state 0x197 .\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](index.html) module"]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr::R](R) reader structure"]
impl crate::Readable for FR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

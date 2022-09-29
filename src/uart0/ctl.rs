#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UARTEN` reader - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
pub type UARTEN_R = crate::BitReader<bool>;
#[doc = "Field `UARTEN` writer - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
pub type UARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `SIREN` reader - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
pub type SIREN_R = crate::BitReader<bool>;
#[doc = "Field `SIREN` writer - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
pub type SIREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `SIRLP` reader - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
pub type SIRLP_R = crate::BitReader<bool>;
#[doc = "Field `SIRLP` writer - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
pub type SIRLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `EOT` reader - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
pub type EOT_R = crate::BitReader<bool>;
#[doc = "Field `EOT` writer - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
pub type EOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `HSE` reader - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
pub type HSE_R = crate::BitReader<bool>;
#[doc = "Field `HSE` writer - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
pub type HSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `LIN` reader - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
pub type LIN_R = crate::BitReader<bool>;
#[doc = "Field `LIN` writer - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
pub type LIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `LBE` reader - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
pub type LBE_R = crate::BitReader<bool>;
#[doc = "Field `LBE` writer - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
pub type LBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TXE` reader - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` writer - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RXE` reader - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
pub type RXE_R = crate::BitReader<bool>;
#[doc = "Field `RXE` writer - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
pub type RXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RTSEN` reader - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub type RTSEN_R = crate::BitReader<bool>;
#[doc = "Field `RTSEN` writer - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub type RTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CTSEN` reader - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub type CTSEN_R = crate::BitReader<bool>;
#[doc = "Field `CTSEN` writer - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub type CTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
    #[inline(always)]
    pub fn siren(&self) -> SIREN_R {
        SIREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub fn sirlp(&self) -> SIRLP_R {
        SIRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
    #[inline(always)]
    pub fn hse(&self) -> HSE_R {
        HSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
    #[inline(always)]
    pub fn lin(&self) -> LIN_R {
        LIN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
    #[inline(always)]
    pub fn lbe(&self) -> LBE_R {
        LBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn uarten(&mut self) -> UARTEN_W<0> {
        UARTEN_W::new(self)
    }
    #[doc = "Bit 1 - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
    #[inline(always)]
    pub fn siren(&mut self) -> SIREN_W<1> {
        SIREN_W::new(self)
    }
    #[doc = "Bit 2 - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub fn sirlp(&mut self) -> SIRLP_W<2> {
        SIRLP_W::new(self)
    }
    #[doc = "Bit 4 - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W<4> {
        EOT_W::new(self)
    }
    #[doc = "Bit 5 - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
    #[inline(always)]
    pub fn hse(&mut self) -> HSE_W<5> {
        HSE_W::new(self)
    }
    #[doc = "Bit 6 - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
    #[inline(always)]
    pub fn lin(&mut self) -> LIN_W<6> {
        LIN_W::new(self)
    }
    #[doc = "Bit 7 - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
    #[inline(always)]
    pub fn lbe(&mut self) -> LBE_W<7> {
        LBE_W::new(self)
    }
    #[doc = "Bit 8 - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W<8> {
        TXE_W::new(self)
    }
    #[doc = "Bit 9 - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W<9> {
        RXE_W::new(self)
    }
    #[doc = "Bit 14 - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W<14> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 15 - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W<15> {
        CTSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART control The CTL register is the control register. All the bits are cleared on reset except for the transmit enable (TXE) and receive enable (RXE) bits, which are set. To enable the UART module, the UARTEN bit must be set. If software requires a configuration change in the module, the UARTEN bit must be cleared before the configuration changes are written. If the UART is disabled during a transmit or receive operation, the current transaction is completed before the UART stopping. Note: The UARTCTL register should not be changed while the UART is enabled or else the results are unpredictable. The following sequence is recommended for making changes to the UARTCTL register: 1. Disable the UART. 2. Wait for the end of transmission or reception of the current character. 3. Flush the transmit FIFO by clearing bit 4 (FEN) in the line control register (UARTLCRH). 4. Reprogram the control register. 5. Enable the UART.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

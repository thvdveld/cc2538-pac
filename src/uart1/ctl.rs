#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `UARTEN` reader - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
pub type UartenR = crate::BitReader;
#[doc = "Field `UARTEN` writer - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
pub type UartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIREN` reader - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
pub type SirenR = crate::BitReader;
#[doc = "Field `SIREN` writer - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
pub type SirenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIRLP` reader - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
pub type SirlpR = crate::BitReader;
#[doc = "Field `SIRLP` writer - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
pub type SirlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOT` reader - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
pub type EotR = crate::BitReader;
#[doc = "Field `EOT` writer - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSE` reader - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
pub type HseR = crate::BitReader;
#[doc = "Field `HSE` writer - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
pub type HseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIN` reader - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
pub type LinR = crate::BitReader;
#[doc = "Field `LIN` writer - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
pub type LinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBE` reader - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
pub type LbeR = crate::BitReader;
#[doc = "Field `LBE` writer - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
pub type LbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
pub type RxeR = crate::BitReader;
#[doc = "Field `RXE` writer - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEN` writer - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn uarten(&self) -> UartenR {
        UartenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
    #[inline(always)]
    pub fn siren(&self) -> SirenR {
        SirenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub fn sirlp(&self) -> SirlpR {
        SirlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
    #[inline(always)]
    pub fn hse(&self) -> HseR {
        HseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
    #[inline(always)]
    pub fn lin(&self) -> LinR {
        LinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
    #[inline(always)]
    pub fn lbe(&self) -> LbeR {
        LbeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
    #[inline(always)]
    #[must_use]
    pub fn uarten(&mut self) -> UartenW<CtlSpec> {
        UartenW::new(self, 0)
    }
    #[doc = "Bit 1 - UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
    #[inline(always)]
    #[must_use]
    pub fn siren(&mut self) -> SirenW<CtlSpec> {
        SirenW::new(self, 1)
    }
    #[doc = "Bit 2 - UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    #[must_use]
    pub fn sirlp(&mut self) -> SirlpW<CtlSpec> {
        SirlpW::new(self, 2)
    }
    #[doc = "Bit 4 - End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EotW<CtlSpec> {
        EotW::new(self, 4)
    }
    #[doc = "Bit 5 - High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
    #[inline(always)]
    #[must_use]
    pub fn hse(&mut self) -> HseW<CtlSpec> {
        HseW::new(self, 5)
    }
    #[doc = "Bit 6 - LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lin(&mut self) -> LinW<CtlSpec> {
        LinW::new(self, 6)
    }
    #[doc = "Bit 7 - UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lbe(&mut self) -> LbeW<CtlSpec> {
        LbeW::new(self, 7)
    }
    #[doc = "Bit 8 - UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TxeW<CtlSpec> {
        TxeW::new(self, 8)
    }
    #[doc = "Bit 9 - UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RxeW<CtlSpec> {
        RxeW::new(self, 9)
    }
    #[doc = "Bit 14 - U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RtsenW<CtlSpec> {
        RtsenW::new(self, 14)
    }
    #[doc = "Bit 15 - U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CtsenW<CtlSpec> {
        CtsenW::new(self, 15)
    }
}
#[doc = "UART control The CTL register is the control register. All the bits are cleared on reset except for the transmit enable (TXE) and receive enable (RXE) bits, which are set. To enable the UART module, the UARTEN bit must be set. If software requires a configuration change in the module, the UARTEN bit must be cleared before the configuration changes are written. If the UART is disabled during a transmit or receive operation, the current transaction is completed before the UART stopping. Note: The UARTCTL register should not be changed while the UART is enabled or else the results are unpredictable. The following sequence is recommended for making changes to the UARTCTL register: 1. Disable the UART. 2. Wait for the end of transmission or reception of the current character. 3. Flush the transmit FIFO by clearing bit 4 (FEN) in the line control register (UARTLCRH). 4. Reprogram the control register. 5. Enable the UART.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}

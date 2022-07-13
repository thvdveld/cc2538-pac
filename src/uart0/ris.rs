#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LME5RIS` reader - LIN mode edge 5 raw interrupt status 1: The timer value at the 5th falling edge of the LIN sync field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
pub type LME5RIS_R = crate::BitReader<bool>;
#[doc = "Field `LME1RIS` reader - LIN mode edge 1 raw interrupt status 1: The timer value at the 1st falling edge of the LIN Sync Field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
pub type LME1RIS_R = crate::BitReader<bool>;
#[doc = "Field `LMSBRIS` reader - LIN mode sync break raw interrupt status 1: A LIN sync break has been detected. 0: No interrupt This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
pub type LMSBRIS_R = crate::BitReader<bool>;
#[doc = "Field `NINEBITRIS` reader - 9-mit mode raw interrupt status 1: A receive address match has occurred. 0: No interrupt This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
pub type NINEBITRIS_R = crate::BitReader<bool>;
#[doc = "Field `OERIS` reader - UART overrun error raw interrupt status 1: An overrun error has occurred. 0: No interrupt This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
pub type OERIS_R = crate::BitReader<bool>;
#[doc = "Field `BERIS` reader - UART break error raw interrupt status 1: A break error has occurred. 0: No interrupt This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
pub type BERIS_R = crate::BitReader<bool>;
#[doc = "Field `PERIS` reader - UART parity error raw interrupt status 1: A parity error has occurred. 0: No interrupt This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
pub type PERIS_R = crate::BitReader<bool>;
#[doc = "Field `FERIS` reader - UART framing error raw interrupt status 1: A framing error has occurred. 0: No interrupt This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
pub type FERIS_R = crate::BitReader<bool>;
#[doc = "Field `RTRIS` reader - UART receive time-out raw interrupt status 1: A receive time out has occurred. 0: No interrupt This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
pub type RTRIS_R = crate::BitReader<bool>;
#[doc = "Field `TXRIS` reader - UART transmit raw interrupt status 1: If the EOT bit in the UARTCTL register is clear, the transmit FIFO level has passed through the condition defined in the UARTIFLS register. If the EOT bit is set, the last bit of all transmitted data and flags has left the serializer. 0: No interrupt This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
pub type TXRIS_R = crate::BitReader<bool>;
#[doc = "Field `RXRIS` reader - UART receive raw interrupt status 1: The receive FIFO level has passed through the condition defined in the UARTIFLS register. 0: No interrupt This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
pub type RXRIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 15 - LIN mode edge 5 raw interrupt status 1: The timer value at the 5th falling edge of the LIN sync field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme5ris(&self) -> LME5RIS_R {
        LME5RIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN mode edge 1 raw interrupt status 1: The timer value at the 1st falling edge of the LIN Sync Field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme1ris(&self) -> LME1RIS_R {
        LME1RIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - LIN mode sync break raw interrupt status 1: A LIN sync break has been detected. 0: No interrupt This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lmsbris(&self) -> LMSBRIS_R {
        LMSBRIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-mit mode raw interrupt status 1: A receive address match has occurred. 0: No interrupt This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn ninebitris(&self) -> NINEBITRIS_R {
        NINEBITRIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 10 - UART overrun error raw interrupt status 1: An overrun error has occurred. 0: No interrupt This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - UART break error raw interrupt status 1: A break error has occurred. 0: No interrupt This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - UART parity error raw interrupt status 1: A parity error has occurred. 0: No interrupt This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - UART framing error raw interrupt status 1: A framing error has occurred. 0: No interrupt This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - UART receive time-out raw interrupt status 1: A receive time out has occurred. 0: No interrupt This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - UART transmit raw interrupt status 1: If the EOT bit in the UARTCTL register is clear, the transmit FIFO level has passed through the condition defined in the UARTIFLS register. If the EOT bit is set, the last bit of all transmitted data and flags has left the serializer. 0: No interrupt This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - UART receive raw interrupt status 1: The receive FIFO level has passed through the condition defined in the UARTIFLS register. 0: No interrupt This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "UART raw interrupt status The RIS register is the raw interrupt status register. On a read, this register gives the current raw status value of the corresponding interrupt. A write has no effect. Note that the HW modem flow control bits are only implemented on UART1 and are tied inactive on UART0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

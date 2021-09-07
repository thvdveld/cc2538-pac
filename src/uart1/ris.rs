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
pub struct LME5RIS_R(crate::FieldReader<bool, bool>);
impl LME5RIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LME5RIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LME5RIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LME1RIS` reader - LIN mode edge 1 raw interrupt status 1: The timer value at the 1st falling edge of the LIN Sync Field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
pub struct LME1RIS_R(crate::FieldReader<bool, bool>);
impl LME1RIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LME1RIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LME1RIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LMSBRIS` reader - LIN mode sync break raw interrupt status 1: A LIN sync break has been detected. 0: No interrupt This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
pub struct LMSBRIS_R(crate::FieldReader<bool, bool>);
impl LMSBRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LMSBRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LMSBRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NINEBITRIS` reader - 9-mit mode raw interrupt status 1: A receive address match has occurred. 0: No interrupt This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
pub struct NINEBITRIS_R(crate::FieldReader<bool, bool>);
impl NINEBITRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NINEBITRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NINEBITRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OERIS` reader - UART overrun error raw interrupt status 1: An overrun error has occurred. 0: No interrupt This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
pub struct OERIS_R(crate::FieldReader<bool, bool>);
impl OERIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERIS` reader - UART break error raw interrupt status 1: A break error has occurred. 0: No interrupt This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
pub struct BERIS_R(crate::FieldReader<bool, bool>);
impl BERIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIS` reader - UART parity error raw interrupt status 1: A parity error has occurred. 0: No interrupt This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
pub struct PERIS_R(crate::FieldReader<bool, bool>);
impl PERIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FERIS` reader - UART framing error raw interrupt status 1: A framing error has occurred. 0: No interrupt This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
pub struct FERIS_R(crate::FieldReader<bool, bool>);
impl FERIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FERIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTRIS` reader - UART receive time-out raw interrupt status 1: A receive time out has occurred. 0: No interrupt This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
pub struct RTRIS_R(crate::FieldReader<bool, bool>);
impl RTRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRIS` reader - UART transmit raw interrupt status 1: If the EOT bit in the UARTCTL register is clear, the transmit FIFO level has passed through the condition defined in the UARTIFLS register. If the EOT bit is set, the last bit of all transmitted data and flags has left the serializer. 0: No interrupt This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
pub struct TXRIS_R(crate::FieldReader<bool, bool>);
impl TXRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRIS` reader - UART receive raw interrupt status 1: The receive FIFO level has passed through the condition defined in the UARTIFLS register. 0: No interrupt This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
pub struct RXRIS_R(crate::FieldReader<bool, bool>);
impl RXRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15 - LIN mode edge 5 raw interrupt status 1: The timer value at the 5th falling edge of the LIN sync field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme5ris(&self) -> LME5RIS_R {
        LME5RIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LIN mode edge 1 raw interrupt status 1: The timer value at the 1st falling edge of the LIN Sync Field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme1ris(&self) -> LME1RIS_R {
        LME1RIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LIN mode sync break raw interrupt status 1: A LIN sync break has been detected. 0: No interrupt This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lmsbris(&self) -> LMSBRIS_R {
        LMSBRIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 9-mit mode raw interrupt status 1: A receive address match has occurred. 0: No interrupt This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn ninebitris(&self) -> NINEBITRIS_R {
        NINEBITRIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART overrun error raw interrupt status 1: An overrun error has occurred. 0: No interrupt This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART break error raw interrupt status 1: A break error has occurred. 0: No interrupt This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART parity error raw interrupt status 1: A parity error has occurred. 0: No interrupt This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART framing error raw interrupt status 1: A framing error has occurred. 0: No interrupt This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART receive time-out raw interrupt status 1: A receive time out has occurred. 0: No interrupt This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART transmit raw interrupt status 1: If the EOT bit in the UARTCTL register is clear, the transmit FIFO level has passed through the condition defined in the UARTIFLS register. If the EOT bit is set, the last bit of all transmitted data and flags has left the serializer. 0: No interrupt This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART receive raw interrupt status 1: The receive FIFO level has passed through the condition defined in the UARTIFLS register. 0: No interrupt This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
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

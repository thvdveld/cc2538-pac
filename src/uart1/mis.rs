#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LME5MIS` reader - LIN mode edge 5 masked interrupt status 1: An unmasked interrupt was signaled due to the 5th falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
pub struct LME5MIS_R(crate::FieldReader<bool, bool>);
impl LME5MIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LME5MIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LME5MIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LME1MIS` reader - LIN mode edge 1 masked interrupt status 1: An unmasked interrupt was signaled due to the 1st falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
pub struct LME1MIS_R(crate::FieldReader<bool, bool>);
impl LME1MIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LME1MIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LME1MIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LMSBMIS` reader - LIN mode sync break masked interrupt status 1: An unmasked interrupt was signaled due to the receipt of a LIN sync break. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
pub struct LMSBMIS_R(crate::FieldReader<bool, bool>);
impl LMSBMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LMSBMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LMSBMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NINEBITMIS` reader - 9-bit mode masked interrupt status 1: An unmasked interrupt was signaled due to a receive address match. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
pub struct NINEBITMIS_R(crate::FieldReader<bool, bool>);
impl NINEBITMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NINEBITMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NINEBITMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEMIS` reader - UART overrun error masked interrupt status 1: An unmasked interrupt was signaled due to an overrun error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
pub struct OEMIS_R(crate::FieldReader<bool, bool>);
impl OEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEMIS` reader - UART break error masked interrupt status 1: An unmasked interrupt was signaled due to a break error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
pub struct BEMIS_R(crate::FieldReader<bool, bool>);
impl BEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEMIS` reader - UART parity error masked interrupt status 1: An unmasked interrupt was signaled due to a parity error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
pub struct PEMIS_R(crate::FieldReader<bool, bool>);
impl PEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEMIS` reader - UART framing error masked interrupt status 1: An unmasked interrupt was signaled due to a framing error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
pub struct FEMIS_R(crate::FieldReader<bool, bool>);
impl FEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTMIS` reader - UART receive time-out masked interrupt status 1: An unmasked interrupt was signaled due to a receive time out. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
pub struct RTMIS_R(crate::FieldReader<bool, bool>);
impl RTMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMIS` reader - UART transmit masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified transmit FIFO level (if the EOT bit is clear) or due to the transmission of the last data bit (if the EOT bit is set). 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
pub struct TXMIS_R(crate::FieldReader<bool, bool>);
impl TXMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMIS` reader - UART receive masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified receive FIFO level. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
pub struct RXMIS_R(crate::FieldReader<bool, bool>);
impl RXMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15 - LIN mode edge 5 masked interrupt status 1: An unmasked interrupt was signaled due to the 5th falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme5mis(&self) -> LME5MIS_R {
        LME5MIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LIN mode edge 1 masked interrupt status 1: An unmasked interrupt was signaled due to the 1st falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme1mis(&self) -> LME1MIS_R {
        LME1MIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LIN mode sync break masked interrupt status 1: An unmasked interrupt was signaled due to the receipt of a LIN sync break. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lmsbmis(&self) -> LMSBMIS_R {
        LMSBMIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 9-bit mode masked interrupt status 1: An unmasked interrupt was signaled due to a receive address match. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn ninebitmis(&self) -> NINEBITMIS_R {
        NINEBITMIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART overrun error masked interrupt status 1: An unmasked interrupt was signaled due to an overrun error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART break error masked interrupt status 1: An unmasked interrupt was signaled due to a break error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART parity error masked interrupt status 1: An unmasked interrupt was signaled due to a parity error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART framing error masked interrupt status 1: An unmasked interrupt was signaled due to a framing error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART receive time-out masked interrupt status 1: An unmasked interrupt was signaled due to a receive time out. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART transmit masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified transmit FIFO level (if the EOT bit is clear) or due to the transmission of the last data bit (if the EOT bit is set). 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART receive masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified receive FIFO level. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "UART masked interrupt status The MIS register is the masked interrupt status register. On a read, this register gives the current masked status value of the corresponding interrupt. A write has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

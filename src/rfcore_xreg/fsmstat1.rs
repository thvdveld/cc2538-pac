#[doc = "Register `FSMSTAT1` reader"]
pub struct R(crate::R<FSMSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSMSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSMSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSMSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO` reader - FIFO is high when there is data in the RX FIFO. FIFO is low during RX FIFO overflow."]
pub struct FIFO_R(crate::FieldReader<bool, bool>);
impl FIFO_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOP` reader - FIFOP is set high when there are at more than FIFOP_THR bytes of data in the RX FIFO that has passed frame filtering. FIFOP is set high when there is at least one complete frame in the RX FIFO. FIFOP is high during RX FIFO overflow."]
pub struct FIFOP_R(crate::FieldReader<bool, bool>);
impl FIFOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFD` reader - In TX 0: When a complete frame with SFD was sent or no SFD was sent 1: SFD was sent. In RX 0: When a complete frame was received or no SFD was received 1: SFD was received."]
pub struct SFD_R(crate::FieldReader<bool, bool>);
impl SFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCA` reader - Clear channel assessment Dependent on CCA_MODE settings. See CCACTRL1 for details."]
pub struct CCA_R(crate::FieldReader<bool, bool>);
impl CCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLED_CCA` reader - Contains a sampled value of the CCA The value is updated when a SSAMPLECCA or STXONCCA strobe is issued."]
pub struct SAMPLED_CCA_R(crate::FieldReader<bool, bool>);
impl SAMPLED_CCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAMPLED_CCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPLED_CCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_STATUS` reader - 1 when PLL is in lock; otherwise 0"]
pub struct LOCK_STATUS_R(crate::FieldReader<bool, bool>);
impl LOCK_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ACTIVE` reader - Status signal Active when FFC is in one of the transmit states"]
pub struct TX_ACTIVE_R(crate::FieldReader<bool, bool>);
impl TX_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ACTIVE` reader - Status signal Active when FFC is in one of the receive states"]
pub struct RX_ACTIVE_R(crate::FieldReader<bool, bool>);
impl RX_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - FIFO is high when there is data in the RX FIFO. FIFO is low during RX FIFO overflow."]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FIFOP is set high when there are at more than FIFOP_THR bytes of data in the RX FIFO that has passed frame filtering. FIFOP is set high when there is at least one complete frame in the RX FIFO. FIFOP is high during RX FIFO overflow."]
    #[inline(always)]
    pub fn fifop(&self) -> FIFOP_R {
        FIFOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - In TX 0: When a complete frame with SFD was sent or no SFD was sent 1: SFD was sent. In RX 0: When a complete frame was received or no SFD was received 1: SFD was received."]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear channel assessment Dependent on CCA_MODE settings. See CCACTRL1 for details."]
    #[inline(always)]
    pub fn cca(&self) -> CCA_R {
        CCA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Contains a sampled value of the CCA The value is updated when a SSAMPLECCA or STXONCCA strobe is issued."]
    #[inline(always)]
    pub fn sampled_cca(&self) -> SAMPLED_CCA_R {
        SAMPLED_CCA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1 when PLL is in lock; otherwise 0"]
    #[inline(always)]
    pub fn lock_status(&self) -> LOCK_STATUS_R {
        LOCK_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status signal Active when FFC is in one of the transmit states"]
    #[inline(always)]
    pub fn tx_active(&self) -> TX_ACTIVE_R {
        TX_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Status signal Active when FFC is in one of the receive states"]
    #[inline(always)]
    pub fn rx_active(&self) -> RX_ACTIVE_R {
        RX_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Radio status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsmstat1](index.html) module"]
pub struct FSMSTAT1_SPEC;
impl crate::RegisterSpec for FSMSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsmstat1::R](R) reader structure"]
impl crate::Readable for FSMSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSMSTAT1 to value 0"]
impl crate::Resettable for FSMSTAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

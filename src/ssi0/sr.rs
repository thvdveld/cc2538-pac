#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BSY` reader - SSI busy bit (RO) Reset value: 0x0 0: SSI is idle. 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub struct BSY_R(crate::FieldReader<bool, bool>);
impl BSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFF` reader - SSI receive FIFO full (RO) Reset value: 0x0 0: Receive FIFO is not full. 1: Receive FIFO is full."]
pub struct RFF_R(crate::FieldReader<bool, bool>);
impl RFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNE` reader - SSI receive FIFO not empty (RO) Reset value: 0x0 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
pub struct RNE_R(crate::FieldReader<bool, bool>);
impl RNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TNF` reader - SSI transmit FIFO not full (RO) Reset value: 0x1 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
pub struct TNF_R(crate::FieldReader<bool, bool>);
impl TNF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFE` reader - SSI transmit FIFO empty (RO) Reset value: 0x1 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
pub struct TFE_R(crate::FieldReader<bool, bool>);
impl TFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - SSI busy bit (RO) Reset value: 0x0 0: SSI is idle. 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI receive FIFO full (RO) Reset value: 0x0 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI receive FIFO not empty (RO) Reset value: 0x0 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI transmit FIFO not full (RO) Reset value: 0x1 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SSI transmit FIFO empty (RO) Reset value: 0x1 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "The SR register contains bits that indicate the FIFO fill status and the SSI busy status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

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
#[doc = "Field `TBMMIS` reader - GPTM Timer B match masked interrupt"]
pub struct TBMMIS_R(crate::FieldReader<bool, bool>);
impl TBMMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBEMIS` reader - GPTM Timer B capture event masked interrupt"]
pub struct CBEMIS_R(crate::FieldReader<bool, bool>);
impl CBEMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBMMIS` reader - GPTM Timer B capture match masked interrupt"]
pub struct CBMMIS_R(crate::FieldReader<bool, bool>);
impl CBMMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBTOMIS` reader - GPTM Timer B time-out masked interrupt"]
pub struct TBTOMIS_R(crate::FieldReader<bool, bool>);
impl TBTOMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBTOMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBTOMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMRIS` reader - GPTM Timer A match raw interrupt"]
pub struct TAMRIS_R(crate::FieldReader<bool, bool>);
impl TAMRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAEMIS` reader - GPTM Timer A capture event raw interrupt"]
pub struct CAEMIS_R(crate::FieldReader<bool, bool>);
impl CAEMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAMMIS` reader - GPTM Timer A capture match raw interrupt"]
pub struct CAMMIS_R(crate::FieldReader<bool, bool>);
impl CAMMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TATOMIS` reader - GPTM Timer A time-out raw interrupt"]
pub struct TATOMIS_R(crate::FieldReader<bool, bool>);
impl TATOMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TATOMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TATOMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 11 - GPTM Timer B match masked interrupt"]
    #[inline(always)]
    pub fn tbmmis(&self) -> TBMMIS_R {
        TBMMIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event masked interrupt"]
    #[inline(always)]
    pub fn cbemis(&self) -> CBEMIS_R {
        CBEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match masked interrupt"]
    #[inline(always)]
    pub fn cbmmis(&self) -> CBMMIS_R {
        CBMMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out masked interrupt"]
    #[inline(always)]
    pub fn tbtomis(&self) -> TBTOMIS_R {
        TBTOMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match raw interrupt"]
    #[inline(always)]
    pub fn tamris(&self) -> TAMRIS_R {
        TAMRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event raw interrupt"]
    #[inline(always)]
    pub fn caemis(&self) -> CAEMIS_R {
        CAEMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match raw interrupt"]
    #[inline(always)]
    pub fn cammis(&self) -> CAMMIS_R {
        CAMMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPTM Timer A time-out raw interrupt"]
    #[inline(always)]
    pub fn tatomis(&self) -> TATOMIS_R {
        TATOMIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
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

#[doc = "Register `CIF` reader"]
pub struct R(crate::R<CIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOFIF` reader - Start-of-frame interrupt flag Cleared by hardware when read"]
pub struct SOFIF_R(crate::FieldReader<bool, bool>);
impl SOFIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTIF` reader - Reset interrupt flag Cleared by hardware when read"]
pub struct RSTIF_R(crate::FieldReader<bool, bool>);
impl RSTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUMEIF` reader - Resume interrupt flag Cleared by hardware when read"]
pub struct RESUMEIF_R(crate::FieldReader<bool, bool>);
impl RESUMEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUMEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUMEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPENDIF` reader - Suspend interrupt flag Cleared by hardware when read"]
pub struct SUSPENDIF_R(crate::FieldReader<bool, bool>);
impl SUSPENDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPENDIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPENDIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - Start-of-frame interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn rstif(&self) -> RSTIF_R {
        RSTIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Resume interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn resumeif(&self) -> RESUMEIF_R {
        RESUMEIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Suspend interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn suspendif(&self) -> SUSPENDIF_R {
        SUSPENDIF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Common USB interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cif](index.html) module"]
pub struct CIF_SPEC;
impl crate::RegisterSpec for CIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cif::R](R) reader structure"]
impl crate::Readable for CIF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIF to value 0"]
impl crate::Resettable for CIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

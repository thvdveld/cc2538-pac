#[doc = "Register `STLOAD` reader"]
pub struct R(crate::R<STLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STLOAD` reader - Status signal for when STx registers have been uploaded to 32-kHz counter. 1: Load is complete 0: Load is busy and STx regs are blocked for writing"]
pub struct STLOAD_R(crate::FieldReader<bool, bool>);
impl STLOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STLOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STLOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Status signal for when STx registers have been uploaded to 32-kHz counter. 1: Load is complete 0: Load is busy and STx regs are blocked for writing"]
    #[inline(always)]
    pub fn stload(&self) -> STLOAD_R {
        STLOAD_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Sleep Timer load status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stload](index.html) module"]
pub struct STLOAD_SPEC;
impl crate::RegisterSpec for STLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stload::R](R) reader structure"]
impl crate::Readable for STLOAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STLOAD to value 0"]
impl crate::Resettable for STLOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `STCV1` reader"]
pub struct R(crate::R<STCV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STCV1` reader - Bits \\[15:8\\]
of Sleep Timer capture value"]
pub struct STCV1_R(crate::FieldReader<u8, u8>);
impl STCV1_R {
    pub(crate) fn new(bits: u8) -> Self {
        STCV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STCV1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Bits \\[15:8\\]
of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv1(&self) -> STCV1_R {
        STCV1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Sleep Timer Capture value byte 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcv1](index.html) module"]
pub struct STCV1_SPEC;
impl crate::RegisterSpec for STCV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcv1::R](R) reader structure"]
impl crate::Readable for STCV1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STCV1 to value 0"]
impl crate::Resettable for STCV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

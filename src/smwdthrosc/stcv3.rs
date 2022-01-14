#[doc = "Register `STCV3` reader"]
pub struct R(crate::R<STCV3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCV3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCV3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCV3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STCV3` reader - Bits \\[32:24\\]
of Sleep Timer capture value"]
pub struct STCV3_R(crate::FieldReader<u8, u8>);
impl STCV3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STCV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STCV3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Bits \\[32:24\\]
of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv3(&self) -> STCV3_R {
        STCV3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Sleep Timer Capture value byte 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcv3](index.html) module"]
pub struct STCV3_SPEC;
impl crate::RegisterSpec for STCV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcv3::R](R) reader structure"]
impl crate::Readable for STCV3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STCV3 to value 0"]
impl crate::Resettable for STCV3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

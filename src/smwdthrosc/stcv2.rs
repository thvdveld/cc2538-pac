#[doc = "Register `STCV2` reader"]
pub struct R(crate::R<STCV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STCV2` reader - Bits \\[23:16\\]
of Sleep Timer capture value"]
pub type STCV2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Bits \\[23:16\\]
of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv2(&self) -> STCV2_R {
        STCV2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Sleep Timer Capture value byte 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcv2](index.html) module"]
pub struct STCV2_SPEC;
impl crate::RegisterSpec for STCV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcv2::R](R) reader structure"]
impl crate::Readable for STCV2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STCV2 to value 0"]
impl crate::Resettable for STCV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

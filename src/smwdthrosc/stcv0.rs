#[doc = "Register `STCV0` reader"]
pub struct R(crate::R<STCV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STCV0` reader - Bits \\[7:0\\]
of Sleep Timer capture value"]
pub type STCV0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Bits \\[7:0\\]
of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv0(&self) -> STCV0_R {
        STCV0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Sleep Timer Capture value byte 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcv0](index.html) module"]
pub struct STCV0_SPEC;
impl crate::RegisterSpec for STCV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcv0::R](R) reader structure"]
impl crate::Readable for STCV0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STCV0 to value 0"]
impl crate::Resettable for STCV0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `CSPY` reader"]
pub struct R(crate::R<CSPY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSPY` reader - Used by CSP instructions RANDXY, INCY, DECY, and conditional instructions."]
pub type CSPY_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Used by CSP instructions RANDXY, INCY, DECY, and conditional instructions."]
    #[inline(always)]
    pub fn cspy(&self) -> CSPY_R {
        CSPY_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP Y data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspy](index.html) module"]
pub struct CSPY_SPEC;
impl crate::RegisterSpec for CSPY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspy::R](R) reader structure"]
impl crate::Readable for CSPY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSPY to value 0"]
impl crate::Resettable for CSPY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

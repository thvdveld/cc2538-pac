#[doc = "Register `CSPX` reader"]
pub struct R(crate::R<CSPX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSPX` reader - Used by CSP instructions WAITX, RANDXY, INCX, DECX, and conditional instructions."]
pub type CSPX_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Used by CSP instructions WAITX, RANDXY, INCX, DECX, and conditional instructions."]
    #[inline(always)]
    pub fn cspx(&self) -> CSPX_R {
        CSPX_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP X data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspx](index.html) module"]
pub struct CSPX_SPEC;
impl crate::RegisterSpec for CSPX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspx::R](R) reader structure"]
impl crate::Readable for CSPX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSPX to value 0"]
impl crate::Resettable for CSPX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

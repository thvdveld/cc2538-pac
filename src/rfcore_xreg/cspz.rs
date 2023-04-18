#[doc = "Register `CSPZ` reader"]
pub struct R(crate::R<CSPZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSPZ` reader - Used by CSP instructions INCZ, DECZ, and conditional instructions."]
pub type CSPZ_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Used by CSP instructions INCZ, DECZ, and conditional instructions."]
    #[inline(always)]
    pub fn cspz(&self) -> CSPZ_R {
        CSPZ_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP Z data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspz](index.html) module"]
pub struct CSPZ_SPEC;
impl crate::RegisterSpec for CSPZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspz::R](R) reader structure"]
impl crate::Readable for CSPZ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSPZ to value 0"]
impl crate::Resettable for CSPZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CSPX` reader"]
pub type R = crate::R<CSPX_SPEC>;
#[doc = "Field `CSPX` reader - Used by CSP instructions WAITX, RANDXY, INCX, DECX, and conditional instructions."]
pub type CSPX_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Used by CSP instructions WAITX, RANDXY, INCX, DECX, and conditional instructions."]
    #[inline(always)]
    pub fn cspx(&self) -> CSPX_R {
        CSPX_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP X data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSPX_SPEC;
impl crate::RegisterSpec for CSPX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspx::R`](R) reader structure"]
impl crate::Readable for CSPX_SPEC {}
#[doc = "`reset()` method sets CSPX to value 0"]
impl crate::Resettable for CSPX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

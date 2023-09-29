#[doc = "Register `CSPZ` reader"]
pub type R = crate::R<CSPZ_SPEC>;
#[doc = "Field `CSPZ` reader - Used by CSP instructions INCZ, DECZ, and conditional instructions."]
pub type CSPZ_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Used by CSP instructions INCZ, DECZ, and conditional instructions."]
    #[inline(always)]
    pub fn cspz(&self) -> CSPZ_R {
        CSPZ_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP Z data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspz::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSPZ_SPEC;
impl crate::RegisterSpec for CSPZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspz::R`](R) reader structure"]
impl crate::Readable for CSPZ_SPEC {}
#[doc = "`reset()` method sets CSPZ to value 0"]
impl crate::Resettable for CSPZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CSPZ` reader"]
pub type R = crate::R<CspzSpec>;
#[doc = "Field `CSPZ` reader - Used by CSP instructions INCZ, DECZ, and conditional instructions."]
pub type CspzR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Used by CSP instructions INCZ, DECZ, and conditional instructions."]
    #[inline(always)]
    pub fn cspz(&self) -> CspzR {
        CspzR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP Z data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CspzSpec;
impl crate::RegisterSpec for CspzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspz::R`](R) reader structure"]
impl crate::Readable for CspzSpec {}
#[doc = "`reset()` method sets CSPZ to value 0"]
impl crate::Resettable for CspzSpec {
    const RESET_VALUE: u32 = 0;
}

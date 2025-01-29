#[doc = "Register `CSPX` reader"]
pub type R = crate::R<CspxSpec>;
#[doc = "Field `CSPX` reader - Used by CSP instructions WAITX, RANDXY, INCX, DECX, and conditional instructions."]
pub type CspxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Used by CSP instructions WAITX, RANDXY, INCX, DECX, and conditional instructions."]
    #[inline(always)]
    pub fn cspx(&self) -> CspxR {
        CspxR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP X data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CspxSpec;
impl crate::RegisterSpec for CspxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspx::R`](R) reader structure"]
impl crate::Readable for CspxSpec {}
#[doc = "`reset()` method sets CSPX to value 0"]
impl crate::Resettable for CspxSpec {
    const RESET_VALUE: u32 = 0;
}

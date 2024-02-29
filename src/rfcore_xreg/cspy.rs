#[doc = "Register `CSPY` reader"]
pub type R = crate::R<CspySpec>;
#[doc = "Field `CSPY` reader - Used by CSP instructions RANDXY, INCY, DECY, and conditional instructions."]
pub type CspyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Used by CSP instructions RANDXY, INCY, DECY, and conditional instructions."]
    #[inline(always)]
    pub fn cspy(&self) -> CspyR {
        CspyR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP Y data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CspySpec;
impl crate::RegisterSpec for CspySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspy::R`](R) reader structure"]
impl crate::Readable for CspySpec {}
#[doc = "`reset()` method sets CSPY to value 0"]
impl crate::Resettable for CspySpec {
    const RESET_VALUE: u32 = 0;
}

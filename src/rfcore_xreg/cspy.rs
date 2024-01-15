#[doc = "Register `CSPY` reader"]
pub type R = crate::R<CSPY_SPEC>;
#[doc = "Field `CSPY` reader - Used by CSP instructions RANDXY, INCY, DECY, and conditional instructions."]
pub type CSPY_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Used by CSP instructions RANDXY, INCY, DECY, and conditional instructions."]
    #[inline(always)]
    pub fn cspy(&self) -> CSPY_R {
        CSPY_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP Y data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSPY_SPEC;
impl crate::RegisterSpec for CSPY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspy::R`](R) reader structure"]
impl crate::Readable for CSPY_SPEC {}
#[doc = "`reset()` method sets CSPY to value 0"]
impl crate::Resettable for CSPY_SPEC {
    const RESET_VALUE: u32 = 0;
}

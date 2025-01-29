#[doc = "Register `CSPPROG_16` reader"]
pub type R = crate::R<Cspprog16Spec>;
#[doc = "Field `CSP_INSTR` reader - Byte N of the CSP program memory"]
pub type CspInstrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte N of the CSP program memory"]
    #[inline(always)]
    pub fn csp_instr(&self) -> CspInstrR {
        CspInstrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP program\n\nYou can [`read`](crate::Reg::read) this register and get [`cspprog_16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cspprog16Spec;
impl crate::RegisterSpec for Cspprog16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspprog_16::R`](R) reader structure"]
impl crate::Readable for Cspprog16Spec {}
#[doc = "`reset()` method sets CSPPROG_16 to value 0"]
impl crate::Resettable for Cspprog16Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CSPPROG_21` reader"]
pub type R = crate::R<CSPPROG_21_SPEC>;
#[doc = "Field `CSP_INSTR` reader - Byte N of the CSP program memory"]
pub type CSP_INSTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte N of the CSP program memory"]
    #[inline(always)]
    pub fn csp_instr(&self) -> CSP_INSTR_R {
        CSP_INSTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP program\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspprog_21::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSPPROG_21_SPEC;
impl crate::RegisterSpec for CSPPROG_21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspprog_21::R`](R) reader structure"]
impl crate::Readable for CSPPROG_21_SPEC {}
#[doc = "`reset()` method sets CSPPROG_21 to value 0"]
impl crate::Resettable for CSPPROG_21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

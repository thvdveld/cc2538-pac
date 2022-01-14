#[doc = "Register `CSPPROG_22` reader"]
pub struct R(crate::R<CSPPROG_22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPPROG_22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPPROG_22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPPROG_22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSP_INSTR` reader - Byte N of the CSP program memory"]
pub struct CSP_INSTR_R(crate::FieldReader<u8, u8>);
impl CSP_INSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSP_INSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSP_INSTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Byte N of the CSP program memory"]
    #[inline(always)]
    pub fn csp_instr(&self) -> CSP_INSTR_R {
        CSP_INSTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspprog_22](index.html) module"]
pub struct CSPPROG_22_SPEC;
impl crate::RegisterSpec for CSPPROG_22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspprog_22::R](R) reader structure"]
impl crate::Readable for CSPPROG_22_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSPPROG_22 to value 0"]
impl crate::Resettable for CSPPROG_22_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

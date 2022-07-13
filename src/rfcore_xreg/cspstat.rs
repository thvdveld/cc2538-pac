#[doc = "Register `CSPSTAT` reader"]
pub struct R(crate::R<CSPSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSP_RUNNING` reader - 1: CSP is running. 0: CSP is idle."]
pub type CSP_RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `CSP_PC` reader - CSP program counter"]
pub type CSP_PC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 5 - 1: CSP is running. 0: CSP is idle."]
    #[inline(always)]
    pub fn csp_running(&self) -> CSP_RUNNING_R {
        CSP_RUNNING_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:4 - CSP program counter"]
    #[inline(always)]
    pub fn csp_pc(&self) -> CSP_PC_R {
        CSP_PC_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "CSP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspstat](index.html) module"]
pub struct CSPSTAT_SPEC;
impl crate::RegisterSpec for CSPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspstat::R](R) reader structure"]
impl crate::Readable for CSPSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSPSTAT to value 0"]
impl crate::Resettable for CSPSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

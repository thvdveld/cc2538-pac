#[doc = "Register `CSPSTAT` reader"]
pub type R = crate::R<CSPSTAT_SPEC>;
#[doc = "Field `CSP_PC` reader - CSP program counter"]
pub type CSP_PC_R = crate::FieldReader;
#[doc = "Field `CSP_RUNNING` reader - 1: CSP is running. 0: CSP is idle."]
pub type CSP_RUNNING_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - CSP program counter"]
    #[inline(always)]
    pub fn csp_pc(&self) -> CSP_PC_R {
        CSP_PC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 1: CSP is running. 0: CSP is idle."]
    #[inline(always)]
    pub fn csp_running(&self) -> CSP_RUNNING_R {
        CSP_RUNNING_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "CSP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSPSTAT_SPEC;
impl crate::RegisterSpec for CSPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspstat::R`](R) reader structure"]
impl crate::Readable for CSPSTAT_SPEC {}
#[doc = "`reset()` method sets CSPSTAT to value 0"]
impl crate::Resettable for CSPSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}

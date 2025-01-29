#[doc = "Register `CSPSTAT` reader"]
pub type R = crate::R<CspstatSpec>;
#[doc = "Field `CSP_PC` reader - CSP program counter"]
pub type CspPcR = crate::FieldReader;
#[doc = "Field `CSP_RUNNING` reader - 1: CSP is running. 0: CSP is idle."]
pub type CspRunningR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - CSP program counter"]
    #[inline(always)]
    pub fn csp_pc(&self) -> CspPcR {
        CspPcR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 1: CSP is running. 0: CSP is idle."]
    #[inline(always)]
    pub fn csp_running(&self) -> CspRunningR {
        CspRunningR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "CSP status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CspstatSpec;
impl crate::RegisterSpec for CspstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspstat::R`](R) reader structure"]
impl crate::Readable for CspstatSpec {}
#[doc = "`reset()` method sets CSPSTAT to value 0"]
impl crate::Resettable for CspstatSpec {
    const RESET_VALUE: u32 = 0;
}

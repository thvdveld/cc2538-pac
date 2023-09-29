#[doc = "Register `STLOAD` reader"]
pub type R = crate::R<STLOAD_SPEC>;
#[doc = "Field `STLOAD` reader - Status signal for when STx registers have been uploaded to 32-kHz counter. 1: Load is complete 0: Load is busy and STx regs are blocked for writing"]
pub type STLOAD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status signal for when STx registers have been uploaded to 32-kHz counter. 1: Load is complete 0: Load is busy and STx regs are blocked for writing"]
    #[inline(always)]
    pub fn stload(&self) -> STLOAD_R {
        STLOAD_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Sleep Timer load status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stload::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STLOAD_SPEC;
impl crate::RegisterSpec for STLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stload::R`](R) reader structure"]
impl crate::Readable for STLOAD_SPEC {}
#[doc = "`reset()` method sets STLOAD to value 0"]
impl crate::Resettable for STLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

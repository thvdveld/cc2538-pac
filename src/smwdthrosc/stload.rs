#[doc = "Register `STLOAD` reader"]
pub type R = crate::R<StloadSpec>;
#[doc = "Field `STLOAD` reader - Status signal for when STx registers have been uploaded to 32-kHz counter. 1: Load is complete 0: Load is busy and STx regs are blocked for writing"]
pub type StloadR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status signal for when STx registers have been uploaded to 32-kHz counter. 1: Load is complete 0: Load is busy and STx regs are blocked for writing"]
    #[inline(always)]
    pub fn stload(&self) -> StloadR {
        StloadR::new((self.bits & 1) != 0)
    }
}
#[doc = "Sleep Timer load status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stload::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StloadSpec;
impl crate::RegisterSpec for StloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stload::R`](R) reader structure"]
impl crate::Readable for StloadSpec {}
#[doc = "`reset()` method sets STLOAD to value 0"]
impl crate::Resettable for StloadSpec {
    const RESET_VALUE: u32 = 0;
}

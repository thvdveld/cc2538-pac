#[doc = "Register `PWRDBG` reader"]
pub type R = crate::R<PwrdbgSpec>;
#[doc = "Register `PWRDBG` writer"]
pub type W = crate::W<PwrdbgSpec>;
#[doc = "Field `FORCE_WARM_RESET` reader - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
pub type ForceWarmResetR = crate::BitReader;
#[doc = "Field `FORCE_WARM_RESET` writer - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
pub type ForceWarmResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
    #[inline(always)]
    pub fn force_warm_reset(&self) -> ForceWarmResetR {
        ForceWarmResetR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
    #[inline(always)]
    pub fn force_warm_reset(&mut self) -> ForceWarmResetW<PwrdbgSpec> {
        ForceWarmResetW::new(self, 3)
    }
}
#[doc = "Power debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrdbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrdbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrdbgSpec;
impl crate::RegisterSpec for PwrdbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrdbg::R`](R) reader structure"]
impl crate::Readable for PwrdbgSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrdbg::W`](W) writer structure"]
impl crate::Writable for PwrdbgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRDBG to value 0"]
impl crate::Resettable for PwrdbgSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PWRDBG` reader"]
pub type R = crate::R<PWRDBG_SPEC>;
#[doc = "Register `PWRDBG` writer"]
pub type W = crate::W<PWRDBG_SPEC>;
#[doc = "Field `FORCE_WARM_RESET` reader - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
pub type FORCE_WARM_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_WARM_RESET` writer - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
pub type FORCE_WARM_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
    #[inline(always)]
    pub fn force_warm_reset(&self) -> FORCE_WARM_RESET_R {
        FORCE_WARM_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: No action 1: When written high, the chip is reset in the same manner as a CLD event and is readable from the RST field in the CLOCK_STA register."]
    #[inline(always)]
    #[must_use]
    pub fn force_warm_reset(&mut self) -> FORCE_WARM_RESET_W<PWRDBG_SPEC, 3> {
        FORCE_WARM_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Power debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdbg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdbg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRDBG_SPEC;
impl crate::RegisterSpec for PWRDBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrdbg::R`](R) reader structure"]
impl crate::Readable for PWRDBG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrdbg::W`](W) writer structure"]
impl crate::Writable for PWRDBG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRDBG to value 0"]
impl crate::Resettable for PWRDBG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

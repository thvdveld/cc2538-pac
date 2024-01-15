#[doc = "Register `CTRL_SW_RESET` reader"]
pub type R = crate::R<CTRL_SW_RESET_SPEC>;
#[doc = "Register `CTRL_SW_RESET` writer"]
pub type W = crate::W<CTRL_SW_RESET_SPEC>;
#[doc = "Field `SW_RESET` reader - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub type SW_RESET_R = crate::BitReader;
#[doc = "Field `SW_RESET` writer - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub type SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[inline(always)]
    pub fn sw_reset(&self) -> SW_RESET_R {
        SW_RESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[inline(always)]
    #[must_use]
    pub fn sw_reset(&mut self) -> SW_RESET_W<CTRL_SW_RESET_SPEC> {
        SW_RESET_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_sw_reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_sw_reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SW_RESET_SPEC;
impl crate::RegisterSpec for CTRL_SW_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_sw_reset::R`](R) reader structure"]
impl crate::Readable for CTRL_SW_RESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_sw_reset::W`](W) writer structure"]
impl crate::Writable for CTRL_SW_RESET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_SW_RESET to value 0"]
impl crate::Resettable for CTRL_SW_RESET_SPEC {
    const RESET_VALUE: u32 = 0;
}

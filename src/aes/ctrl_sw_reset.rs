#[doc = "Register `CTRL_SW_RESET` reader"]
pub type R = crate::R<CtrlSwResetSpec>;
#[doc = "Register `CTRL_SW_RESET` writer"]
pub type W = crate::W<CtrlSwResetSpec>;
#[doc = "Field `SW_RESET` reader - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub type SwResetR = crate::BitReader;
#[doc = "Field `SW_RESET` writer - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub type SwResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[inline(always)]
    pub fn sw_reset(&self) -> SwResetR {
        SwResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[inline(always)]
    pub fn sw_reset(&mut self) -> SwResetW<CtrlSwResetSpec> {
        SwResetW::new(self, 0)
    }
}
#[doc = "Software reset\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_sw_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_sw_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSwResetSpec;
impl crate::RegisterSpec for CtrlSwResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_sw_reset::R`](R) reader structure"]
impl crate::Readable for CtrlSwResetSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_sw_reset::W`](W) writer structure"]
impl crate::Writable for CtrlSwResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_SW_RESET to value 0"]
impl crate::Resettable for CtrlSwResetSpec {
    const RESET_VALUE: u32 = 0;
}

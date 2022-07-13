#[doc = "Register `CTRL_SW_RESET` reader"]
pub struct R(crate::R<CTRL_SW_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SW_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SW_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SW_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_SW_RESET` writer"]
pub struct W(crate::W<CTRL_SW_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SW_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SW_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SW_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_RESET` reader - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub type SW_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SW_RESET` writer - If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register, and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the written area flags; therefore, the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub type SW_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SW_RESET_SPEC, bool, O>;
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
    pub fn sw_reset(&mut self) -> SW_RESET_W<0> {
        SW_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_sw_reset](index.html) module"]
pub struct CTRL_SW_RESET_SPEC;
impl crate::RegisterSpec for CTRL_SW_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_sw_reset::R](R) reader structure"]
impl crate::Readable for CTRL_SW_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_sw_reset::W](W) writer structure"]
impl crate::Writable for CTRL_SW_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_SW_RESET to value 0"]
impl crate::Resettable for CTRL_SW_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

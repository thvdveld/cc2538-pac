#[doc = "Register `CTRL_INT_CFG` reader"]
pub struct R(crate::R<CTRL_INT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_INT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_INT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_INT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_INT_CFG` writer"]
pub struct W(crate::W<CTRL_INT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_INT_CFG_SPEC>;
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
impl From<crate::W<CTRL_INT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_INT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEVEL` reader - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
pub type LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `LEVEL` writer - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
pub type LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_INT_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W<0> {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_int_cfg](index.html) module"]
pub struct CTRL_INT_CFG_SPEC;
impl crate::RegisterSpec for CTRL_INT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_int_cfg::R](R) reader structure"]
impl crate::Readable for CTRL_INT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_int_cfg::W](W) writer structure"]
impl crate::Writable for CTRL_INT_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_INT_CFG to value 0"]
impl crate::Resettable for CTRL_INT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `CTRL_INT_CFG` reader"]
pub type R = crate::R<CTRL_INT_CFG_SPEC>;
#[doc = "Register `CTRL_INT_CFG` writer"]
pub type W = crate::W<CTRL_INT_CFG_SPEC>;
#[doc = "Field `LEVEL` reader - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
pub type LEVEL_R = crate::BitReader;
#[doc = "Field `LEVEL` writer - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
pub type LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<CTRL_INT_CFG_SPEC> {
        LEVEL_W::new(self, 0)
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
#[doc = "Interrupt configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_int_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_int_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_INT_CFG_SPEC;
impl crate::RegisterSpec for CTRL_INT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_int_cfg::R`](R) reader structure"]
impl crate::Readable for CTRL_INT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_int_cfg::W`](W) writer structure"]
impl crate::Writable for CTRL_INT_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_INT_CFG to value 0"]
impl crate::Resettable for CTRL_INT_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}

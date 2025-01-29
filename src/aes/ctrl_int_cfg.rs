#[doc = "Register `CTRL_INT_CFG` reader"]
pub type R = crate::R<CtrlIntCfgSpec>;
#[doc = "Register `CTRL_INT_CFG` writer"]
pub type W = crate::W<CtrlIntCfgSpec>;
#[doc = "Field `LEVEL` reader - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
pub type LevelR = crate::BitReader;
#[doc = "Field `LEVEL` writer - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
pub type LevelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<CtrlIntCfgSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "Interrupt configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_int_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_int_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlIntCfgSpec;
impl crate::RegisterSpec for CtrlIntCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_int_cfg::R`](R) reader structure"]
impl crate::Readable for CtrlIntCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_int_cfg::W`](W) writer structure"]
impl crate::Writable for CtrlIntCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_INT_CFG to value 0"]
impl crate::Resettable for CtrlIntCfgSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CSPCTRL` reader"]
pub type R = crate::R<CSPCTRL_SPEC>;
#[doc = "Register `CSPCTRL` writer"]
pub type W = crate::W<CSPCTRL_SPEC>;
#[doc = "Field `MCU_CTRL` reader - CSP MCU control input"]
pub type MCU_CTRL_R = crate::BitReader;
#[doc = "Field `MCU_CTRL` writer - CSP MCU control input"]
pub type MCU_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSP MCU control input"]
    #[inline(always)]
    pub fn mcu_ctrl(&self) -> MCU_CTRL_R {
        MCU_CTRL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSP MCU control input"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ctrl(&mut self) -> MCU_CTRL_W<CSPCTRL_SPEC> {
        MCU_CTRL_W::new(self, 0)
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
#[doc = "CSP control bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cspctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSPCTRL_SPEC;
impl crate::RegisterSpec for CSPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspctrl::R`](R) reader structure"]
impl crate::Readable for CSPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cspctrl::W`](W) writer structure"]
impl crate::Writable for CSPCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSPCTRL to value 0"]
impl crate::Resettable for CSPCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CSPCTRL` reader"]
pub type R = crate::R<CspctrlSpec>;
#[doc = "Register `CSPCTRL` writer"]
pub type W = crate::W<CspctrlSpec>;
#[doc = "Field `MCU_CTRL` reader - CSP MCU control input"]
pub type McuCtrlR = crate::BitReader;
#[doc = "Field `MCU_CTRL` writer - CSP MCU control input"]
pub type McuCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSP MCU control input"]
    #[inline(always)]
    pub fn mcu_ctrl(&self) -> McuCtrlR {
        McuCtrlR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSP MCU control input"]
    #[inline(always)]
    pub fn mcu_ctrl(&mut self) -> McuCtrlW<CspctrlSpec> {
        McuCtrlW::new(self, 0)
    }
}
#[doc = "CSP control bit\n\nYou can [`read`](crate::Reg::read) this register and get [`cspctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cspctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CspctrlSpec;
impl crate::RegisterSpec for CspctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspctrl::R`](R) reader structure"]
impl crate::Readable for CspctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cspctrl::W`](W) writer structure"]
impl crate::Writable for CspctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSPCTRL to value 0"]
impl crate::Resettable for CspctrlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `USBEN` reader - USB enable The USB controller is reset when this bit is cleared"]
pub type UsbenR = crate::BitReader;
#[doc = "Field `USBEN` writer - USB enable The USB controller is reset when this bit is cleared"]
pub type UsbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLEN` reader - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
pub type PllenR = crate::BitReader;
#[doc = "Field `PLLEN` writer - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
pub type PllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLOCKED` reader - PLL lock status. The PLL is locked when USB_CTRL.PLLLOCKED is 1."]
pub type PlllockedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB enable The USB controller is reset when this bit is cleared"]
    #[inline(always)]
    pub fn usben(&self) -> UsbenR {
        UsbenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
    #[inline(always)]
    pub fn pllen(&self) -> PllenR {
        PllenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL lock status. The PLL is locked when USB_CTRL.PLLLOCKED is 1."]
    #[inline(always)]
    pub fn plllocked(&self) -> PlllockedR {
        PlllockedR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB enable The USB controller is reset when this bit is cleared"]
    #[inline(always)]
    pub fn usben(&mut self) -> UsbenW<CtrlSpec> {
        UsbenW::new(self, 0)
    }
    #[doc = "Bit 1 - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
    #[inline(always)]
    pub fn pllen(&mut self) -> PllenW<CtrlSpec> {
        PllenW::new(self, 1)
    }
}
#[doc = "USB peripheral control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `USBEN` reader - USB enable The USB controller is reset when this bit is cleared"]
pub type USBEN_R = crate::BitReader;
#[doc = "Field `USBEN` writer - USB enable The USB controller is reset when this bit is cleared"]
pub type USBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLEN` reader - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
pub type PLLEN_R = crate::BitReader;
#[doc = "Field `PLLEN` writer - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
pub type PLLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLLOCKED` reader - PLL lock status. The PLL is locked when USB_CTRL.PLLLOCKED is 1."]
pub type PLLLOCKED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB enable The USB controller is reset when this bit is cleared"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL lock status. The PLL is locked when USB_CTRL.PLLLOCKED is 1."]
    #[inline(always)]
    pub fn plllocked(&self) -> PLLLOCKED_R {
        PLLLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB enable The USB controller is reset when this bit is cleared"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<CTRL_SPEC, 0> {
        USBEN_W::new(self)
    }
    #[doc = "Bit 1 - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<CTRL_SPEC, 1> {
        PLLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB peripheral control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

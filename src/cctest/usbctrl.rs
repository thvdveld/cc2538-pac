#[doc = "Register `USBCTRL` reader"]
pub type R = crate::R<USBCTRL_SPEC>;
#[doc = "Register `USBCTRL` writer"]
pub type W = crate::W<USBCTRL_SPEC>;
#[doc = "Field `USB_STB` reader - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
pub type USB_STB_R = crate::BitReader;
#[doc = "Field `USB_STB` writer - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
pub type USB_STB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
    #[inline(always)]
    pub fn usb_stb(&self) -> USB_STB_R {
        USB_STB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
    #[inline(always)]
    #[must_use]
    pub fn usb_stb(&mut self) -> USB_STB_W<USBCTRL_SPEC, 0> {
        USB_STB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB PHY stand-by control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCTRL_SPEC;
impl crate::RegisterSpec for USBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbctrl::R`](R) reader structure"]
impl crate::Readable for USBCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbctrl::W`](W) writer structure"]
impl crate::Writable for USBCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCTRL to value 0"]
impl crate::Resettable for USBCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

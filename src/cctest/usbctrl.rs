#[doc = "Register `USBCTRL` reader"]
pub type R = crate::R<UsbctrlSpec>;
#[doc = "Register `USBCTRL` writer"]
pub type W = crate::W<UsbctrlSpec>;
#[doc = "Field `USB_STB` reader - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
pub type UsbStbR = crate::BitReader;
#[doc = "Field `USB_STB` writer - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
pub type UsbStbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
    #[inline(always)]
    pub fn usb_stb(&self) -> UsbStbR {
        UsbStbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
    #[inline(always)]
    #[must_use]
    pub fn usb_stb(&mut self) -> UsbStbW<UsbctrlSpec> {
        UsbStbW::new(self, 0)
    }
}
#[doc = "USB PHY stand-by control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbctrlSpec;
impl crate::RegisterSpec for UsbctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbctrl::R`](R) reader structure"]
impl crate::Readable for UsbctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbctrl::W`](W) writer structure"]
impl crate::Writable for UsbctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBCTRL to value 0"]
impl crate::Resettable for UsbctrlSpec {
    const RESET_VALUE: u32 = 0;
}

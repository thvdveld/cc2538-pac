#[doc = "Register `USB_CTRL` reader"]
pub type R = crate::R<UsbCtrlSpec>;
#[doc = "Register `USB_CTRL` writer"]
pub type W = crate::W<UsbCtrlSpec>;
#[doc = "Field `USB_EDGE_CTL` reader - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
pub type UsbEdgeCtlR = crate::BitReader;
#[doc = "Field `USB_EDGE_CTL` writer - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
pub type UsbEdgeCtlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
    #[inline(always)]
    pub fn usb_edge_ctl(&self) -> UsbEdgeCtlR {
        UsbEdgeCtlR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
    #[inline(always)]
    #[must_use]
    pub fn usb_edge_ctl(&mut self) -> UsbEdgeCtlW<UsbCtrlSpec> {
        UsbEdgeCtlW::new(self, 0)
    }
}
#[doc = "This register is used to control which edge of the USB controller input generates a power-up interrupt to the system.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbCtrlSpec;
impl crate::RegisterSpec for UsbCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_ctrl::R`](R) reader structure"]
impl crate::Readable for UsbCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_ctrl::W`](W) writer structure"]
impl crate::Writable for UsbCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_CTRL to value 0"]
impl crate::Resettable for UsbCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

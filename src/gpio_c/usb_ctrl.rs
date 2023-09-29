#[doc = "Register `USB_CTRL` reader"]
pub type R = crate::R<USB_CTRL_SPEC>;
#[doc = "Register `USB_CTRL` writer"]
pub type W = crate::W<USB_CTRL_SPEC>;
#[doc = "Field `USB_EDGE_CTL` reader - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
pub type USB_EDGE_CTL_R = crate::BitReader;
#[doc = "Field `USB_EDGE_CTL` writer - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
pub type USB_EDGE_CTL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
    #[inline(always)]
    pub fn usb_edge_ctl(&self) -> USB_EDGE_CTL_R {
        USB_EDGE_CTL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
    #[inline(always)]
    #[must_use]
    pub fn usb_edge_ctl(&mut self) -> USB_EDGE_CTL_W<USB_CTRL_SPEC, 0> {
        USB_EDGE_CTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register is used to control which edge of the USB controller input generates a power-up interrupt to the system.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_CTRL_SPEC;
impl crate::RegisterSpec for USB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_ctrl::R`](R) reader structure"]
impl crate::Readable for USB_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_ctrl::W`](W) writer structure"]
impl crate::Writable for USB_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_CTRL to value 0"]
impl crate::Resettable for USB_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

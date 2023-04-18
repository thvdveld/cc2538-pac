#[doc = "Register `USB_CTRL` reader"]
pub struct R(crate::R<USB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CTRL` writer"]
pub struct W(crate::W<USB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_EDGE_CTL` reader - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
pub type USB_EDGE_CTL_R = crate::BitReader<bool>;
#[doc = "Field `USB_EDGE_CTL` writer - Used to set the edge which triggers the USB power up interrupt 0: Rising 1: Falling"]
pub type USB_EDGE_CTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CTRL_SPEC, bool, O>;
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
    pub fn usb_edge_ctl(&mut self) -> USB_EDGE_CTL_W<0> {
        USB_EDGE_CTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control which edge of the USB controller input generates a power-up interrupt to the system.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ctrl](index.html) module"]
pub struct USB_CTRL_SPEC;
impl crate::RegisterSpec for USB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_ctrl::R](R) reader structure"]
impl crate::Readable for USB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_ctrl::W](W) writer structure"]
impl crate::Writable for USB_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_CTRL to value 0"]
impl crate::Resettable for USB_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

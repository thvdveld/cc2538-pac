#[doc = "Register `USB_IRQ_ACK` reader"]
pub struct R(crate::R<USB_IRQ_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_IRQ_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_IRQ_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_IRQ_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_IRQ_ACK` writer"]
pub struct W(crate::W<USB_IRQ_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_IRQ_ACK_SPEC>;
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
impl From<crate::W<USB_IRQ_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_IRQ_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBACK` reader - USB masked interrupt status: 1: Detected 0: Not detected"]
pub type USBACK_R = crate::BitReader<bool>;
#[doc = "Field `USBACK` writer - USB masked interrupt status: 1: Detected 0: Not detected"]
pub type USBACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_IRQ_ACK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn usback(&self) -> USBACK_R {
        USBACK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn usback(&mut self) -> USBACK_W<0> {
        USBACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Same functionality as IRQ_DETECT_ACK, but for USB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_irq_ack](index.html) module"]
pub struct USB_IRQ_ACK_SPEC;
impl crate::RegisterSpec for USB_IRQ_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_irq_ack::R](R) reader structure"]
impl crate::Readable for USB_IRQ_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_irq_ack::W](W) writer structure"]
impl crate::Writable for USB_IRQ_ACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_IRQ_ACK to value 0"]
impl crate::Resettable for USB_IRQ_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

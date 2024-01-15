#[doc = "Register `USB_IRQ_ACK` reader"]
pub type R = crate::R<USB_IRQ_ACK_SPEC>;
#[doc = "Register `USB_IRQ_ACK` writer"]
pub type W = crate::W<USB_IRQ_ACK_SPEC>;
#[doc = "Field `USBACK` reader - USB masked interrupt status: 1: Detected 0: Not detected"]
pub type USBACK_R = crate::BitReader;
#[doc = "Field `USBACK` writer - USB masked interrupt status: 1: Detected 0: Not detected"]
pub type USBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[must_use]
    pub fn usback(&mut self) -> USBACK_W<USB_IRQ_ACK_SPEC> {
        USBACK_W::new(self, 0)
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
#[doc = "Same functionality as IRQ_DETECT_ACK, but for USB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_irq_ack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_irq_ack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_IRQ_ACK_SPEC;
impl crate::RegisterSpec for USB_IRQ_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_irq_ack::R`](R) reader structure"]
impl crate::Readable for USB_IRQ_ACK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_irq_ack::W`](W) writer structure"]
impl crate::Writable for USB_IRQ_ACK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_IRQ_ACK to value 0"]
impl crate::Resettable for USB_IRQ_ACK_SPEC {
    const RESET_VALUE: u32 = 0;
}

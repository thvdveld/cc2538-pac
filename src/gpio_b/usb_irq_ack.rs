#[doc = "Register `USB_IRQ_ACK` reader"]
pub type R = crate::R<UsbIrqAckSpec>;
#[doc = "Register `USB_IRQ_ACK` writer"]
pub type W = crate::W<UsbIrqAckSpec>;
#[doc = "Field `USBACK` reader - USB masked interrupt status: 1: Detected 0: Not detected"]
pub type UsbackR = crate::BitReader;
#[doc = "Field `USBACK` writer - USB masked interrupt status: 1: Detected 0: Not detected"]
pub type UsbackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn usback(&self) -> UsbackR {
        UsbackR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn usback(&mut self) -> UsbackW<UsbIrqAckSpec> {
        UsbackW::new(self, 0)
    }
}
#[doc = "Same functionality as IRQ_DETECT_ACK, but for USB\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_irq_ack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_irq_ack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbIrqAckSpec;
impl crate::RegisterSpec for UsbIrqAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_irq_ack::R`](R) reader structure"]
impl crate::Readable for UsbIrqAckSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_irq_ack::W`](W) writer structure"]
impl crate::Writable for UsbIrqAckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_IRQ_ACK to value 0"]
impl crate::Resettable for UsbIrqAckSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `F0` reader"]
pub type R = crate::R<F0Spec>;
#[doc = "Register `F0` writer"]
pub type W = crate::W<F0Spec>;
#[doc = "Field `USBF0` reader - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
pub type Usbf0R = crate::FieldReader;
#[doc = "Field `USBF0` writer - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
pub type Usbf0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
    #[inline(always)]
    pub fn usbf0(&self) -> Usbf0R {
        Usbf0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
    #[inline(always)]
    pub fn usbf0(&mut self) -> Usbf0W<F0Spec> {
        Usbf0W::new(self, 0)
    }
}
#[doc = "Endpoint 0 FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F0Spec;
impl crate::RegisterSpec for F0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f0::R`](R) reader structure"]
impl crate::Readable for F0Spec {}
#[doc = "`write(|w| ..)` method takes [`f0::W`](W) writer structure"]
impl crate::Writable for F0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets F0 to value 0"]
impl crate::Resettable for F0Spec {
    const RESET_VALUE: u32 = 0;
}

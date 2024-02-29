#[doc = "Register `F5` reader"]
pub type R = crate::R<F5Spec>;
#[doc = "Register `F5` writer"]
pub type W = crate::W<F5Spec>;
#[doc = "Field `USBF5` reader - Endpoint 5 FIFO register Reading this register unloads one byte from the EP5 OUT FIFO. Writing to this register loads one byte into the EP5 IN FIFO."]
pub type Usbf5R = crate::FieldReader;
#[doc = "Field `USBF5` writer - Endpoint 5 FIFO register Reading this register unloads one byte from the EP5 OUT FIFO. Writing to this register loads one byte into the EP5 IN FIFO."]
pub type Usbf5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 5 FIFO register Reading this register unloads one byte from the EP5 OUT FIFO. Writing to this register loads one byte into the EP5 IN FIFO."]
    #[inline(always)]
    pub fn usbf5(&self) -> Usbf5R {
        Usbf5R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 5 FIFO register Reading this register unloads one byte from the EP5 OUT FIFO. Writing to this register loads one byte into the EP5 IN FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn usbf5(&mut self) -> Usbf5W<F5Spec> {
        Usbf5W::new(self, 0)
    }
}
#[doc = "IN/OUT endpoint 5 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F5Spec;
impl crate::RegisterSpec for F5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f5::R`](R) reader structure"]
impl crate::Readable for F5Spec {}
#[doc = "`write(|w| ..)` method takes [`f5::W`](W) writer structure"]
impl crate::Writable for F5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets F5 to value 0"]
impl crate::Resettable for F5Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `F2` reader"]
pub type R = crate::R<F2Spec>;
#[doc = "Register `F2` writer"]
pub type W = crate::W<F2Spec>;
#[doc = "Field `USBF2` reader - Endpoint 2 FIFO register Reading this register unloads one byte from the EP2 OUT FIFO. Writing to this register loads one byte into the EP2 IN FIFO."]
pub type Usbf2R = crate::FieldReader;
#[doc = "Field `USBF2` writer - Endpoint 2 FIFO register Reading this register unloads one byte from the EP2 OUT FIFO. Writing to this register loads one byte into the EP2 IN FIFO."]
pub type Usbf2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 2 FIFO register Reading this register unloads one byte from the EP2 OUT FIFO. Writing to this register loads one byte into the EP2 IN FIFO."]
    #[inline(always)]
    pub fn usbf2(&self) -> Usbf2R {
        Usbf2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 2 FIFO register Reading this register unloads one byte from the EP2 OUT FIFO. Writing to this register loads one byte into the EP2 IN FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn usbf2(&mut self) -> Usbf2W<F2Spec> {
        Usbf2W::new(self, 0)
    }
}
#[doc = "IN/OUT endpoint 2 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F2Spec;
impl crate::RegisterSpec for F2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f2::R`](R) reader structure"]
impl crate::Readable for F2Spec {}
#[doc = "`write(|w| ..)` method takes [`f2::W`](W) writer structure"]
impl crate::Writable for F2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets F2 to value 0"]
impl crate::Resettable for F2Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `F1` reader"]
pub type R = crate::R<F1Spec>;
#[doc = "Register `F1` writer"]
pub type W = crate::W<F1Spec>;
#[doc = "Field `USBF1` reader - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
pub type Usbf1R = crate::FieldReader;
#[doc = "Field `USBF1` writer - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
pub type Usbf1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
    #[inline(always)]
    pub fn usbf1(&self) -> Usbf1R {
        Usbf1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
    #[inline(always)]
    pub fn usbf1(&mut self) -> Usbf1W<F1Spec> {
        Usbf1W::new(self, 0)
    }
}
#[doc = "IN/OUT endpoint 1 FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`f1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F1Spec;
impl crate::RegisterSpec for F1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f1::R`](R) reader structure"]
impl crate::Readable for F1Spec {}
#[doc = "`write(|w| ..)` method takes [`f1::W`](W) writer structure"]
impl crate::Writable for F1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets F1 to value 0"]
impl crate::Resettable for F1Spec {
    const RESET_VALUE: u32 = 0;
}

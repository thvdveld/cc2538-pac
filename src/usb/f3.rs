#[doc = "Register `F3` reader"]
pub type R = crate::R<F3Spec>;
#[doc = "Register `F3` writer"]
pub type W = crate::W<F3Spec>;
#[doc = "Field `USBF3` reader - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
pub type Usbf3R = crate::FieldReader;
#[doc = "Field `USBF3` writer - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
pub type Usbf3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
    #[inline(always)]
    pub fn usbf3(&self) -> Usbf3R {
        Usbf3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
    #[inline(always)]
    pub fn usbf3(&mut self) -> Usbf3W<F3Spec> {
        Usbf3W::new(self, 0)
    }
}
#[doc = "IN/OUT endpoint 3 FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`f3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F3Spec;
impl crate::RegisterSpec for F3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f3::R`](R) reader structure"]
impl crate::Readable for F3Spec {}
#[doc = "`write(|w| ..)` method takes [`f3::W`](W) writer structure"]
impl crate::Writable for F3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets F3 to value 0"]
impl crate::Resettable for F3Spec {
    const RESET_VALUE: u32 = 0;
}

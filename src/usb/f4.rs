#[doc = "Register `F4` reader"]
pub type R = crate::R<F4Spec>;
#[doc = "Register `F4` writer"]
pub type W = crate::W<F4Spec>;
#[doc = "Field `USBF4` reader - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
pub type Usbf4R = crate::FieldReader;
#[doc = "Field `USBF4` writer - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
pub type Usbf4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
    #[inline(always)]
    pub fn usbf4(&self) -> Usbf4R {
        Usbf4R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
    #[inline(always)]
    pub fn usbf4(&mut self) -> Usbf4W<F4Spec> {
        Usbf4W::new(self, 0)
    }
}
#[doc = "IN/OUT endpoint 4 FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F4Spec;
impl crate::RegisterSpec for F4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f4::R`](R) reader structure"]
impl crate::Readable for F4Spec {}
#[doc = "`write(|w| ..)` method takes [`f4::W`](W) writer structure"]
impl crate::Writable for F4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets F4 to value 0"]
impl crate::Resettable for F4Spec {
    const RESET_VALUE: u32 = 0;
}

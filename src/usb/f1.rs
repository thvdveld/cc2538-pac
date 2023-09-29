#[doc = "Register `F1` reader"]
pub type R = crate::R<F1_SPEC>;
#[doc = "Register `F1` writer"]
pub type W = crate::W<F1_SPEC>;
#[doc = "Field `USBF1` reader - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
pub type USBF1_R = crate::FieldReader;
#[doc = "Field `USBF1` writer - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
pub type USBF1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
    #[inline(always)]
    pub fn usbf1(&self) -> USBF1_R {
        USBF1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn usbf1(&mut self) -> USBF1_W<F1_SPEC, 0> {
        USBF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IN/OUT endpoint 1 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F1_SPEC;
impl crate::RegisterSpec for F1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f1::R`](R) reader structure"]
impl crate::Readable for F1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`f1::W`](W) writer structure"]
impl crate::Writable for F1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets F1 to value 0"]
impl crate::Resettable for F1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

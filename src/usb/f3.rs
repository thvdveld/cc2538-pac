#[doc = "Register `F3` reader"]
pub type R = crate::R<F3_SPEC>;
#[doc = "Register `F3` writer"]
pub type W = crate::W<F3_SPEC>;
#[doc = "Field `USBF3` reader - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
pub type USBF3_R = crate::FieldReader;
#[doc = "Field `USBF3` writer - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
pub type USBF3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
    #[inline(always)]
    pub fn usbf3(&self) -> USBF3_R {
        USBF3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn usbf3(&mut self) -> USBF3_W<F3_SPEC, 0> {
        USBF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IN/OUT endpoint 3 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F3_SPEC;
impl crate::RegisterSpec for F3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f3::R`](R) reader structure"]
impl crate::Readable for F3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`f3::W`](W) writer structure"]
impl crate::Writable for F3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets F3 to value 0"]
impl crate::Resettable for F3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

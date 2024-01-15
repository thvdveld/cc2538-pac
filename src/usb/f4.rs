#[doc = "Register `F4` reader"]
pub type R = crate::R<F4_SPEC>;
#[doc = "Register `F4` writer"]
pub type W = crate::W<F4_SPEC>;
#[doc = "Field `USBF4` reader - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
pub type USBF4_R = crate::FieldReader;
#[doc = "Field `USBF4` writer - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
pub type USBF4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
    #[inline(always)]
    pub fn usbf4(&self) -> USBF4_R {
        USBF4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn usbf4(&mut self) -> USBF4_W<F4_SPEC> {
        USBF4_W::new(self, 0)
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
#[doc = "IN/OUT endpoint 4 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F4_SPEC;
impl crate::RegisterSpec for F4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f4::R`](R) reader structure"]
impl crate::Readable for F4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`f4::W`](W) writer structure"]
impl crate::Writable for F4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets F4 to value 0"]
impl crate::Resettable for F4_SPEC {
    const RESET_VALUE: u32 = 0;
}

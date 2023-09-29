#[doc = "Register `IE` reader"]
pub type R = crate::R<IE_SPEC>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IE_SPEC>;
#[doc = "Field `IE` reader - Bits set: Corresponding pin is not masked Bits cleared: Corresponding pin is masked"]
pub type IE_R = crate::FieldReader;
#[doc = "Field `IE` writer - Bits set: Corresponding pin is not masked Bits cleared: Corresponding pin is masked"]
pub type IE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Corresponding pin is not masked Bits cleared: Corresponding pin is masked"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Corresponding pin is not masked Bits cleared: Corresponding pin is masked"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<IE_SPEC, 0> {
        IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The IE register is the interrupt mask register. Bits set to high in IE allow the corresponding pins to trigger their individual interrupts and the combined GPIOINTR line. Clearing a bit disables interrupt triggering on that pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

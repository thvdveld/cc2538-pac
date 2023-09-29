#[doc = "Register `PB2_OVER` reader"]
pub type R = crate::R<PB2_OVER_SPEC>;
#[doc = "Register `PB2_OVER` writer"]
pub type W = crate::W<PB2_OVER_SPEC>;
#[doc = "Field `PB2_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PB2_OVER_R = crate::FieldReader;
#[doc = "Field `PB2_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PB2_OVER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb2_over(&self) -> PB2_OVER_R {
        PB2_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb2_over(&mut self) -> PB2_OVER_W<PB2_OVER_SPEC, 0> {
        PB2_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb2_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb2_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB2_OVER_SPEC;
impl crate::RegisterSpec for PB2_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb2_over::R`](R) reader structure"]
impl crate::Readable for PB2_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb2_over::W`](W) writer structure"]
impl crate::Writable for PB2_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PB2_OVER to value 0x04"]
impl crate::Resettable for PB2_OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}

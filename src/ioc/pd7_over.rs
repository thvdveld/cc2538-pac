#[doc = "Register `PD7_OVER` reader"]
pub type R = crate::R<PD7_OVER_SPEC>;
#[doc = "Register `PD7_OVER` writer"]
pub type W = crate::W<PD7_OVER_SPEC>;
#[doc = "Field `PD7_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PD7_OVER_R = crate::FieldReader;
#[doc = "Field `PD7_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PD7_OVER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd7_over(&self) -> PD7_OVER_R {
        PD7_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd7_over(&mut self) -> PD7_OVER_W<PD7_OVER_SPEC, 0> {
        PD7_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd7_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd7_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD7_OVER_SPEC;
impl crate::RegisterSpec for PD7_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd7_over::R`](R) reader structure"]
impl crate::Readable for PD7_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd7_over::W`](W) writer structure"]
impl crate::Writable for PD7_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD7_OVER to value 0x04"]
impl crate::Resettable for PD7_OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}

#[doc = "Register `PA3_OVER` reader"]
pub type R = crate::R<PA3_OVER_SPEC>;
#[doc = "Register `PA3_OVER` writer"]
pub type W = crate::W<PA3_OVER_SPEC>;
#[doc = "Field `PA3_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PA3_OVER_R = crate::FieldReader;
#[doc = "Field `PA3_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PA3_OVER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa3_over(&self) -> PA3_OVER_R {
        PA3_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pa3_over(&mut self) -> PA3_OVER_W<PA3_OVER_SPEC, 0> {
        PA3_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa3_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa3_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA3_OVER_SPEC;
impl crate::RegisterSpec for PA3_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa3_over::R`](R) reader structure"]
impl crate::Readable for PA3_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pa3_over::W`](W) writer structure"]
impl crate::Writable for PA3_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PA3_OVER to value 0x04"]
impl crate::Resettable for PA3_OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}

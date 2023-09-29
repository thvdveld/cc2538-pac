#[doc = "Register `PC7_OVER` reader"]
pub type R = crate::R<PC7_OVER_SPEC>;
#[doc = "Register `PC7_OVER` writer"]
pub type W = crate::W<PC7_OVER_SPEC>;
#[doc = "Field `PC7_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PC7_OVER_R = crate::FieldReader;
#[doc = "Field `PC7_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PC7_OVER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pc7_over(&self) -> PC7_OVER_R {
        PC7_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pc7_over(&mut self) -> PC7_OVER_W<PC7_OVER_SPEC, 0> {
        PC7_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc7_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc7_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC7_OVER_SPEC;
impl crate::RegisterSpec for PC7_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc7_over::R`](R) reader structure"]
impl crate::Readable for PC7_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc7_over::W`](W) writer structure"]
impl crate::Writable for PC7_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PC7_OVER to value 0x04"]
impl crate::Resettable for PC7_OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}

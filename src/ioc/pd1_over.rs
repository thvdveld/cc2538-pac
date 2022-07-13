#[doc = "Register `PD1_OVER` reader"]
pub struct R(crate::R<PD1_OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD1_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD1_OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD1_OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD1_OVER` writer"]
pub struct W(crate::W<PD1_OVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD1_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PD1_OVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD1_OVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD1_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PD1_OVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD1_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PD1_OVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PD1_OVER_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd1_over(&self) -> PD1_OVER_R {
        PD1_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd1_over(&mut self) -> PD1_OVER_W<0> {
        PD1_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd1_over](index.html) module"]
pub struct PD1_OVER_SPEC;
impl crate::RegisterSpec for PD1_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd1_over::R](R) reader structure"]
impl crate::Readable for PD1_OVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd1_over::W](W) writer structure"]
impl crate::Writable for PD1_OVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD1_OVER to value 0x04"]
impl crate::Resettable for PD1_OVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}

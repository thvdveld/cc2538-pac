#[doc = "Register `PA7_OVER` reader"]
pub struct R(crate::R<PA7_OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA7_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA7_OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA7_OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PA7_OVER` writer"]
pub struct W(crate::W<PA7_OVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA7_OVER_SPEC>;
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
impl From<crate::W<PA7_OVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA7_OVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA7_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PA7_OVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA7_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PA7_OVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA7_OVER_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa7_over(&self) -> PA7_OVER_R {
        PA7_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa7_over(&mut self) -> PA7_OVER_W<0> {
        PA7_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa7_over](index.html) module"]
pub struct PA7_OVER_SPEC;
impl crate::RegisterSpec for PA7_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa7_over::R](R) reader structure"]
impl crate::Readable for PA7_OVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa7_over::W](W) writer structure"]
impl crate::Writable for PA7_OVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PA7_OVER to value 0x04"]
impl crate::Resettable for PA7_OVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}

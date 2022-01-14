#[doc = "Register `PB1_OVER` reader"]
pub struct R(crate::R<PB1_OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB1_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB1_OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB1_OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB1_OVER` writer"]
pub struct W(crate::W<PB1_OVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB1_OVER_SPEC>;
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
impl From<crate::W<PB1_OVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB1_OVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB1_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub struct PB1_OVER_R(crate::FieldReader<u8, u8>);
impl PB1_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB1_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB1_OVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB1_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub struct PB1_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PB1_OVER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb1_over(&self) -> PB1_OVER_R {
        PB1_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb1_over(&mut self) -> PB1_OVER_W {
        PB1_OVER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb1_over](index.html) module"]
pub struct PB1_OVER_SPEC;
impl crate::RegisterSpec for PB1_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb1_over::R](R) reader structure"]
impl crate::Readable for PB1_OVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb1_over::W](W) writer structure"]
impl crate::Writable for PB1_OVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB1_OVER to value 0x04"]
impl crate::Resettable for PB1_OVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}

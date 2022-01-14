#[doc = "Register `PC5_OVER` reader"]
pub struct R(crate::R<PC5_OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC5_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC5_OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC5_OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC5_OVER` writer"]
pub struct W(crate::W<PC5_OVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC5_OVER_SPEC>;
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
impl From<crate::W<PC5_OVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC5_OVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC5_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub struct PC5_OVER_R(crate::FieldReader<u8, u8>);
impl PC5_OVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC5_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC5_OVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC5_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub struct PC5_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PC5_OVER_W<'a> {
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
    pub fn pc5_over(&self) -> PC5_OVER_R {
        PC5_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pc5_over(&mut self) -> PC5_OVER_W {
        PC5_OVER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc5_over](index.html) module"]
pub struct PC5_OVER_SPEC;
impl crate::RegisterSpec for PC5_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc5_over::R](R) reader structure"]
impl crate::Readable for PC5_OVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc5_over::W](W) writer structure"]
impl crate::Writable for PC5_OVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC5_OVER to value 0x04"]
impl crate::Resettable for PC5_OVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}

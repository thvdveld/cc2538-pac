#[doc = "Register `PD7_SEL` reader"]
pub struct R(crate::R<PD7_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD7_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD7_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD7_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD7_SEL` writer"]
pub struct W(crate::W<PD7_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD7_SEL_SPEC>;
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
impl From<crate::W<PD7_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD7_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD7_sel` reader - Select one peripheral signal output for PD7."]
pub struct PD7_SEL_R(crate::FieldReader<u8, u8>);
impl PD7_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD7_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD7_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD7_sel` writer - Select one peripheral signal output for PD7."]
pub struct PD7_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD7_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD7."]
    #[inline(always)]
    pub fn pd7_sel(&self) -> PD7_SEL_R {
        PD7_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD7."]
    #[inline(always)]
    pub fn pd7_sel(&mut self) -> PD7_SEL_W {
        PD7_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral select control for PD7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd7_sel](index.html) module"]
pub struct PD7_SEL_SPEC;
impl crate::RegisterSpec for PD7_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd7_sel::R](R) reader structure"]
impl crate::Readable for PD7_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd7_sel::W](W) writer structure"]
impl crate::Writable for PD7_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD7_SEL to value 0"]
impl crate::Resettable for PD7_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

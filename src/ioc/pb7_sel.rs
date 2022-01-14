#[doc = "Register `PB7_SEL` reader"]
pub struct R(crate::R<PB7_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB7_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB7_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB7_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB7_SEL` writer"]
pub struct W(crate::W<PB7_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB7_SEL_SPEC>;
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
impl From<crate::W<PB7_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB7_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB7_sel` reader - Select one peripheral signal output for PB7."]
pub struct PB7_SEL_R(crate::FieldReader<u8, u8>);
impl PB7_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PB7_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB7_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB7_sel` writer - Select one peripheral signal output for PB7."]
pub struct PB7_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PB7_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB7."]
    #[inline(always)]
    pub fn pb7_sel(&self) -> PB7_SEL_R {
        PB7_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB7."]
    #[inline(always)]
    pub fn pb7_sel(&mut self) -> PB7_SEL_W {
        PB7_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral select control for PB7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb7_sel](index.html) module"]
pub struct PB7_SEL_SPEC;
impl crate::RegisterSpec for PB7_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb7_sel::R](R) reader structure"]
impl crate::Readable for PB7_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb7_sel::W](W) writer structure"]
impl crate::Writable for PB7_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB7_SEL to value 0"]
impl crate::Resettable for PB7_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

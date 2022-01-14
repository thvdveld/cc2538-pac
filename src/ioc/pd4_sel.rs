#[doc = "Register `PD4_SEL` reader"]
pub struct R(crate::R<PD4_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD4_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD4_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD4_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD4_SEL` writer"]
pub struct W(crate::W<PD4_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD4_SEL_SPEC>;
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
impl From<crate::W<PD4_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD4_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD4_sel` reader - Select one peripheral signal output for PD4."]
pub struct PD4_SEL_R(crate::FieldReader<u8, u8>);
impl PD4_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD4_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD4_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD4_sel` writer - Select one peripheral signal output for PD4."]
pub struct PD4_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD4."]
    #[inline(always)]
    pub fn pd4_sel(&self) -> PD4_SEL_R {
        PD4_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD4."]
    #[inline(always)]
    pub fn pd4_sel(&mut self) -> PD4_SEL_W {
        PD4_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral select control for PD4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd4_sel](index.html) module"]
pub struct PD4_SEL_SPEC;
impl crate::RegisterSpec for PD4_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd4_sel::R](R) reader structure"]
impl crate::Readable for PD4_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd4_sel::W](W) writer structure"]
impl crate::Writable for PD4_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD4_SEL to value 0"]
impl crate::Resettable for PD4_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

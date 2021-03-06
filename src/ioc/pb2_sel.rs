#[doc = "Register `PB2_SEL` reader"]
pub struct R(crate::R<PB2_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB2_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB2_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB2_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB2_SEL` writer"]
pub struct W(crate::W<PB2_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB2_SEL_SPEC>;
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
impl From<crate::W<PB2_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB2_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB2_sel` reader - Select one peripheral signal output for PB2."]
pub type PB2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PB2_sel` writer - Select one peripheral signal output for PB2."]
pub type PB2_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PB2_SEL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB2."]
    #[inline(always)]
    pub fn pb2_sel(&self) -> PB2_SEL_R {
        PB2_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB2."]
    #[inline(always)]
    pub fn pb2_sel(&mut self) -> PB2_SEL_W<0> {
        PB2_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral select control for PB2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb2_sel](index.html) module"]
pub struct PB2_SEL_SPEC;
impl crate::RegisterSpec for PB2_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb2_sel::R](R) reader structure"]
impl crate::Readable for PB2_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb2_sel::W](W) writer structure"]
impl crate::Writable for PB2_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB2_SEL to value 0"]
impl crate::Resettable for PB2_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

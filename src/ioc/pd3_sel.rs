#[doc = "Register `PD3_SEL` reader"]
pub struct R(crate::R<PD3_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD3_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD3_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD3_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD3_SEL` writer"]
pub struct W(crate::W<PD3_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD3_SEL_SPEC>;
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
impl From<crate::W<PD3_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD3_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD3_sel` reader - Select one peripheral signal output for PD3."]
pub type PD3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD3_sel` writer - Select one peripheral signal output for PD3."]
pub type PD3_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PD3_SEL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD3."]
    #[inline(always)]
    pub fn pd3_sel(&self) -> PD3_SEL_R {
        PD3_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD3."]
    #[inline(always)]
    pub fn pd3_sel(&mut self) -> PD3_SEL_W<0> {
        PD3_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral select control for PD3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd3_sel](index.html) module"]
pub struct PD3_SEL_SPEC;
impl crate::RegisterSpec for PD3_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd3_sel::R](R) reader structure"]
impl crate::Readable for PD3_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd3_sel::W](W) writer structure"]
impl crate::Writable for PD3_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD3_SEL to value 0"]
impl crate::Resettable for PD3_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

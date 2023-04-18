#[doc = "Register `PD2_SEL` reader"]
pub struct R(crate::R<PD2_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD2_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD2_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD2_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD2_SEL` writer"]
pub struct W(crate::W<PD2_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD2_SEL_SPEC>;
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
impl From<crate::W<PD2_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD2_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD2_sel` reader - Select one peripheral signal output for PD2."]
pub type PD2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD2_sel` writer - Select one peripheral signal output for PD2."]
pub type PD2_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PD2_SEL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD2."]
    #[inline(always)]
    pub fn pd2_sel(&self) -> PD2_SEL_R {
        PD2_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD2."]
    #[inline(always)]
    #[must_use]
    pub fn pd2_sel(&mut self) -> PD2_SEL_W<0> {
        PD2_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral select control for PD2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd2_sel](index.html) module"]
pub struct PD2_SEL_SPEC;
impl crate::RegisterSpec for PD2_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd2_sel::R](R) reader structure"]
impl crate::Readable for PD2_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd2_sel::W](W) writer structure"]
impl crate::Writable for PD2_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD2_SEL to value 0"]
impl crate::Resettable for PD2_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

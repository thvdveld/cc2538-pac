#[doc = "Register `PA4_SEL` reader"]
pub struct R(crate::R<PA4_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA4_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA4_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA4_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PA4_SEL` writer"]
pub struct W(crate::W<PA4_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA4_SEL_SPEC>;
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
impl From<crate::W<PA4_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA4_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA4_sel` reader - Select one peripheral signal output for PA4."]
pub type PA4_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA4_sel` writer - Select one peripheral signal output for PA4."]
pub type PA4_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA4_SEL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA4."]
    #[inline(always)]
    pub fn pa4_sel(&self) -> PA4_SEL_R {
        PA4_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA4."]
    #[inline(always)]
    #[must_use]
    pub fn pa4_sel(&mut self) -> PA4_SEL_W<0> {
        PA4_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral select control for PA4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa4_sel](index.html) module"]
pub struct PA4_SEL_SPEC;
impl crate::RegisterSpec for PA4_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa4_sel::R](R) reader structure"]
impl crate::Readable for PA4_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa4_sel::W](W) writer structure"]
impl crate::Writable for PA4_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PA4_SEL to value 0"]
impl crate::Resettable for PA4_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

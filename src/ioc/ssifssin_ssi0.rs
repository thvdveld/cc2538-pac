#[doc = "Register `SSIFSSIN_SSI0` reader"]
pub struct R(crate::R<SSIFSSIN_SSI0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSIFSSIN_SSI0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSIFSSIN_SSI0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSIFSSIN_SSI0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSIFSSIN_SSI0` writer"]
pub struct W(crate::W<SSIFSSIN_SSI0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSIFSSIN_SSI0_SPEC>;
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
impl From<crate::W<SSIFSSIN_SSI0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSIFSSIN_SSI0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
pub type INPUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
pub type INPUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SSIFSSIN_SSI0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
    #[inline(always)]
    pub fn input_sel(&self) -> INPUT_SEL_R {
        INPUT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
    #[inline(always)]
    pub fn input_sel(&mut self) -> INPUT_SEL_W<0> {
        INPUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssifssin_ssi0](index.html) module"]
pub struct SSIFSSIN_SSI0_SPEC;
impl crate::RegisterSpec for SSIFSSIN_SSI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssifssin_ssi0::R](R) reader structure"]
impl crate::Readable for SSIFSSIN_SSI0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssifssin_ssi0::W](W) writer structure"]
impl crate::Writable for SSIFSSIN_SSI0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSIFSSIN_SSI0 to value 0"]
impl crate::Resettable for SSIFSSIN_SSI0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `GPT2OCP1` reader"]
pub struct R(crate::R<GPT2OCP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPT2OCP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPT2OCP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPT2OCP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPT2OCP1` writer"]
pub struct W(crate::W<GPT2OCP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPT2OCP1_SPEC>;
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
impl From<crate::W<GPT2OCP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPT2OCP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as GPT2OCP1 1: PA1 selected as GPT2OCP1 ... 31: PD7 selected as GPT2OCP1"]
pub type INPUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as GPT2OCP1 1: PA1 selected as GPT2OCP1 ... 31: PD7 selected as GPT2OCP1"]
pub type INPUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPT2OCP1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as GPT2OCP1 1: PA1 selected as GPT2OCP1 ... 31: PD7 selected as GPT2OCP1"]
    #[inline(always)]
    pub fn input_sel(&self) -> INPUT_SEL_R {
        INPUT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as GPT2OCP1 1: PA1 selected as GPT2OCP1 ... 31: PD7 selected as GPT2OCP1"]
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
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt2ocp1](index.html) module"]
pub struct GPT2OCP1_SPEC;
impl crate::RegisterSpec for GPT2OCP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpt2ocp1::R](R) reader structure"]
impl crate::Readable for GPT2OCP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpt2ocp1::W](W) writer structure"]
impl crate::Writable for GPT2OCP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPT2OCP1 to value 0"]
impl crate::Resettable for GPT2OCP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

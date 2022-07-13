#[doc = "Register `AGCCTRL1` reader"]
pub struct R(crate::R<AGCCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGCCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGCCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGCCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGCCTRL1` writer"]
pub struct W(crate::W<AGCCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGCCTRL1_SPEC>;
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
impl From<crate::W<AGCCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGCCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGC_REF` reader - Target value for the AGC control loop, given in 1-dB steps"]
pub type AGC_REF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AGC_REF` writer - Target value for the AGC control loop, given in 1-dB steps"]
pub type AGC_REF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AGCCTRL1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Target value for the AGC control loop, given in 1-dB steps"]
    #[inline(always)]
    pub fn agc_ref(&self) -> AGC_REF_R {
        AGC_REF_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Target value for the AGC control loop, given in 1-dB steps"]
    #[inline(always)]
    pub fn agc_ref(&mut self) -> AGC_REF_W<0> {
        AGC_REF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGC reference level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agcctrl1](index.html) module"]
pub struct AGCCTRL1_SPEC;
impl crate::RegisterSpec for AGCCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [agcctrl1::R](R) reader structure"]
impl crate::Readable for AGCCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agcctrl1::W](W) writer structure"]
impl crate::Writable for AGCCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AGCCTRL1 to value 0"]
impl crate::Resettable for AGCCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

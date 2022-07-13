#[doc = "Register `DACTEST0` reader"]
pub struct R(crate::R<DACTEST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACTEST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACTEST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACTEST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACTEST0` writer"]
pub struct W(crate::W<DACTEST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACTEST0_SPEC>;
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
impl From<crate::W<DACTEST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACTEST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_Q_O` reader - Q-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, or channel filtered data, then DAC_Q_O controls the part of the word in question that is actually multiplexed to the DAC, as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, the DAC outputs only zeros (minimum value)."]
pub type DAC_Q_O_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_Q_O` writer - Q-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, or channel filtered data, then DAC_Q_O controls the part of the word in question that is actually multiplexed to the DAC, as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, the DAC outputs only zeros (minimum value)."]
pub type DAC_Q_O_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACTEST0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Q-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, or channel filtered data, then DAC_Q_O controls the part of the word in question that is actually multiplexed to the DAC, as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_q_o(&self) -> DAC_Q_O_R {
        DAC_Q_O_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Q-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, or channel filtered data, then DAC_Q_O controls the part of the word in question that is actually multiplexed to the DAC, as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_q_o(&mut self) -> DAC_Q_O_W<0> {
        DAC_Q_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC override value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dactest0](index.html) module"]
pub struct DACTEST0_SPEC;
impl crate::RegisterSpec for DACTEST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dactest0::R](R) reader structure"]
impl crate::Readable for DACTEST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dactest0::W](W) writer structure"]
impl crate::Writable for DACTEST0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACTEST0 to value 0"]
impl crate::Resettable for DACTEST0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

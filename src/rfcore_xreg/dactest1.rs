#[doc = "Register `DACTEST1` reader"]
pub struct R(crate::R<DACTEST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACTEST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACTEST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACTEST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACTEST1` writer"]
pub struct W(crate::W<DACTEST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACTEST1_SPEC>;
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
impl From<crate::W<DACTEST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACTEST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_I_O` reader - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
pub struct DAC_I_O_R(crate::FieldReader<u8, u8>);
impl DAC_I_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_I_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_I_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_I_O` writer - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
pub struct DAC_I_O_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_I_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_i_o(&self) -> DAC_I_O_R {
        DAC_I_O_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_i_o(&mut self) -> DAC_I_O_W {
        DAC_I_O_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC override value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dactest1](index.html) module"]
pub struct DACTEST1_SPEC;
impl crate::RegisterSpec for DACTEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dactest1::R](R) reader structure"]
impl crate::Readable for DACTEST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dactest1::W](W) writer structure"]
impl crate::Writable for DACTEST1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACTEST1 to value 0"]
impl crate::Resettable for DACTEST1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

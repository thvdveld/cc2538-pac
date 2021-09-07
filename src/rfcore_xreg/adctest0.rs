#[doc = "Register `ADCTEST0` reader"]
pub struct R(crate::R<ADCTEST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCTEST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCTEST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCTEST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCTEST0` writer"]
pub struct W(crate::W<ADCTEST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCTEST0_SPEC>;
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
impl From<crate::W<ADCTEST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCTEST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_VREF_ADJ` reader - Quantizer threshold control for test and debug"]
pub struct ADC_VREF_ADJ_R(crate::FieldReader<u8, u8>);
impl ADC_VREF_ADJ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_VREF_ADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_VREF_ADJ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_VREF_ADJ` writer - Quantizer threshold control for test and debug"]
pub struct ADC_VREF_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_VREF_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `ADC_QUANT_ADJ` reader - Quantizer threshold control for test and debug"]
pub struct ADC_QUANT_ADJ_R(crate::FieldReader<u8, u8>);
impl ADC_QUANT_ADJ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_QUANT_ADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_QUANT_ADJ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_QUANT_ADJ` writer - Quantizer threshold control for test and debug"]
pub struct ADC_QUANT_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_QUANT_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ADC_GM_ADJ` reader - Gm-control for test and debug"]
pub struct ADC_GM_ADJ_R(crate::FieldReader<u8, u8>);
impl ADC_GM_ADJ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_GM_ADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_GM_ADJ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_GM_ADJ` writer - Gm-control for test and debug"]
pub struct ADC_GM_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_GM_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `ADC_DAC2_EN` reader - Enables DAC2 for enhanced ADC stability"]
pub struct ADC_DAC2_EN_R(crate::FieldReader<bool, bool>);
impl ADC_DAC2_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_DAC2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DAC2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_DAC2_EN` writer - Enables DAC2 for enhanced ADC stability"]
pub struct ADC_DAC2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DAC2_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_vref_adj(&self) -> ADC_VREF_ADJ_R {
        ADC_VREF_ADJ_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_quant_adj(&self) -> ADC_QUANT_ADJ_R {
        ADC_QUANT_ADJ_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 1:3 - Gm-control for test and debug"]
    #[inline(always)]
    pub fn adc_gm_adj(&self) -> ADC_GM_ADJ_R {
        ADC_GM_ADJ_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Enables DAC2 for enhanced ADC stability"]
    #[inline(always)]
    pub fn adc_dac2_en(&self) -> ADC_DAC2_EN_R {
        ADC_DAC2_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_vref_adj(&mut self) -> ADC_VREF_ADJ_W {
        ADC_VREF_ADJ_W { w: self }
    }
    #[doc = "Bits 4:5 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_quant_adj(&mut self) -> ADC_QUANT_ADJ_W {
        ADC_QUANT_ADJ_W { w: self }
    }
    #[doc = "Bits 1:3 - Gm-control for test and debug"]
    #[inline(always)]
    pub fn adc_gm_adj(&mut self) -> ADC_GM_ADJ_W {
        ADC_GM_ADJ_W { w: self }
    }
    #[doc = "Bit 0 - Enables DAC2 for enhanced ADC stability"]
    #[inline(always)]
    pub fn adc_dac2_en(&mut self) -> ADC_DAC2_EN_W {
        ADC_DAC2_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctest0](index.html) module"]
pub struct ADCTEST0_SPEC;
impl crate::RegisterSpec for ADCTEST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adctest0::R](R) reader structure"]
impl crate::Readable for ADCTEST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adctest0::W](W) writer structure"]
impl crate::Writable for ADCTEST0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCTEST0 to value 0"]
impl crate::Resettable for ADCTEST0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

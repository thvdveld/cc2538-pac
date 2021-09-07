#[doc = "Register `ADCTEST2` reader"]
pub struct R(crate::R<ADCTEST2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCTEST2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCTEST2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCTEST2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCTEST2` writer"]
pub struct W(crate::W<ADCTEST2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCTEST2_SPEC>;
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
impl From<crate::W<ADCTEST2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCTEST2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_TEST_MODE` reader - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
pub struct ADC_TEST_MODE_R(crate::FieldReader<u8, u8>);
impl ADC_TEST_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_TEST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_TEST_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_TEST_MODE` writer - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
pub struct ADC_TEST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_TEST_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `AAF_RS` reader - Controls series resistance of AAF"]
pub struct AAF_RS_R(crate::FieldReader<u8, u8>);
impl AAF_RS_R {
    pub(crate) fn new(bits: u8) -> Self {
        AAF_RS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AAF_RS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAF_RS` writer - Controls series resistance of AAF"]
pub struct AAF_RS_W<'a> {
    w: &'a mut W,
}
impl<'a> AAF_RS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `ADC_FF_ADJ` reader - Adjust feed forward"]
pub struct ADC_FF_ADJ_R(crate::FieldReader<u8, u8>);
impl ADC_FF_ADJ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_FF_ADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_FF_ADJ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_FF_ADJ` writer - Adjust feed forward"]
pub struct ADC_FF_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_FF_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `ADC_DAC_ROT` reader - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
pub struct ADC_DAC_ROT_R(crate::FieldReader<bool, bool>);
impl ADC_DAC_ROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_DAC_ROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DAC_ROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_DAC_ROT` writer - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
pub struct ADC_DAC_ROT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DAC_ROT_W<'a> {
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
    #[doc = "Bits 5:6 - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    pub fn adc_test_mode(&self) -> ADC_TEST_MODE_R {
        ADC_TEST_MODE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Controls series resistance of AAF"]
    #[inline(always)]
    pub fn aaf_rs(&self) -> AAF_RS_R {
        AAF_RS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - Adjust feed forward"]
    #[inline(always)]
    pub fn adc_ff_adj(&self) -> ADC_FF_ADJ_R {
        ADC_FF_ADJ_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    pub fn adc_dac_rot(&self) -> ADC_DAC_ROT_R {
        ADC_DAC_ROT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:6 - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    pub fn adc_test_mode(&mut self) -> ADC_TEST_MODE_W {
        ADC_TEST_MODE_W { w: self }
    }
    #[doc = "Bits 3:4 - Controls series resistance of AAF"]
    #[inline(always)]
    pub fn aaf_rs(&mut self) -> AAF_RS_W {
        AAF_RS_W { w: self }
    }
    #[doc = "Bits 1:2 - Adjust feed forward"]
    #[inline(always)]
    pub fn adc_ff_adj(&mut self) -> ADC_FF_ADJ_W {
        ADC_FF_ADJ_W { w: self }
    }
    #[doc = "Bit 0 - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    pub fn adc_dac_rot(&mut self) -> ADC_DAC_ROT_W {
        ADC_DAC_ROT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctest2](index.html) module"]
pub struct ADCTEST2_SPEC;
impl crate::RegisterSpec for ADCTEST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adctest2::R](R) reader structure"]
impl crate::Readable for ADCTEST2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adctest2::W](W) writer structure"]
impl crate::Writable for ADCTEST2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCTEST2 to value 0"]
impl crate::Resettable for ADCTEST2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

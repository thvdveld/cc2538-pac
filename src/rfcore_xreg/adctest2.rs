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
pub type ADC_TEST_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_TEST_MODE` writer - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
pub type ADC_TEST_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCTEST2_SPEC, u8, u8, 2, O>;
#[doc = "Field `AAF_RS` reader - Controls series resistance of AAF"]
pub type AAF_RS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AAF_RS` writer - Controls series resistance of AAF"]
pub type AAF_RS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCTEST2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_FF_ADJ` reader - Adjust feed forward"]
pub type ADC_FF_ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_FF_ADJ` writer - Adjust feed forward"]
pub type ADC_FF_ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCTEST2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_DAC_ROT` reader - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
pub type ADC_DAC_ROT_R = crate::BitReader<bool>;
#[doc = "Field `ADC_DAC_ROT` writer - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
pub type ADC_DAC_ROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCTEST2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 5:6 - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    pub fn adc_test_mode(&self) -> ADC_TEST_MODE_R {
        ADC_TEST_MODE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Controls series resistance of AAF"]
    #[inline(always)]
    pub fn aaf_rs(&self) -> AAF_RS_R {
        AAF_RS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 1:2 - Adjust feed forward"]
    #[inline(always)]
    pub fn adc_ff_adj(&self) -> ADC_FF_ADJ_R {
        ADC_FF_ADJ_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 0 - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    pub fn adc_dac_rot(&self) -> ADC_DAC_ROT_R {
        ADC_DAC_ROT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:6 - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    pub fn adc_test_mode(&mut self) -> ADC_TEST_MODE_W<5> {
        ADC_TEST_MODE_W::new(self)
    }
    #[doc = "Bits 3:4 - Controls series resistance of AAF"]
    #[inline(always)]
    pub fn aaf_rs(&mut self) -> AAF_RS_W<3> {
        AAF_RS_W::new(self)
    }
    #[doc = "Bits 1:2 - Adjust feed forward"]
    #[inline(always)]
    pub fn adc_ff_adj(&mut self) -> ADC_FF_ADJ_W<1> {
        ADC_FF_ADJ_W::new(self)
    }
    #[doc = "Bit 0 - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    pub fn adc_dac_rot(&mut self) -> ADC_DAC_ROT_W<0> {
        ADC_DAC_ROT_W::new(self)
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

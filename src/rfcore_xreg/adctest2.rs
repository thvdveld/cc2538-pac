#[doc = "Register `ADCTEST2` reader"]
pub type R = crate::R<ADCTEST2_SPEC>;
#[doc = "Register `ADCTEST2` writer"]
pub type W = crate::W<ADCTEST2_SPEC>;
#[doc = "Field `ADC_DAC_ROT` reader - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
pub type ADC_DAC_ROT_R = crate::BitReader;
#[doc = "Field `ADC_DAC_ROT` writer - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
pub type ADC_DAC_ROT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_FF_ADJ` reader - Adjust feed forward"]
pub type ADC_FF_ADJ_R = crate::FieldReader;
#[doc = "Field `ADC_FF_ADJ` writer - Adjust feed forward"]
pub type ADC_FF_ADJ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `AAF_RS` reader - Controls series resistance of AAF"]
pub type AAF_RS_R = crate::FieldReader;
#[doc = "Field `AAF_RS` writer - Controls series resistance of AAF"]
pub type AAF_RS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ADC_TEST_MODE` reader - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
pub type ADC_TEST_MODE_R = crate::FieldReader;
#[doc = "Field `ADC_TEST_MODE` writer - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
pub type ADC_TEST_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    pub fn adc_dac_rot(&self) -> ADC_DAC_ROT_R {
        ADC_DAC_ROT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Adjust feed forward"]
    #[inline(always)]
    pub fn adc_ff_adj(&self) -> ADC_FF_ADJ_R {
        ADC_FF_ADJ_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Controls series resistance of AAF"]
    #[inline(always)]
    pub fn aaf_rs(&self) -> AAF_RS_R {
        AAF_RS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    pub fn adc_test_mode(&self) -> ADC_TEST_MODE_R {
        ADC_TEST_MODE_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dac_rot(&mut self) -> ADC_DAC_ROT_W<ADCTEST2_SPEC, 0> {
        ADC_DAC_ROT_W::new(self)
    }
    #[doc = "Bits 1:2 - Adjust feed forward"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ff_adj(&mut self) -> ADC_FF_ADJ_W<ADCTEST2_SPEC, 1> {
        ADC_FF_ADJ_W::new(self)
    }
    #[doc = "Bits 3:4 - Controls series resistance of AAF"]
    #[inline(always)]
    #[must_use]
    pub fn aaf_rs(&mut self) -> AAF_RS_W<ADCTEST2_SPEC, 3> {
        AAF_RS_W::new(self)
    }
    #[doc = "Bits 5:6 - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_test_mode(&mut self) -> ADC_TEST_MODE_W<ADCTEST2_SPEC, 5> {
        ADC_TEST_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC tuning\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctest2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctest2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCTEST2_SPEC;
impl crate::RegisterSpec for ADCTEST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adctest2::R`](R) reader structure"]
impl crate::Readable for ADCTEST2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adctest2::W`](W) writer structure"]
impl crate::Writable for ADCTEST2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCTEST2 to value 0"]
impl crate::Resettable for ADCTEST2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

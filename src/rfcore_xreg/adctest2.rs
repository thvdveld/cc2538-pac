#[doc = "Register `ADCTEST2` reader"]
pub type R = crate::R<Adctest2Spec>;
#[doc = "Register `ADCTEST2` writer"]
pub type W = crate::W<Adctest2Spec>;
#[doc = "Field `ADC_DAC_ROT` reader - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
pub type AdcDacRotR = crate::BitReader;
#[doc = "Field `ADC_DAC_ROT` writer - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
pub type AdcDacRotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_FF_ADJ` reader - Adjust feed forward"]
pub type AdcFfAdjR = crate::FieldReader;
#[doc = "Field `ADC_FF_ADJ` writer - Adjust feed forward"]
pub type AdcFfAdjW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AAF_RS` reader - Controls series resistance of AAF"]
pub type AafRsR = crate::FieldReader;
#[doc = "Field `AAF_RS` writer - Controls series resistance of AAF"]
pub type AafRsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_TEST_MODE` reader - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
pub type AdcTestModeR = crate::FieldReader;
#[doc = "Field `ADC_TEST_MODE` writer - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
pub type AdcTestModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    pub fn adc_dac_rot(&self) -> AdcDacRotR {
        AdcDacRotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Adjust feed forward"]
    #[inline(always)]
    pub fn adc_ff_adj(&self) -> AdcFfAdjR {
        AdcFfAdjR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Controls series resistance of AAF"]
    #[inline(always)]
    pub fn aaf_rs(&self) -> AafRsR {
        AafRsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    pub fn adc_test_mode(&self) -> AdcTestModeR {
        AdcTestModeR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dac_rot(&mut self) -> AdcDacRotW<Adctest2Spec> {
        AdcDacRotW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Adjust feed forward"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ff_adj(&mut self) -> AdcFfAdjW<Adctest2Spec> {
        AdcFfAdjW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Controls series resistance of AAF"]
    #[inline(always)]
    #[must_use]
    pub fn aaf_rs(&mut self) -> AafRsW<Adctest2Spec> {
        AafRsW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_test_mode(&mut self) -> AdcTestModeW<Adctest2Spec> {
        AdcTestModeW::new(self, 5)
    }
}
#[doc = "ADC tuning\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctest2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctest2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adctest2Spec;
impl crate::RegisterSpec for Adctest2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adctest2::R`](R) reader structure"]
impl crate::Readable for Adctest2Spec {}
#[doc = "`write(|w| ..)` method takes [`adctest2::W`](W) writer structure"]
impl crate::Writable for Adctest2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCTEST2 to value 0"]
impl crate::Resettable for Adctest2Spec {
    const RESET_VALUE: u32 = 0;
}

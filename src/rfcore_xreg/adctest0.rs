#[doc = "Register `ADCTEST0` reader"]
pub type R = crate::R<Adctest0Spec>;
#[doc = "Register `ADCTEST0` writer"]
pub type W = crate::W<Adctest0Spec>;
#[doc = "Field `ADC_DAC2_EN` reader - Enables DAC2 for enhanced ADC stability"]
pub type AdcDac2EnR = crate::BitReader;
#[doc = "Field `ADC_DAC2_EN` writer - Enables DAC2 for enhanced ADC stability"]
pub type AdcDac2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_GM_ADJ` reader - Gm-control for test and debug"]
pub type AdcGmAdjR = crate::FieldReader;
#[doc = "Field `ADC_GM_ADJ` writer - Gm-control for test and debug"]
pub type AdcGmAdjW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADC_QUANT_ADJ` reader - Quantizer threshold control for test and debug"]
pub type AdcQuantAdjR = crate::FieldReader;
#[doc = "Field `ADC_QUANT_ADJ` writer - Quantizer threshold control for test and debug"]
pub type AdcQuantAdjW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_VREF_ADJ` reader - Quantizer threshold control for test and debug"]
pub type AdcVrefAdjR = crate::FieldReader;
#[doc = "Field `ADC_VREF_ADJ` writer - Quantizer threshold control for test and debug"]
pub type AdcVrefAdjW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enables DAC2 for enhanced ADC stability"]
    #[inline(always)]
    pub fn adc_dac2_en(&self) -> AdcDac2EnR {
        AdcDac2EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Gm-control for test and debug"]
    #[inline(always)]
    pub fn adc_gm_adj(&self) -> AdcGmAdjR {
        AdcGmAdjR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_quant_adj(&self) -> AdcQuantAdjR {
        AdcQuantAdjR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_vref_adj(&self) -> AdcVrefAdjR {
        AdcVrefAdjR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables DAC2 for enhanced ADC stability"]
    #[inline(always)]
    pub fn adc_dac2_en(&mut self) -> AdcDac2EnW<Adctest0Spec> {
        AdcDac2EnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Gm-control for test and debug"]
    #[inline(always)]
    pub fn adc_gm_adj(&mut self) -> AdcGmAdjW<Adctest0Spec> {
        AdcGmAdjW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_quant_adj(&mut self) -> AdcQuantAdjW<Adctest0Spec> {
        AdcQuantAdjW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_vref_adj(&mut self) -> AdcVrefAdjW<Adctest0Spec> {
        AdcVrefAdjW::new(self, 6)
    }
}
#[doc = "ADC tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`adctest0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adctest0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adctest0Spec;
impl crate::RegisterSpec for Adctest0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adctest0::R`](R) reader structure"]
impl crate::Readable for Adctest0Spec {}
#[doc = "`write(|w| ..)` method takes [`adctest0::W`](W) writer structure"]
impl crate::Writable for Adctest0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCTEST0 to value 0"]
impl crate::Resettable for Adctest0Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `ADCTEST0` reader"]
pub type R = crate::R<ADCTEST0_SPEC>;
#[doc = "Register `ADCTEST0` writer"]
pub type W = crate::W<ADCTEST0_SPEC>;
#[doc = "Field `ADC_DAC2_EN` reader - Enables DAC2 for enhanced ADC stability"]
pub type ADC_DAC2_EN_R = crate::BitReader;
#[doc = "Field `ADC_DAC2_EN` writer - Enables DAC2 for enhanced ADC stability"]
pub type ADC_DAC2_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_GM_ADJ` reader - Gm-control for test and debug"]
pub type ADC_GM_ADJ_R = crate::FieldReader;
#[doc = "Field `ADC_GM_ADJ` writer - Gm-control for test and debug"]
pub type ADC_GM_ADJ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ADC_QUANT_ADJ` reader - Quantizer threshold control for test and debug"]
pub type ADC_QUANT_ADJ_R = crate::FieldReader;
#[doc = "Field `ADC_QUANT_ADJ` writer - Quantizer threshold control for test and debug"]
pub type ADC_QUANT_ADJ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ADC_VREF_ADJ` reader - Quantizer threshold control for test and debug"]
pub type ADC_VREF_ADJ_R = crate::FieldReader;
#[doc = "Field `ADC_VREF_ADJ` writer - Quantizer threshold control for test and debug"]
pub type ADC_VREF_ADJ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - Enables DAC2 for enhanced ADC stability"]
    #[inline(always)]
    pub fn adc_dac2_en(&self) -> ADC_DAC2_EN_R {
        ADC_DAC2_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Gm-control for test and debug"]
    #[inline(always)]
    pub fn adc_gm_adj(&self) -> ADC_GM_ADJ_R {
        ADC_GM_ADJ_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_quant_adj(&self) -> ADC_QUANT_ADJ_R {
        ADC_QUANT_ADJ_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_vref_adj(&self) -> ADC_VREF_ADJ_R {
        ADC_VREF_ADJ_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables DAC2 for enhanced ADC stability"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dac2_en(&mut self) -> ADC_DAC2_EN_W<ADCTEST0_SPEC, 0> {
        ADC_DAC2_EN_W::new(self)
    }
    #[doc = "Bits 1:3 - Gm-control for test and debug"]
    #[inline(always)]
    #[must_use]
    pub fn adc_gm_adj(&mut self) -> ADC_GM_ADJ_W<ADCTEST0_SPEC, 1> {
        ADC_GM_ADJ_W::new(self)
    }
    #[doc = "Bits 4:5 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    #[must_use]
    pub fn adc_quant_adj(&mut self) -> ADC_QUANT_ADJ_W<ADCTEST0_SPEC, 4> {
        ADC_QUANT_ADJ_W::new(self)
    }
    #[doc = "Bits 6:7 - Quantizer threshold control for test and debug"]
    #[inline(always)]
    #[must_use]
    pub fn adc_vref_adj(&mut self) -> ADC_VREF_ADJ_W<ADCTEST0_SPEC, 6> {
        ADC_VREF_ADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC tuning\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctest0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctest0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCTEST0_SPEC;
impl crate::RegisterSpec for ADCTEST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adctest0::R`](R) reader structure"]
impl crate::Readable for ADCTEST0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adctest0::W`](W) writer structure"]
impl crate::Writable for ADCTEST0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCTEST0 to value 0"]
impl crate::Resettable for ADCTEST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

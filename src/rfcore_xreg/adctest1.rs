#[doc = "Register `ADCTEST1` reader"]
pub type R = crate::R<Adctest1Spec>;
#[doc = "Register `ADCTEST1` writer"]
pub type W = crate::W<Adctest1Spec>;
#[doc = "Field `ADC_C3_ADJ` reader - Used to adjust capacitor values in ADC"]
pub type AdcC3AdjR = crate::FieldReader;
#[doc = "Field `ADC_C3_ADJ` writer - Used to adjust capacitor values in ADC"]
pub type AdcC3AdjW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_C2_ADJ` reader - Used to adjust capacitor values in ADC"]
pub type AdcC2AdjR = crate::FieldReader;
#[doc = "Field `ADC_C2_ADJ` writer - Used to adjust capacitor values in ADC"]
pub type AdcC2AdjW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_TEST_CTRL` reader - ADC test mode selector"]
pub type AdcTestCtrlR = crate::FieldReader;
#[doc = "Field `ADC_TEST_CTRL` writer - ADC test mode selector"]
pub type AdcTestCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c3_adj(&self) -> AdcC3AdjR {
        AdcC3AdjR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c2_adj(&self) -> AdcC2AdjR {
        AdcC2AdjR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - ADC test mode selector"]
    #[inline(always)]
    pub fn adc_test_ctrl(&self) -> AdcTestCtrlR {
        AdcTestCtrlR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c3_adj(&mut self) -> AdcC3AdjW<Adctest1Spec> {
        AdcC3AdjW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c2_adj(&mut self) -> AdcC2AdjW<Adctest1Spec> {
        AdcC2AdjW::new(self, 2)
    }
    #[doc = "Bits 4:7 - ADC test mode selector"]
    #[inline(always)]
    pub fn adc_test_ctrl(&mut self) -> AdcTestCtrlW<Adctest1Spec> {
        AdcTestCtrlW::new(self, 4)
    }
}
#[doc = "ADC tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`adctest1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adctest1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adctest1Spec;
impl crate::RegisterSpec for Adctest1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adctest1::R`](R) reader structure"]
impl crate::Readable for Adctest1Spec {}
#[doc = "`write(|w| ..)` method takes [`adctest1::W`](W) writer structure"]
impl crate::Writable for Adctest1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCTEST1 to value 0"]
impl crate::Resettable for Adctest1Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `ADCTEST1` reader"]
pub type R = crate::R<ADCTEST1_SPEC>;
#[doc = "Register `ADCTEST1` writer"]
pub type W = crate::W<ADCTEST1_SPEC>;
#[doc = "Field `ADC_C3_ADJ` reader - Used to adjust capacitor values in ADC"]
pub type ADC_C3_ADJ_R = crate::FieldReader;
#[doc = "Field `ADC_C3_ADJ` writer - Used to adjust capacitor values in ADC"]
pub type ADC_C3_ADJ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_C2_ADJ` reader - Used to adjust capacitor values in ADC"]
pub type ADC_C2_ADJ_R = crate::FieldReader;
#[doc = "Field `ADC_C2_ADJ` writer - Used to adjust capacitor values in ADC"]
pub type ADC_C2_ADJ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_TEST_CTRL` reader - ADC test mode selector"]
pub type ADC_TEST_CTRL_R = crate::FieldReader;
#[doc = "Field `ADC_TEST_CTRL` writer - ADC test mode selector"]
pub type ADC_TEST_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c3_adj(&self) -> ADC_C3_ADJ_R {
        ADC_C3_ADJ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c2_adj(&self) -> ADC_C2_ADJ_R {
        ADC_C2_ADJ_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - ADC test mode selector"]
    #[inline(always)]
    pub fn adc_test_ctrl(&self) -> ADC_TEST_CTRL_R {
        ADC_TEST_CTRL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc_c3_adj(&mut self) -> ADC_C3_ADJ_W<ADCTEST1_SPEC> {
        ADC_C3_ADJ_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc_c2_adj(&mut self) -> ADC_C2_ADJ_W<ADCTEST1_SPEC> {
        ADC_C2_ADJ_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - ADC test mode selector"]
    #[inline(always)]
    #[must_use]
    pub fn adc_test_ctrl(&mut self) -> ADC_TEST_CTRL_W<ADCTEST1_SPEC> {
        ADC_TEST_CTRL_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC tuning\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctest1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctest1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCTEST1_SPEC;
impl crate::RegisterSpec for ADCTEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adctest1::R`](R) reader structure"]
impl crate::Readable for ADCTEST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adctest1::W`](W) writer structure"]
impl crate::Writable for ADCTEST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCTEST1 to value 0"]
impl crate::Resettable for ADCTEST1_SPEC {
    const RESET_VALUE: u32 = 0;
}

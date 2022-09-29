#[doc = "Register `ADCTEST1` reader"]
pub struct R(crate::R<ADCTEST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCTEST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCTEST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCTEST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCTEST1` writer"]
pub struct W(crate::W<ADCTEST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCTEST1_SPEC>;
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
impl From<crate::W<ADCTEST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCTEST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_C3_ADJ` reader - Used to adjust capacitor values in ADC"]
pub type ADC_C3_ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_C3_ADJ` writer - Used to adjust capacitor values in ADC"]
pub type ADC_C3_ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCTEST1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_C2_ADJ` reader - Used to adjust capacitor values in ADC"]
pub type ADC_C2_ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_C2_ADJ` writer - Used to adjust capacitor values in ADC"]
pub type ADC_C2_ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCTEST1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_TEST_CTRL` reader - ADC test mode selector"]
pub type ADC_TEST_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_TEST_CTRL` writer - ADC test mode selector"]
pub type ADC_TEST_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCTEST1_SPEC, u8, u8, 4, O>;
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
    pub fn adc_c3_adj(&mut self) -> ADC_C3_ADJ_W<0> {
        ADC_C3_ADJ_W::new(self)
    }
    #[doc = "Bits 2:3 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c2_adj(&mut self) -> ADC_C2_ADJ_W<2> {
        ADC_C2_ADJ_W::new(self)
    }
    #[doc = "Bits 4:7 - ADC test mode selector"]
    #[inline(always)]
    pub fn adc_test_ctrl(&mut self) -> ADC_TEST_CTRL_W<4> {
        ADC_TEST_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctest1](index.html) module"]
pub struct ADCTEST1_SPEC;
impl crate::RegisterSpec for ADCTEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adctest1::R](R) reader structure"]
impl crate::Readable for ADCTEST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adctest1::W](W) writer structure"]
impl crate::Writable for ADCTEST1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCTEST1 to value 0"]
impl crate::Resettable for ADCTEST1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `ADCH` reader"]
pub type R = crate::R<ADCH_SPEC>;
#[doc = "Field `ADC` reader - Most-significant part of ADC conversion result"]
pub type ADC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Most-significant part of ADC conversion result"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "This register contains the most-significant part of ADC conversion result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCH_SPEC;
impl crate::RegisterSpec for ADCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adch::R`](R) reader structure"]
impl crate::Readable for ADCH_SPEC {}
#[doc = "`reset()` method sets ADCH to value 0"]
impl crate::Resettable for ADCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

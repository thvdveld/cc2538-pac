#[doc = "Register `ADCL` reader"]
pub type R = crate::R<ADCL_SPEC>;
#[doc = "Field `ADC` reader - Least-significant part of ADC conversion result"]
pub type ADC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 2:7 - Least-significant part of ADC conversion result"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[doc = "This register contains the least-significant part of ADC conversion result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCL_SPEC;
impl crate::RegisterSpec for ADCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcl::R`](R) reader structure"]
impl crate::Readable for ADCL_SPEC {}
#[doc = "`reset()` method sets ADCL to value 0"]
impl crate::Resettable for ADCL_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `ADCL` reader"]
pub struct R(crate::R<ADCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC` reader - Least-significant part of ADC conversion result"]
pub type ADC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 2:7 - Least-significant part of ADC conversion result"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[doc = "This register contains the least-significant part of ADC conversion result.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcl](index.html) module"]
pub struct ADCL_SPEC;
impl crate::RegisterSpec for ADCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcl::R](R) reader structure"]
impl crate::Readable for ADCL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADCL to value 0"]
impl crate::Resettable for ADCL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

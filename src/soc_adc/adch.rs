#[doc = "Register `ADCH` reader"]
pub struct R(crate::R<ADCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC` reader - Most-significant part of ADC conversion result"]
pub struct ADC_R(crate::FieldReader<u8, u8>);
impl ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Most-significant part of ADC conversion result"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "This register contains the most-significant part of ADC conversion result.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adch](index.html) module"]
pub struct ADCH_SPEC;
impl crate::RegisterSpec for ADCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adch::R](R) reader structure"]
impl crate::Readable for ADCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADCH to value 0"]
impl crate::Resettable for ADCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `ADCH` reader"]
pub type R = crate::R<AdchSpec>;
#[doc = "Field `ADC` reader - Most-significant part of ADC conversion result"]
pub type AdcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Most-significant part of ADC conversion result"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "This register contains the most-significant part of ADC conversion result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdchSpec;
impl crate::RegisterSpec for AdchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adch::R`](R) reader structure"]
impl crate::Readable for AdchSpec {}
#[doc = "`reset()` method sets ADCH to value 0"]
impl crate::Resettable for AdchSpec {
    const RESET_VALUE: u32 = 0;
}

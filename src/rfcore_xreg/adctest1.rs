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
#[doc = "Field `ADC_TEST_CTRL` reader - ADC test mode selector"]
pub struct ADC_TEST_CTRL_R(crate::FieldReader<u8, u8>);
impl ADC_TEST_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_TEST_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_TEST_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_TEST_CTRL` writer - ADC test mode selector"]
pub struct ADC_TEST_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_TEST_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `ADC_C2_ADJ` reader - Used to adjust capacitor values in ADC"]
pub struct ADC_C2_ADJ_R(crate::FieldReader<u8, u8>);
impl ADC_C2_ADJ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_C2_ADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_C2_ADJ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_C2_ADJ` writer - Used to adjust capacitor values in ADC"]
pub struct ADC_C2_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_C2_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `ADC_C3_ADJ` reader - Used to adjust capacitor values in ADC"]
pub struct ADC_C3_ADJ_R(crate::FieldReader<u8, u8>);
impl ADC_C3_ADJ_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_C3_ADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_C3_ADJ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_C3_ADJ` writer - Used to adjust capacitor values in ADC"]
pub struct ADC_C3_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_C3_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - ADC test mode selector"]
    #[inline(always)]
    pub fn adc_test_ctrl(&self) -> ADC_TEST_CTRL_R {
        ADC_TEST_CTRL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c2_adj(&self) -> ADC_C2_ADJ_R {
        ADC_C2_ADJ_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c3_adj(&self) -> ADC_C3_ADJ_R {
        ADC_C3_ADJ_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - ADC test mode selector"]
    #[inline(always)]
    pub fn adc_test_ctrl(&mut self) -> ADC_TEST_CTRL_W {
        ADC_TEST_CTRL_W { w: self }
    }
    #[doc = "Bits 2:3 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c2_adj(&mut self) -> ADC_C2_ADJ_W {
        ADC_C2_ADJ_W { w: self }
    }
    #[doc = "Bits 0:1 - Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c3_adj(&mut self) -> ADC_C3_ADJ_W {
        ADC_C3_ADJ_W { w: self }
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

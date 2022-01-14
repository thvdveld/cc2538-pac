#[doc = "Register `TR0` reader"]
pub struct R(crate::R<TR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR0` writer"]
pub struct W(crate::W<TR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR0_SPEC>;
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
impl From<crate::W<TR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCTM` reader - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
pub struct ADCTM_R(crate::FieldReader<bool, bool>);
impl ADCTM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADCTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCTM` writer - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
pub struct ADCTM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
    #[inline(always)]
    pub fn adctm(&self) -> ADCTM_R {
        ADCTM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set to 1 to connect the temperature sensor to the SOC_ADC. See also RFCORE_XREG_ATEST register description to enable the temperature sensor."]
    #[inline(always)]
    pub fn adctm(&mut self) -> ADCTM_W {
        ADCTM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr0](index.html) module"]
pub struct TR0_SPEC;
impl crate::RegisterSpec for TR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr0::R](R) reader structure"]
impl crate::Readable for TR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr0::W](W) writer structure"]
impl crate::Writable for TR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR0 to value 0"]
impl crate::Resettable for TR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

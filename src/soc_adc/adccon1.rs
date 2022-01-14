#[doc = "Register `ADCCON1` reader"]
pub struct R(crate::R<ADCCON1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCON1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCON1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCON1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCON1` writer"]
pub struct W(crate::W<ADCCON1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCON1_SPEC>;
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
impl From<crate::W<ADCCON1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCON1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOC` reader - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
pub struct EOC_R(crate::FieldReader<bool, bool>);
impl EOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC` writer - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
pub struct EOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ST` reader - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
pub struct ST_R(crate::FieldReader<bool, bool>);
impl ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST` writer - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `STSEL` reader - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
pub struct STSEL_R(crate::FieldReader<u8, u8>);
impl STSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSEL` writer - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
pub struct STSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `RCTRL` reader - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
pub struct RCTRL_R(crate::FieldReader<u8, u8>);
impl RCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCTRL` writer - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
pub struct RCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
    #[inline(always)]
    pub fn stsel(&self) -> STSEL_R {
        STSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
    #[inline(always)]
    pub fn rctrl(&self) -> RCTRL_R {
        RCTRL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W { w: self }
    }
    #[doc = "Bit 6 - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Bits 4:5 - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
    #[inline(always)]
    pub fn stsel(&mut self) -> STSEL_W {
        STSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
    #[inline(always)]
    pub fn rctrl(&mut self) -> RCTRL_W {
        RCTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the ADC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adccon1](index.html) module"]
pub struct ADCCON1_SPEC;
impl crate::RegisterSpec for ADCCON1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adccon1::R](R) reader structure"]
impl crate::Readable for ADCCON1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adccon1::W](W) writer structure"]
impl crate::Writable for ADCCON1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCON1 to value 0"]
impl crate::Resettable for ADCCON1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

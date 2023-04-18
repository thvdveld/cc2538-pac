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
#[doc = "Field `RCTRL` reader - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
pub type RCTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCTRL` writer - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
pub type RCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCON1_SPEC, u8, u8, 2, O>;
#[doc = "Field `STSEL` reader - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
pub type STSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STSEL` writer - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
pub type STSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCON1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ST` reader - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCON1_SPEC, bool, O>;
#[doc = "Field `EOC` reader - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
pub type EOC_R = crate::BitReader<bool>;
#[doc = "Field `EOC` writer - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCON1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:3 - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
    #[inline(always)]
    pub fn rctrl(&self) -> RCTRL_R {
        RCTRL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
    #[inline(always)]
    pub fn stsel(&self) -> STSEL_R {
        STSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
    #[inline(always)]
    #[must_use]
    pub fn rctrl(&mut self) -> RCTRL_W<2> {
        RCTRL_W::new(self)
    }
    #[doc = "Bits 4:5 - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
    #[inline(always)]
    #[must_use]
    pub fn stsel(&mut self) -> STSEL_W<4> {
        STSEL_W::new(self)
    }
    #[doc = "Bit 6 - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<6> {
        ST_W::new(self)
    }
    #[doc = "Bit 7 - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<7> {
        EOC_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCON1 to value 0"]
impl crate::Resettable for ADCCON1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

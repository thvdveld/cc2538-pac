#[doc = "Register `ADCCON1` reader"]
pub type R = crate::R<Adccon1Spec>;
#[doc = "Register `ADCCON1` writer"]
pub type W = crate::W<Adccon1Spec>;
#[doc = "Field `RCTRL` reader - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
pub type RctrlR = crate::FieldReader;
#[doc = "Field `RCTRL` writer - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
pub type RctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STSEL` reader - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
pub type StselR = crate::FieldReader;
#[doc = "Field `STSEL` writer - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
pub type StselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ST` reader - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
pub type StR = crate::BitReader;
#[doc = "Field `ST` writer - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
pub type StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:3 - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
    #[inline(always)]
    pub fn rctrl(&self) -> RctrlR {
        RctrlR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
    #[inline(always)]
    pub fn stsel(&self) -> StselR {
        StselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - Controls the 16-bit random-number generator (see User Guide Chapter 16) When 01 is written, the setting automatically returns to 00 when the operation completes. 00: Normal operation (13x unrolling) 01: Clock the LFSR once (13x unrolling) 10: Reserved 11: Stopped. The random-number generator is turned off."]
    #[inline(always)]
    pub fn rctrl(&mut self) -> RctrlW<Adccon1Spec> {
        RctrlW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Start select Selects the event that starts a new conversion sequence 00: Not implemented 01: Full speed. Do not wait for triggers 10: Timer 1 channel 0 compare event 11: ADCCON1.ST = 1"]
    #[inline(always)]
    pub fn stsel(&mut self) -> StselW<Adccon1Spec> {
        StselW::new(self, 4)
    }
    #[doc = "Bit 6 - Start conversion Read as 1 until conversion completes 0: No conversion in progress. 1: Start a conversion sequence if ADCCON1.STSEL = 11 and no sequence is running."]
    #[inline(always)]
    pub fn st(&mut self) -> StW<Adccon1Spec> {
        StW::new(self, 6)
    }
    #[doc = "Bit 7 - End of conversion. Cleared when ADCH has been read. If a new conversion is completed before the previous data has been read, the EOC bit remains high. 0: Conversion not complete 1: Conversion completed"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<Adccon1Spec> {
        EocW::new(self, 7)
    }
}
#[doc = "This register controls the ADC.\n\nYou can [`read`](crate::Reg::read) this register and get [`adccon1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccon1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adccon1Spec;
impl crate::RegisterSpec for Adccon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adccon1::R`](R) reader structure"]
impl crate::Readable for Adccon1Spec {}
#[doc = "`write(|w| ..)` method takes [`adccon1::W`](W) writer structure"]
impl crate::Writable for Adccon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCON1 to value 0"]
impl crate::Resettable for Adccon1Spec {
    const RESET_VALUE: u32 = 0;
}

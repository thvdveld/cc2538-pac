#[doc = "Register `MDMTEST1` reader"]
pub type R = crate::R<Mdmtest1Spec>;
#[doc = "Register `MDMTEST1` writer"]
pub type W = crate::W<Mdmtest1Spec>;
#[doc = "Field `LOOPBACK_EN` reader - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
pub type LoopbackEnR = crate::BitReader;
#[doc = "Field `LOOPBACK_EN` writer - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
pub type LoopbackEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFC_SNIFF_EN` reader - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
pub type RfcSniffEnR = crate::BitReader;
#[doc = "Field `RFC_SNIFF_EN` writer - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
pub type RfcSniffEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMP_AMP` reader - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
pub type RampAmpR = crate::BitReader;
#[doc = "Field `RAMP_AMP` writer - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
pub type RampAmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD_IF` reader - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
pub type ModIfR = crate::BitReader;
#[doc = "Field `MOD_IF` writer - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
pub type ModIfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEMIRROR_IF` reader - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
pub type UsemirrorIfR = crate::BitReader;
#[doc = "Field `USEMIRROR_IF` writer - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
pub type UsemirrorIfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
    #[inline(always)]
    pub fn loopback_en(&self) -> LoopbackEnR {
        LoopbackEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
    #[inline(always)]
    pub fn rfc_sniff_en(&self) -> RfcSniffEnR {
        RfcSniffEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
    #[inline(always)]
    pub fn ramp_amp(&self) -> RampAmpR {
        RampAmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
    #[inline(always)]
    pub fn mod_if(&self) -> ModIfR {
        ModIfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
    #[inline(always)]
    pub fn usemirror_if(&self) -> UsemirrorIfR {
        UsemirrorIfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
    #[inline(always)]
    #[must_use]
    pub fn loopback_en(&mut self) -> LoopbackEnW<Mdmtest1Spec> {
        LoopbackEnW::new(self, 0)
    }
    #[doc = "Bit 2 - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_sniff_en(&mut self) -> RfcSniffEnW<Mdmtest1Spec> {
        RfcSniffEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
    #[inline(always)]
    #[must_use]
    pub fn ramp_amp(&mut self) -> RampAmpW<Mdmtest1Spec> {
        RampAmpW::new(self, 3)
    }
    #[doc = "Bit 4 - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
    #[inline(always)]
    #[must_use]
    pub fn mod_if(&mut self) -> ModIfW<Mdmtest1Spec> {
        ModIfW::new(self, 4)
    }
    #[doc = "Bit 5 - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
    #[inline(always)]
    #[must_use]
    pub fn usemirror_if(&mut self) -> UsemirrorIfW<Mdmtest1Spec> {
        UsemirrorIfW::new(self, 5)
    }
}
#[doc = "Test Register for Modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdmtest1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdmtest1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdmtest1Spec;
impl crate::RegisterSpec for Mdmtest1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdmtest1::R`](R) reader structure"]
impl crate::Readable for Mdmtest1Spec {}
#[doc = "`write(|w| ..)` method takes [`mdmtest1::W`](W) writer structure"]
impl crate::Writable for Mdmtest1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMTEST1 to value 0"]
impl crate::Resettable for Mdmtest1Spec {
    const RESET_VALUE: u32 = 0;
}

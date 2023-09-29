#[doc = "Register `MDMTEST1` reader"]
pub type R = crate::R<MDMTEST1_SPEC>;
#[doc = "Register `MDMTEST1` writer"]
pub type W = crate::W<MDMTEST1_SPEC>;
#[doc = "Field `LOOPBACK_EN` reader - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
pub type LOOPBACK_EN_R = crate::BitReader;
#[doc = "Field `LOOPBACK_EN` writer - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
pub type LOOPBACK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFC_SNIFF_EN` reader - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
pub type RFC_SNIFF_EN_R = crate::BitReader;
#[doc = "Field `RFC_SNIFF_EN` writer - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
pub type RFC_SNIFF_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RAMP_AMP` reader - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
pub type RAMP_AMP_R = crate::BitReader;
#[doc = "Field `RAMP_AMP` writer - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
pub type RAMP_AMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MOD_IF` reader - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
pub type MOD_IF_R = crate::BitReader;
#[doc = "Field `MOD_IF` writer - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
pub type MOD_IF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USEMIRROR_IF` reader - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
pub type USEMIRROR_IF_R = crate::BitReader;
#[doc = "Field `USEMIRROR_IF` writer - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
pub type USEMIRROR_IF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
    #[inline(always)]
    pub fn loopback_en(&self) -> LOOPBACK_EN_R {
        LOOPBACK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
    #[inline(always)]
    pub fn rfc_sniff_en(&self) -> RFC_SNIFF_EN_R {
        RFC_SNIFF_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
    #[inline(always)]
    pub fn ramp_amp(&self) -> RAMP_AMP_R {
        RAMP_AMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
    #[inline(always)]
    pub fn mod_if(&self) -> MOD_IF_R {
        MOD_IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
    #[inline(always)]
    pub fn usemirror_if(&self) -> USEMIRROR_IF_R {
        USEMIRROR_IF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
    #[inline(always)]
    #[must_use]
    pub fn loopback_en(&mut self) -> LOOPBACK_EN_W<MDMTEST1_SPEC, 0> {
        LOOPBACK_EN_W::new(self)
    }
    #[doc = "Bit 2 - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_sniff_en(&mut self) -> RFC_SNIFF_EN_W<MDMTEST1_SPEC, 2> {
        RFC_SNIFF_EN_W::new(self)
    }
    #[doc = "Bit 3 - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
    #[inline(always)]
    #[must_use]
    pub fn ramp_amp(&mut self) -> RAMP_AMP_W<MDMTEST1_SPEC, 3> {
        RAMP_AMP_W::new(self)
    }
    #[doc = "Bit 4 - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
    #[inline(always)]
    #[must_use]
    pub fn mod_if(&mut self) -> MOD_IF_W<MDMTEST1_SPEC, 4> {
        MOD_IF_W::new(self)
    }
    #[doc = "Bit 5 - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
    #[inline(always)]
    #[must_use]
    pub fn usemirror_if(&mut self) -> USEMIRROR_IF_W<MDMTEST1_SPEC, 5> {
        USEMIRROR_IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Test Register for Modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdmtest1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdmtest1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMTEST1_SPEC;
impl crate::RegisterSpec for MDMTEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdmtest1::R`](R) reader structure"]
impl crate::Readable for MDMTEST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdmtest1::W`](W) writer structure"]
impl crate::Writable for MDMTEST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMTEST1 to value 0"]
impl crate::Resettable for MDMTEST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

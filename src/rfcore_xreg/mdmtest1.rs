#[doc = "Register `MDMTEST1` reader"]
pub struct R(crate::R<MDMTEST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMTEST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMTEST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMTEST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMTEST1` writer"]
pub struct W(crate::W<MDMTEST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMTEST1_SPEC>;
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
impl From<crate::W<MDMTEST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMTEST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOPBACK_EN` reader - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
pub type LOOPBACK_EN_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK_EN` writer - Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
pub type LOOPBACK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMTEST1_SPEC, bool, O>;
#[doc = "Field `RFC_SNIFF_EN` reader - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
pub type RFC_SNIFF_EN_R = crate::BitReader<bool>;
#[doc = "Field `RFC_SNIFF_EN` writer - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
pub type RFC_SNIFF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMTEST1_SPEC, bool, O>;
#[doc = "Field `RAMP_AMP` reader - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
pub type RAMP_AMP_R = crate::BitReader<bool>;
#[doc = "Field `RAMP_AMP` writer - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
pub type RAMP_AMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMTEST1_SPEC, bool, O>;
#[doc = "Field `MOD_IF` reader - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
pub type MOD_IF_R = crate::BitReader<bool>;
#[doc = "Field `MOD_IF` writer - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
pub type MOD_IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMTEST1_SPEC, bool, O>;
#[doc = "Field `USEMIRROR_IF` reader - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
pub type USEMIRROR_IF_R = crate::BitReader<bool>;
#[doc = "Field `USEMIRROR_IF` writer - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
pub type USEMIRROR_IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMTEST1_SPEC, bool, O>;
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
    pub fn loopback_en(&mut self) -> LOOPBACK_EN_W<0> {
        LOOPBACK_EN_W::new(self)
    }
    #[doc = "Bit 2 - 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
    #[inline(always)]
    pub fn rfc_sniff_en(&mut self) -> RFC_SNIFF_EN_W<2> {
        RFC_SNIFF_EN_W::new(self)
    }
    #[doc = "Bit 3 - 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
    #[inline(always)]
    pub fn ramp_amp(&mut self) -> RAMP_AMP_W<3> {
        RAMP_AMP_W::new(self)
    }
    #[doc = "Bit 4 - 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
    #[inline(always)]
    pub fn mod_if(&mut self) -> MOD_IF_W<4> {
        MOD_IF_W::new(self)
    }
    #[doc = "Bit 5 - 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
    #[inline(always)]
    pub fn usemirror_if(&mut self) -> USEMIRROR_IF_W<5> {
        USEMIRROR_IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register for Modem\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdmtest1](index.html) module"]
pub struct MDMTEST1_SPEC;
impl crate::RegisterSpec for MDMTEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdmtest1::R](R) reader structure"]
impl crate::Readable for MDMTEST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdmtest1::W](W) writer structure"]
impl crate::Writable for MDMTEST1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMTEST1 to value 0"]
impl crate::Resettable for MDMTEST1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

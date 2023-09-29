#[doc = "Register `FREQCTRL` reader"]
pub type R = crate::R<FREQCTRL_SPEC>;
#[doc = "Register `FREQCTRL` writer"]
pub type W = crate::W<FREQCTRL_SPEC>;
#[doc = "Field `FREQ` reader - Frequency control word The frequency word in FREQ\\[6:0\\]
is an offset value from 2394 (fRF = FREQ\\[6 0\\]
+ 2394). The RF-frequency is specified from 2405 to 2480 MHz in 1-MHz steps; hence, the only valid settings for FREQ\\[6:0\\]
are 11 to 86 (11 + 2394 = 2405 and 86 + 2394 = 2480). The device supports the frequency range from 2394 to 2507 MHz. Consequently, the usable settings for FREQ\\[6:0\\]
are 0 to 113. Settings outside of the usable range (114 to 127) give a frequency of 2507 MHz. IEEE 802.15.4-2006 specifies a frequency range from 2405 MHz to 2480 MHz with 16 channels 5 MHz apart. The channels are numbered 11 through 26. For an IEEE 802.15.4-2006 compliant system, the only valid settings are thus FREQ\\[6:0\\]
= 11 + 5 (channel number - 11)."]
pub type FREQ_R = crate::FieldReader;
#[doc = "Field `FREQ` writer - Frequency control word The frequency word in FREQ\\[6:0\\]
is an offset value from 2394 (fRF = FREQ\\[6 0\\]
+ 2394). The RF-frequency is specified from 2405 to 2480 MHz in 1-MHz steps; hence, the only valid settings for FREQ\\[6:0\\]
are 11 to 86 (11 + 2394 = 2405 and 86 + 2394 = 2480). The device supports the frequency range from 2394 to 2507 MHz. Consequently, the usable settings for FREQ\\[6:0\\]
are 0 to 113. Settings outside of the usable range (114 to 127) give a frequency of 2507 MHz. IEEE 802.15.4-2006 specifies a frequency range from 2405 MHz to 2480 MHz with 16 channels 5 MHz apart. The channels are numbered 11 through 26. For an IEEE 802.15.4-2006 compliant system, the only valid settings are thus FREQ\\[6:0\\]
= 11 + 5 (channel number - 11)."]
pub type FREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Frequency control word The frequency word in FREQ\\[6:0\\]
is an offset value from 2394 (fRF = FREQ\\[6 0\\]
+ 2394). The RF-frequency is specified from 2405 to 2480 MHz in 1-MHz steps; hence, the only valid settings for FREQ\\[6:0\\]
are 11 to 86 (11 + 2394 = 2405 and 86 + 2394 = 2480). The device supports the frequency range from 2394 to 2507 MHz. Consequently, the usable settings for FREQ\\[6:0\\]
are 0 to 113. Settings outside of the usable range (114 to 127) give a frequency of 2507 MHz. IEEE 802.15.4-2006 specifies a frequency range from 2405 MHz to 2480 MHz with 16 channels 5 MHz apart. The channels are numbered 11 through 26. For an IEEE 802.15.4-2006 compliant system, the only valid settings are thus FREQ\\[6:0\\]
= 11 + 5 (channel number - 11)."]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Frequency control word The frequency word in FREQ\\[6:0\\]
is an offset value from 2394 (fRF = FREQ\\[6 0\\]
+ 2394). The RF-frequency is specified from 2405 to 2480 MHz in 1-MHz steps; hence, the only valid settings for FREQ\\[6:0\\]
are 11 to 86 (11 + 2394 = 2405 and 86 + 2394 = 2480). The device supports the frequency range from 2394 to 2507 MHz. Consequently, the usable settings for FREQ\\[6:0\\]
are 0 to 113. Settings outside of the usable range (114 to 127) give a frequency of 2507 MHz. IEEE 802.15.4-2006 specifies a frequency range from 2405 MHz to 2480 MHz with 16 channels 5 MHz apart. The channels are numbered 11 through 26. For an IEEE 802.15.4-2006 compliant system, the only valid settings are thus FREQ\\[6:0\\]
= 11 + 5 (channel number - 11)."]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<FREQCTRL_SPEC, 0> {
        FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Controls the RF frequency\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`freqctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREQCTRL_SPEC;
impl crate::RegisterSpec for FREQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freqctrl::R`](R) reader structure"]
impl crate::Readable for FREQCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`freqctrl::W`](W) writer structure"]
impl crate::Writable for FREQCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQCTRL to value 0"]
impl crate::Resettable for FREQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

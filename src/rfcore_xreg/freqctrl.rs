#[doc = "Register `FREQCTRL` reader"]
pub type R = crate::R<FreqctrlSpec>;
#[doc = "Register `FREQCTRL` writer"]
pub type W = crate::W<FreqctrlSpec>;
#[doc = "Field `FREQ` reader - Frequency control word The frequency word in FREQ\\[6:0\\]
is an offset value from 2394 (fRF = FREQ\\[6 0\\]
+ 2394). The RF-frequency is specified from 2405 to 2480 MHz in 1-MHz steps; hence, the only valid settings for FREQ\\[6:0\\]
are 11 to 86 (11 + 2394 = 2405 and 86 + 2394 = 2480). The device supports the frequency range from 2394 to 2507 MHz. Consequently, the usable settings for FREQ\\[6:0\\]
are 0 to 113. Settings outside of the usable range (114 to 127) give a frequency of 2507 MHz. IEEE 802.15.4-2006 specifies a frequency range from 2405 MHz to 2480 MHz with 16 channels 5 MHz apart. The channels are numbered 11 through 26. For an IEEE 802.15.4-2006 compliant system, the only valid settings are thus FREQ\\[6:0\\]
= 11 + 5 (channel number - 11)."]
pub type FreqR = crate::FieldReader;
#[doc = "Field `FREQ` writer - Frequency control word The frequency word in FREQ\\[6:0\\]
is an offset value from 2394 (fRF = FREQ\\[6 0\\]
+ 2394). The RF-frequency is specified from 2405 to 2480 MHz in 1-MHz steps; hence, the only valid settings for FREQ\\[6:0\\]
are 11 to 86 (11 + 2394 = 2405 and 86 + 2394 = 2480). The device supports the frequency range from 2394 to 2507 MHz. Consequently, the usable settings for FREQ\\[6:0\\]
are 0 to 113. Settings outside of the usable range (114 to 127) give a frequency of 2507 MHz. IEEE 802.15.4-2006 specifies a frequency range from 2405 MHz to 2480 MHz with 16 channels 5 MHz apart. The channels are numbered 11 through 26. For an IEEE 802.15.4-2006 compliant system, the only valid settings are thus FREQ\\[6:0\\]
= 11 + 5 (channel number - 11)."]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Frequency control word The frequency word in FREQ\\[6:0\\]
is an offset value from 2394 (fRF = FREQ\\[6 0\\]
+ 2394). The RF-frequency is specified from 2405 to 2480 MHz in 1-MHz steps; hence, the only valid settings for FREQ\\[6:0\\]
are 11 to 86 (11 + 2394 = 2405 and 86 + 2394 = 2480). The device supports the frequency range from 2394 to 2507 MHz. Consequently, the usable settings for FREQ\\[6:0\\]
are 0 to 113. Settings outside of the usable range (114 to 127) give a frequency of 2507 MHz. IEEE 802.15.4-2006 specifies a frequency range from 2405 MHz to 2480 MHz with 16 channels 5 MHz apart. The channels are numbered 11 through 26. For an IEEE 802.15.4-2006 compliant system, the only valid settings are thus FREQ\\[6:0\\]
= 11 + 5 (channel number - 11)."]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new((self.bits & 0x7f) as u8)
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
    pub fn freq(&mut self) -> FreqW<FreqctrlSpec> {
        FreqW::new(self, 0)
    }
}
#[doc = "Controls the RF frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`freqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqctrlSpec;
impl crate::RegisterSpec for FreqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freqctrl::R`](R) reader structure"]
impl crate::Readable for FreqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`freqctrl::W`](W) writer structure"]
impl crate::Writable for FreqctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQCTRL to value 0"]
impl crate::Resettable for FreqctrlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `FREQCTRL` reader"]
pub struct R(crate::R<FREQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQCTRL` writer"]
pub struct W(crate::W<FREQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQCTRL_SPEC>;
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
impl From<crate::W<FREQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQ` reader - Frequency control word The frequency word in FREQ\\[6:0\\]
is an offset value from 2394 (fRF = FREQ\\[6 0\\]
+ 2394). The RF-frequency is specified from 2405 to 2480 MHz in 1-MHz steps; hence, the only valid settings for FREQ\\[6:0\\]
are 11 to 86 (11 + 2394 = 2405 and 86 + 2394 = 2480). The device supports the frequency range from 2394 to 2507 MHz. Consequently, the usable settings for FREQ\\[6:0\\]
are 0 to 113. Settings outside of the usable range (114 to 127) give a frequency of 2507 MHz. IEEE 802.15.4-2006 specifies a frequency range from 2405 MHz to 2480 MHz with 16 channels 5 MHz apart. The channels are numbered 11 through 26. For an IEEE 802.15.4-2006 compliant system, the only valid settings are thus FREQ\\[6:0\\]
= 11 + 5 (channel number - 11)."]
pub struct FREQ_R(crate::FieldReader<u8, u8>);
impl FREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQ` writer - Frequency control word The frequency word in FREQ\\[6:0\\]
is an offset value from 2394 (fRF = FREQ\\[6 0\\]
+ 2394). The RF-frequency is specified from 2405 to 2480 MHz in 1-MHz steps; hence, the only valid settings for FREQ\\[6:0\\]
are 11 to 86 (11 + 2394 = 2405 and 86 + 2394 = 2480). The device supports the frequency range from 2394 to 2507 MHz. Consequently, the usable settings for FREQ\\[6:0\\]
are 0 to 113. Settings outside of the usable range (114 to 127) give a frequency of 2507 MHz. IEEE 802.15.4-2006 specifies a frequency range from 2405 MHz to 2480 MHz with 16 channels 5 MHz apart. The channels are numbered 11 through 26. For an IEEE 802.15.4-2006 compliant system, the only valid settings are thus FREQ\\[6:0\\]
= 11 + 5 (channel number - 11)."]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
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
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the RF frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqctrl](index.html) module"]
pub struct FREQCTRL_SPEC;
impl crate::RegisterSpec for FREQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqctrl::R](R) reader structure"]
impl crate::Readable for FREQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqctrl::W](W) writer structure"]
impl crate::Writable for FREQCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQCTRL to value 0"]
impl crate::Resettable for FREQCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

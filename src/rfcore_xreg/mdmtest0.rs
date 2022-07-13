#[doc = "Register `MDMTEST0` reader"]
pub struct R(crate::R<MDMTEST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMTEST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMTEST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMTEST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMTEST0` writer"]
pub struct W(crate::W<MDMTEST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMTEST0_SPEC>;
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
impl From<crate::W<MDMTEST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMTEST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_TONE` reader - Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
pub type TX_TONE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_TONE` writer - Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
pub type TX_TONE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMTEST0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DC_WIN_SIZE` reader - Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
pub type DC_WIN_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DC_WIN_SIZE` writer - Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
pub type DC_WIN_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMTEST0_SPEC, u8, u8, 2, O>;
#[doc = "Field `DC_BLOCK_MODE` reader - Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
pub type DC_BLOCK_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DC_BLOCK_MODE` writer - Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
pub type DC_BLOCK_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MDMTEST0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 4:7 - Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
    #[inline(always)]
    pub fn tx_tone(&self) -> TX_TONE_R {
        TX_TONE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
    #[inline(always)]
    pub fn dc_win_size(&self) -> DC_WIN_SIZE_R {
        DC_WIN_SIZE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
    #[inline(always)]
    pub fn dc_block_mode(&self) -> DC_BLOCK_MODE_R {
        DC_BLOCK_MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
    #[inline(always)]
    pub fn tx_tone(&mut self) -> TX_TONE_W<4> {
        TX_TONE_W::new(self)
    }
    #[doc = "Bits 2:3 - Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
    #[inline(always)]
    pub fn dc_win_size(&mut self) -> DC_WIN_SIZE_W<2> {
        DC_WIN_SIZE_W::new(self)
    }
    #[doc = "Bits 0:1 - Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
    #[inline(always)]
    pub fn dc_block_mode(&mut self) -> DC_BLOCK_MODE_W<0> {
        DC_BLOCK_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test register for modem\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdmtest0](index.html) module"]
pub struct MDMTEST0_SPEC;
impl crate::RegisterSpec for MDMTEST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdmtest0::R](R) reader structure"]
impl crate::Readable for MDMTEST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdmtest0::W](W) writer structure"]
impl crate::Writable for MDMTEST0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMTEST0 to value 0"]
impl crate::Resettable for MDMTEST0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

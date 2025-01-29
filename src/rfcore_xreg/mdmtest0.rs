#[doc = "Register `MDMTEST0` reader"]
pub type R = crate::R<Mdmtest0Spec>;
#[doc = "Register `MDMTEST0` writer"]
pub type W = crate::W<Mdmtest0Spec>;
#[doc = "Field `DC_BLOCK_MODE` reader - Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
pub type DcBlockModeR = crate::FieldReader;
#[doc = "Field `DC_BLOCK_MODE` writer - Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
pub type DcBlockModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DC_WIN_SIZE` reader - Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
pub type DcWinSizeR = crate::FieldReader;
#[doc = "Field `DC_WIN_SIZE` writer - Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
pub type DcWinSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_TONE` reader - Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
pub type TxToneR = crate::FieldReader;
#[doc = "Field `TX_TONE` writer - Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
pub type TxToneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
    #[inline(always)]
    pub fn dc_block_mode(&self) -> DcBlockModeR {
        DcBlockModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
    #[inline(always)]
    pub fn dc_win_size(&self) -> DcWinSizeR {
        DcWinSizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
    #[inline(always)]
    pub fn tx_tone(&self) -> TxToneR {
        TxToneR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
    #[inline(always)]
    pub fn dc_block_mode(&mut self) -> DcBlockModeW<Mdmtest0Spec> {
        DcBlockModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
    #[inline(always)]
    pub fn dc_win_size(&mut self) -> DcWinSizeW<Mdmtest0Spec> {
        DcWinSizeW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
    #[inline(always)]
    pub fn tx_tone(&mut self) -> TxToneW<Mdmtest0Spec> {
        TxToneW::new(self, 4)
    }
}
#[doc = "Test register for modem\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmtest0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmtest0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdmtest0Spec;
impl crate::RegisterSpec for Mdmtest0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdmtest0::R`](R) reader structure"]
impl crate::Readable for Mdmtest0Spec {}
#[doc = "`write(|w| ..)` method takes [`mdmtest0::W`](W) writer structure"]
impl crate::Writable for Mdmtest0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMTEST0 to value 0"]
impl crate::Resettable for Mdmtest0Spec {
    const RESET_VALUE: u32 = 0;
}

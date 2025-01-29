#[doc = "Register `MDMCTRL0` reader"]
pub type R = crate::R<Mdmctrl0Spec>;
#[doc = "Register `MDMCTRL0` writer"]
pub type W = crate::W<Mdmctrl0Spec>;
#[doc = "Field `TX_FILTER` reader - Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
pub type TxFilterR = crate::BitReader;
#[doc = "Field `TX_FILTER` writer - Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
pub type TxFilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREAMBLE_LENGTH` reader - The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
pub type PreambleLengthR = crate::FieldReader;
#[doc = "Field `PREAMBLE_LENGTH` writer - The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
pub type PreambleLengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEMOD_AVG_MODE` reader - Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
pub type DemodAvgModeR = crate::BitReader;
#[doc = "Field `DEMOD_AVG_MODE` writer - Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
pub type DemodAvgModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEM_NUM_ZEROS` reader - Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
pub type DemNumZerosR = crate::FieldReader;
#[doc = "Field `DEM_NUM_ZEROS` writer - Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
pub type DemNumZerosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
    #[inline(always)]
    pub fn tx_filter(&self) -> TxFilterR {
        TxFilterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
    #[inline(always)]
    pub fn preamble_length(&self) -> PreambleLengthR {
        PreambleLengthR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
    #[inline(always)]
    pub fn demod_avg_mode(&self) -> DemodAvgModeR {
        DemodAvgModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
    #[inline(always)]
    pub fn dem_num_zeros(&self) -> DemNumZerosR {
        DemNumZerosR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
    #[inline(always)]
    pub fn tx_filter(&mut self) -> TxFilterW<Mdmctrl0Spec> {
        TxFilterW::new(self, 0)
    }
    #[doc = "Bits 1:4 - The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
    #[inline(always)]
    pub fn preamble_length(&mut self) -> PreambleLengthW<Mdmctrl0Spec> {
        PreambleLengthW::new(self, 1)
    }
    #[doc = "Bit 5 - Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
    #[inline(always)]
    pub fn demod_avg_mode(&mut self) -> DemodAvgModeW<Mdmctrl0Spec> {
        DemodAvgModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
    #[inline(always)]
    pub fn dem_num_zeros(&mut self) -> DemNumZerosW<Mdmctrl0Spec> {
        DemNumZerosW::new(self, 6)
    }
}
#[doc = "Controls modem\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mdmctrl0Spec;
impl crate::RegisterSpec for Mdmctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdmctrl0::R`](R) reader structure"]
impl crate::Readable for Mdmctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mdmctrl0::W`](W) writer structure"]
impl crate::Writable for Mdmctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMCTRL0 to value 0"]
impl crate::Resettable for Mdmctrl0Spec {
    const RESET_VALUE: u32 = 0;
}

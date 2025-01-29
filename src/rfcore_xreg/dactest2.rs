#[doc = "Register `DACTEST2` reader"]
pub type R = crate::R<Dactest2Spec>;
#[doc = "Register `DACTEST2` writer"]
pub type W = crate::W<Dactest2Spec>;
#[doc = "Field `DAC_SRC` reader - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
pub type DacSrcR = crate::FieldReader;
#[doc = "Field `DAC_SRC` writer - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
pub type DacSrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DAC_CASC_CTRL` reader - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
pub type DacCascCtrlR = crate::FieldReader;
#[doc = "Field `DAC_CASC_CTRL` writer - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
pub type DacCascCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAC_DEM_EN` reader - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
pub type DacDemEnR = crate::BitReader;
#[doc = "Field `DAC_DEM_EN` writer - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
pub type DacDemEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    pub fn dac_src(&self) -> DacSrcR {
        DacSrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    pub fn dac_casc_ctrl(&self) -> DacCascCtrlR {
        DacCascCtrlR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    pub fn dac_dem_en(&self) -> DacDemEnR {
        DacDemEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    pub fn dac_src(&mut self) -> DacSrcW<Dactest2Spec> {
        DacSrcW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    pub fn dac_casc_ctrl(&mut self) -> DacCascCtrlW<Dactest2Spec> {
        DacCascCtrlW::new(self, 3)
    }
    #[doc = "Bit 5 - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    pub fn dac_dem_en(&mut self) -> DacDemEnW<Dactest2Spec> {
        DacDemEnW::new(self, 5)
    }
}
#[doc = "DAC test setting\n\nYou can [`read`](crate::Reg::read) this register and get [`dactest2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dactest2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dactest2Spec;
impl crate::RegisterSpec for Dactest2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dactest2::R`](R) reader structure"]
impl crate::Readable for Dactest2Spec {}
#[doc = "`write(|w| ..)` method takes [`dactest2::W`](W) writer structure"]
impl crate::Writable for Dactest2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACTEST2 to value 0"]
impl crate::Resettable for Dactest2Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DACTEST2` reader"]
pub type R = crate::R<DACTEST2_SPEC>;
#[doc = "Register `DACTEST2` writer"]
pub type W = crate::W<DACTEST2_SPEC>;
#[doc = "Field `DAC_SRC` reader - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
pub type DAC_SRC_R = crate::FieldReader;
#[doc = "Field `DAC_SRC` writer - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
pub type DAC_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DAC_CASC_CTRL` reader - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
pub type DAC_CASC_CTRL_R = crate::FieldReader;
#[doc = "Field `DAC_CASC_CTRL` writer - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
pub type DAC_CASC_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAC_DEM_EN` reader - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
pub type DAC_DEM_EN_R = crate::BitReader;
#[doc = "Field `DAC_DEM_EN` writer - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
pub type DAC_DEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    pub fn dac_src(&self) -> DAC_SRC_R {
        DAC_SRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    pub fn dac_casc_ctrl(&self) -> DAC_CASC_CTRL_R {
        DAC_CASC_CTRL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    pub fn dac_dem_en(&self) -> DAC_DEM_EN_R {
        DAC_DEM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dac_src(&mut self) -> DAC_SRC_W<DACTEST2_SPEC> {
        DAC_SRC_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn dac_casc_ctrl(&mut self) -> DAC_CASC_CTRL_W<DACTEST2_SPEC> {
        DAC_CASC_CTRL_W::new(self, 3)
    }
    #[doc = "Bit 5 - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn dac_dem_en(&mut self) -> DAC_DEM_EN_W<DACTEST2_SPEC> {
        DAC_DEM_EN_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC test setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dactest2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dactest2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACTEST2_SPEC;
impl crate::RegisterSpec for DACTEST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dactest2::R`](R) reader structure"]
impl crate::Readable for DACTEST2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dactest2::W`](W) writer structure"]
impl crate::Writable for DACTEST2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACTEST2 to value 0"]
impl crate::Resettable for DACTEST2_SPEC {
    const RESET_VALUE: u32 = 0;
}

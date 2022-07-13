#[doc = "Register `DACTEST2` reader"]
pub struct R(crate::R<DACTEST2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACTEST2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACTEST2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACTEST2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACTEST2` writer"]
pub struct W(crate::W<DACTEST2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACTEST2_SPEC>;
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
impl From<crate::W<DACTEST2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACTEST2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_DEM_EN` reader - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
pub type DAC_DEM_EN_R = crate::BitReader<bool>;
#[doc = "Field `DAC_DEM_EN` writer - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
pub type DAC_DEM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACTEST2_SPEC, bool, O>;
#[doc = "Field `DAC_CASC_CTRL` reader - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
pub type DAC_CASC_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_CASC_CTRL` writer - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
pub type DAC_CASC_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DACTEST2_SPEC, u8, u8, 2, O>;
#[doc = "Field `DAC_SRC` reader - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
pub type DAC_SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_SRC` writer - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
pub type DAC_SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACTEST2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 5 - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    pub fn dac_dem_en(&self) -> DAC_DEM_EN_R {
        DAC_DEM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    pub fn dac_casc_ctrl(&self) -> DAC_CASC_CTRL_R {
        DAC_CASC_CTRL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 0:2 - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    pub fn dac_src(&self) -> DAC_SRC_R {
        DAC_SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    pub fn dac_dem_en(&mut self) -> DAC_DEM_EN_W<5> {
        DAC_DEM_EN_W::new(self)
    }
    #[doc = "Bits 3:4 - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    pub fn dac_casc_ctrl(&mut self) -> DAC_CASC_CTRL_W<3> {
        DAC_CASC_CTRL_W::new(self)
    }
    #[doc = "Bits 0:2 - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    pub fn dac_src(&mut self) -> DAC_SRC_W<0> {
        DAC_SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC test setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dactest2](index.html) module"]
pub struct DACTEST2_SPEC;
impl crate::RegisterSpec for DACTEST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dactest2::R](R) reader structure"]
impl crate::Readable for DACTEST2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dactest2::W](W) writer structure"]
impl crate::Writable for DACTEST2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACTEST2 to value 0"]
impl crate::Resettable for DACTEST2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

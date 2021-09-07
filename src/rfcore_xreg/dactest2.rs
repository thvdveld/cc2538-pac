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
pub struct DAC_DEM_EN_R(crate::FieldReader<bool, bool>);
impl DAC_DEM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC_DEM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_DEM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_DEM_EN` writer - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
pub struct DAC_DEM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DEM_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DAC_CASC_CTRL` reader - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
pub struct DAC_CASC_CTRL_R(crate::FieldReader<u8, u8>);
impl DAC_CASC_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAC_CASC_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CASC_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CASC_CTRL` writer - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
pub struct DAC_CASC_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CASC_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `DAC_SRC` reader - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
pub struct DAC_SRC_R(crate::FieldReader<u8, u8>);
impl DAC_SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAC_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_SRC` writer - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
pub struct DAC_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    pub fn dac_dem_en(&self) -> DAC_DEM_EN_R {
        DAC_DEM_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    pub fn dac_casc_ctrl(&self) -> DAC_CASC_CTRL_R {
        DAC_CASC_CTRL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2 - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    pub fn dac_src(&self) -> DAC_SRC_R {
        DAC_SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    pub fn dac_dem_en(&mut self) -> DAC_DEM_EN_W {
        DAC_DEM_EN_W { w: self }
    }
    #[doc = "Bits 3:4 - Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    pub fn dac_casc_ctrl(&mut self) -> DAC_CASC_CTRL_W {
        DAC_CASC_CTRL_W { w: self }
    }
    #[doc = "Bits 0:2 - The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    pub fn dac_src(&mut self) -> DAC_SRC_W {
        DAC_SRC_W { w: self }
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

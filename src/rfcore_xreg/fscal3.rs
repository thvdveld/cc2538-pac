#[doc = "Register `FSCAL3` reader"]
pub struct R(crate::R<FSCAL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCAL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCAL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCAL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSCAL3` writer"]
pub struct W(crate::W<FSCAL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSCAL3_SPEC>;
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
impl From<crate::W<FSCAL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSCAL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCO_DAC_EN_OV` reader - Enables the VCO DAC when 1"]
pub struct VCO_DAC_EN_OV_R(crate::FieldReader<bool, bool>);
impl VCO_DAC_EN_OV_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCO_DAC_EN_OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_DAC_EN_OV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_DAC_EN_OV` writer - Enables the VCO DAC when 1"]
pub struct VCO_DAC_EN_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_DAC_EN_OV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `VCO_VC_DAC` reader - Bit vector for programming varactor control voltage from VC DAC"]
pub struct VCO_VC_DAC_R(crate::FieldReader<u8, u8>);
impl VCO_VC_DAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        VCO_VC_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_VC_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_VC_DAC` writer - Bit vector for programming varactor control voltage from VC DAC"]
pub struct VCO_VC_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_VC_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `VCO_CAPARR_CAL_CTRL` reader - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
pub struct VCO_CAPARR_CAL_CTRL_R(crate::FieldReader<u8, u8>);
impl VCO_CAPARR_CAL_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VCO_CAPARR_CAL_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_CAPARR_CAL_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_CAPARR_CAL_CTRL` writer - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
pub struct VCO_CAPARR_CAL_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CAPARR_CAL_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Enables the VCO DAC when 1"]
    #[inline(always)]
    pub fn vco_dac_en_ov(&self) -> VCO_DAC_EN_OV_R {
        VCO_DAC_EN_OV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Bit vector for programming varactor control voltage from VC DAC"]
    #[inline(always)]
    pub fn vco_vc_dac(&self) -> VCO_VC_DAC_R {
        VCO_VC_DAC_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
    #[inline(always)]
    pub fn vco_caparr_cal_ctrl(&self) -> VCO_CAPARR_CAL_CTRL_R {
        VCO_CAPARR_CAL_CTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - Enables the VCO DAC when 1"]
    #[inline(always)]
    pub fn vco_dac_en_ov(&mut self) -> VCO_DAC_EN_OV_W {
        VCO_DAC_EN_OV_W { w: self }
    }
    #[doc = "Bits 2:5 - Bit vector for programming varactor control voltage from VC DAC"]
    #[inline(always)]
    pub fn vco_vc_dac(&mut self) -> VCO_VC_DAC_W {
        VCO_VC_DAC_W { w: self }
    }
    #[doc = "Bits 0:1 - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
    #[inline(always)]
    pub fn vco_caparr_cal_ctrl(&mut self) -> VCO_CAPARR_CAL_CTRL_W {
        VCO_CAPARR_CAL_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tune frequency calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscal3](index.html) module"]
pub struct FSCAL3_SPEC;
impl crate::RegisterSpec for FSCAL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fscal3::R](R) reader structure"]
impl crate::Readable for FSCAL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fscal3::W](W) writer structure"]
impl crate::Writable for FSCAL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSCAL3 to value 0"]
impl crate::Resettable for FSCAL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `FSCAL0` reader"]
pub struct R(crate::R<FSCAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSCAL0` writer"]
pub struct W(crate::W<FSCAL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSCAL0_SPEC>;
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
impl From<crate::W<FSCAL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSCAL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCO_CURR_COMP_EN_OV` reader - Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
pub struct VCO_CURR_COMP_EN_OV_R(crate::FieldReader<bool, bool>);
impl VCO_CURR_COMP_EN_OV_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCO_CURR_COMP_EN_OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_CURR_COMP_EN_OV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCO_CURR_COMP_EN_OV` writer - Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
pub struct VCO_CURR_COMP_EN_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CURR_COMP_EN_OV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CHP_DISABLE` reader - Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
pub struct CHP_DISABLE_R(crate::FieldReader<bool, bool>);
impl CHP_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHP_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHP_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHP_DISABLE` writer - Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
pub struct CHP_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP_DISABLE_W<'a> {
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
#[doc = "Field `CHP_CURRENT` reader - Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
pub struct CHP_CURRENT_R(crate::FieldReader<u8, u8>);
impl CHP_CURRENT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHP_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHP_CURRENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHP_CURRENT` writer - Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
pub struct CHP_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `BW_BOOST_MODE` reader - Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
pub struct BW_BOOST_MODE_R(crate::FieldReader<u8, u8>);
impl BW_BOOST_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BW_BOOST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BW_BOOST_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BW_BOOST_MODE` writer - Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
pub struct BW_BOOST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BW_BOOST_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
    #[inline(always)]
    pub fn vco_curr_comp_en_ov(&self) -> VCO_CURR_COMP_EN_OV_R {
        VCO_CURR_COMP_EN_OV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
    #[inline(always)]
    pub fn chp_disable(&self) -> CHP_DISABLE_R {
        CHP_DISABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
    #[inline(always)]
    pub fn chp_current(&self) -> CHP_CURRENT_R {
        CHP_CURRENT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
    #[inline(always)]
    pub fn bw_boost_mode(&self) -> BW_BOOST_MODE_R {
        BW_BOOST_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
    #[inline(always)]
    pub fn vco_curr_comp_en_ov(&mut self) -> VCO_CURR_COMP_EN_OV_W {
        VCO_CURR_COMP_EN_OV_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
    #[inline(always)]
    pub fn chp_disable(&mut self) -> CHP_DISABLE_W {
        CHP_DISABLE_W { w: self }
    }
    #[doc = "Bits 2:5 - Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
    #[inline(always)]
    pub fn chp_current(&mut self) -> CHP_CURRENT_W {
        CHP_CURRENT_W { w: self }
    }
    #[doc = "Bits 0:1 - Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
    #[inline(always)]
    pub fn bw_boost_mode(&mut self) -> BW_BOOST_MODE_W {
        BW_BOOST_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tune frequency calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscal0](index.html) module"]
pub struct FSCAL0_SPEC;
impl crate::RegisterSpec for FSCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fscal0::R](R) reader structure"]
impl crate::Readable for FSCAL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fscal0::W](W) writer structure"]
impl crate::Writable for FSCAL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSCAL0 to value 0"]
impl crate::Resettable for FSCAL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

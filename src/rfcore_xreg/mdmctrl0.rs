#[doc = "Register `MDMCTRL0` reader"]
pub struct R(crate::R<MDMCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMCTRL0` writer"]
pub struct W(crate::W<MDMCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMCTRL0_SPEC>;
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
impl From<crate::W<MDMCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEM_NUM_ZEROS` reader - Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
pub struct DEM_NUM_ZEROS_R(crate::FieldReader<u8, u8>);
impl DEM_NUM_ZEROS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEM_NUM_ZEROS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEM_NUM_ZEROS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEM_NUM_ZEROS` writer - Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
pub struct DEM_NUM_ZEROS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEM_NUM_ZEROS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DEMOD_AVG_MODE` reader - Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
pub struct DEMOD_AVG_MODE_R(crate::FieldReader<bool, bool>);
impl DEMOD_AVG_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEMOD_AVG_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEMOD_AVG_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEMOD_AVG_MODE` writer - Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
pub struct DEMOD_AVG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_AVG_MODE_W<'a> {
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
#[doc = "Field `PREAMBLE_LENGTH` reader - The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
pub struct PREAMBLE_LENGTH_R(crate::FieldReader<u8, u8>);
impl PREAMBLE_LENGTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        PREAMBLE_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREAMBLE_LENGTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREAMBLE_LENGTH` writer - The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
pub struct PREAMBLE_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PREAMBLE_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `TX_FILTER` reader - Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
pub struct TX_FILTER_R(crate::FieldReader<bool, bool>);
impl TX_FILTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FILTER` writer - Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
pub struct TX_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FILTER_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
    #[inline(always)]
    pub fn dem_num_zeros(&self) -> DEM_NUM_ZEROS_R {
        DEM_NUM_ZEROS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
    #[inline(always)]
    pub fn demod_avg_mode(&self) -> DEMOD_AVG_MODE_R {
        DEMOD_AVG_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
    #[inline(always)]
    pub fn preamble_length(&self) -> PREAMBLE_LENGTH_R {
        PREAMBLE_LENGTH_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
    #[inline(always)]
    pub fn tx_filter(&self) -> TX_FILTER_R {
        TX_FILTER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
    #[inline(always)]
    pub fn dem_num_zeros(&mut self) -> DEM_NUM_ZEROS_W {
        DEM_NUM_ZEROS_W { w: self }
    }
    #[doc = "Bit 5 - Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
    #[inline(always)]
    pub fn demod_avg_mode(&mut self) -> DEMOD_AVG_MODE_W {
        DEMOD_AVG_MODE_W { w: self }
    }
    #[doc = "Bits 1:4 - The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
    #[inline(always)]
    pub fn preamble_length(&mut self) -> PREAMBLE_LENGTH_W {
        PREAMBLE_LENGTH_W { w: self }
    }
    #[doc = "Bit 0 - Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
    #[inline(always)]
    pub fn tx_filter(&mut self) -> TX_FILTER_W {
        TX_FILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls modem\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdmctrl0](index.html) module"]
pub struct MDMCTRL0_SPEC;
impl crate::RegisterSpec for MDMCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdmctrl0::R](R) reader structure"]
impl crate::Readable for MDMCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdmctrl0::W](W) writer structure"]
impl crate::Writable for MDMCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMCTRL0 to value 0"]
impl crate::Resettable for MDMCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

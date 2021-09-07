#[doc = "Register `FRMFILT1` reader"]
pub struct R(crate::R<FRMFILT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRMFILT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRMFILT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRMFILT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRMFILT1` writer"]
pub struct W(crate::W<FRMFILT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRMFILT1_SPEC>;
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
impl From<crate::W<FRMFILT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRMFILT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACCEPT_FT_3_MAC_CMD` reader - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
pub struct ACCEPT_FT_3_MAC_CMD_R(crate::FieldReader<bool, bool>);
impl ACCEPT_FT_3_MAC_CMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCEPT_FT_3_MAC_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCEPT_FT_3_MAC_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCEPT_FT_3_MAC_CMD` writer - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
pub struct ACCEPT_FT_3_MAC_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEPT_FT_3_MAC_CMD_W<'a> {
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
#[doc = "Field `ACCEPT_FT_2_ACK` reader - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
pub struct ACCEPT_FT_2_ACK_R(crate::FieldReader<bool, bool>);
impl ACCEPT_FT_2_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCEPT_FT_2_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCEPT_FT_2_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCEPT_FT_2_ACK` writer - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
pub struct ACCEPT_FT_2_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEPT_FT_2_ACK_W<'a> {
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
#[doc = "Field `ACCEPT_FT_1_DATA` reader - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
pub struct ACCEPT_FT_1_DATA_R(crate::FieldReader<bool, bool>);
impl ACCEPT_FT_1_DATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCEPT_FT_1_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCEPT_FT_1_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCEPT_FT_1_DATA` writer - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
pub struct ACCEPT_FT_1_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEPT_FT_1_DATA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ACCEPT_FT_0_BEACON` reader - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
pub struct ACCEPT_FT_0_BEACON_R(crate::FieldReader<bool, bool>);
impl ACCEPT_FT_0_BEACON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCEPT_FT_0_BEACON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCEPT_FT_0_BEACON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCEPT_FT_0_BEACON` writer - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
pub struct ACCEPT_FT_0_BEACON_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEPT_FT_0_BEACON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `MODIFY_FT_FILTER` reader - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
pub struct MODIFY_FT_FILTER_R(crate::FieldReader<u8, u8>);
impl MODIFY_FT_FILTER_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODIFY_FT_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODIFY_FT_FILTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODIFY_FT_FILTER` writer - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
pub struct MODIFY_FT_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MODIFY_FT_FILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_3_mac_cmd(&self) -> ACCEPT_FT_3_MAC_CMD_R {
        ACCEPT_FT_3_MAC_CMD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_2_ack(&self) -> ACCEPT_FT_2_ACK_R {
        ACCEPT_FT_2_ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_1_data(&self) -> ACCEPT_FT_1_DATA_R {
        ACCEPT_FT_1_DATA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_0_beacon(&self) -> ACCEPT_FT_0_BEACON_R {
        ACCEPT_FT_0_BEACON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
    #[inline(always)]
    pub fn modify_ft_filter(&self) -> MODIFY_FT_FILTER_R {
        MODIFY_FT_FILTER_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_3_mac_cmd(&mut self) -> ACCEPT_FT_3_MAC_CMD_W {
        ACCEPT_FT_3_MAC_CMD_W { w: self }
    }
    #[doc = "Bit 5 - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_2_ack(&mut self) -> ACCEPT_FT_2_ACK_W {
        ACCEPT_FT_2_ACK_W { w: self }
    }
    #[doc = "Bit 4 - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_1_data(&mut self) -> ACCEPT_FT_1_DATA_W {
        ACCEPT_FT_1_DATA_W { w: self }
    }
    #[doc = "Bit 3 - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_0_beacon(&mut self) -> ACCEPT_FT_0_BEACON_W {
        ACCEPT_FT_0_BEACON_W { w: self }
    }
    #[doc = "Bits 1:2 - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
    #[inline(always)]
    pub fn modify_ft_filter(&mut self) -> MODIFY_FT_FILTER_W {
        MODIFY_FT_FILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmfilt1](index.html) module"]
pub struct FRMFILT1_SPEC;
impl crate::RegisterSpec for FRMFILT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frmfilt1::R](R) reader structure"]
impl crate::Readable for FRMFILT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frmfilt1::W](W) writer structure"]
impl crate::Writable for FRMFILT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRMFILT1 to value 0"]
impl crate::Resettable for FRMFILT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

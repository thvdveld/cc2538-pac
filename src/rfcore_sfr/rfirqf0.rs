#[doc = "Register `RFIRQF0` reader"]
pub struct R(crate::R<RFIRQF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIRQF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIRQF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIRQF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIRQF0` writer"]
pub struct W(crate::W<RFIRQF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIRQF0_SPEC>;
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
impl From<crate::W<RFIRQF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIRQF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXMASKZERO` reader - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXMASKZERO_R(crate::FieldReader<bool, bool>);
impl RXMASKZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXMASKZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMASKZERO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMASKZERO` writer - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXMASKZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMASKZERO_W<'a> {
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
#[doc = "Field `RXPKTDONE` reader - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXPKTDONE_R(crate::FieldReader<bool, bool>);
impl RXPKTDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXPKTDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPKTDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPKTDONE` writer - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXPKTDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPKTDONE_W<'a> {
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
#[doc = "Field `FRAME_ACCEPTED` reader - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
pub struct FRAME_ACCEPTED_R(crate::FieldReader<bool, bool>);
impl FRAME_ACCEPTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_ACCEPTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_ACCEPTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_ACCEPTED` writer - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
pub struct FRAME_ACCEPTED_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_ACCEPTED_W<'a> {
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
#[doc = "Field `SRC_MATCH_FOUND` reader - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
pub struct SRC_MATCH_FOUND_R(crate::FieldReader<bool, bool>);
impl SRC_MATCH_FOUND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRC_MATCH_FOUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_MATCH_FOUND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_MATCH_FOUND` writer - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
pub struct SRC_MATCH_FOUND_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MATCH_FOUND_W<'a> {
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
#[doc = "Field `SRC_MATCH_DONE` reader - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
pub struct SRC_MATCH_DONE_R(crate::FieldReader<bool, bool>);
impl SRC_MATCH_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRC_MATCH_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_MATCH_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_MATCH_DONE` writer - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
pub struct SRC_MATCH_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MATCH_DONE_W<'a> {
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
#[doc = "Field `FIFOP` reader - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
pub struct FIFOP_R(crate::FieldReader<bool, bool>);
impl FIFOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOP` writer - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
pub struct FIFOP_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SFD` reader - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub struct SFD_R(crate::FieldReader<bool, bool>);
impl SFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFD` writer - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub struct SFD_W<'a> {
    w: &'a mut W,
}
impl<'a> SFD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ACT_UNUSED` reader - Reserved 0: No interrupt pending 1: Interrupt pending"]
pub struct ACT_UNUSED_R(crate::FieldReader<bool, bool>);
impl ACT_UNUSED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACT_UNUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_UNUSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_UNUSED` writer - Reserved 0: No interrupt pending 1: Interrupt pending"]
pub struct ACT_UNUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_UNUSED_W<'a> {
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
    #[doc = "Bit 7 - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxmaskzero(&self) -> RXMASKZERO_R {
        RXMASKZERO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxpktdone(&self) -> RXPKTDONE_R {
        RXPKTDONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn frame_accepted(&self) -> FRAME_ACCEPTED_R {
        FRAME_ACCEPTED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_found(&self) -> SRC_MATCH_FOUND_R {
        SRC_MATCH_FOUND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_done(&self) -> SRC_MATCH_DONE_R {
        SRC_MATCH_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn fifop(&self) -> FIFOP_R {
        FIFOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Reserved 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn act_unused(&self) -> ACT_UNUSED_R {
        ACT_UNUSED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxmaskzero(&mut self) -> RXMASKZERO_W {
        RXMASKZERO_W { w: self }
    }
    #[doc = "Bit 6 - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxpktdone(&mut self) -> RXPKTDONE_W {
        RXPKTDONE_W { w: self }
    }
    #[doc = "Bit 5 - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn frame_accepted(&mut self) -> FRAME_ACCEPTED_W {
        FRAME_ACCEPTED_W { w: self }
    }
    #[doc = "Bit 4 - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_found(&mut self) -> SRC_MATCH_FOUND_W {
        SRC_MATCH_FOUND_W { w: self }
    }
    #[doc = "Bit 3 - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_done(&mut self) -> SRC_MATCH_DONE_W {
        SRC_MATCH_DONE_W { w: self }
    }
    #[doc = "Bit 2 - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn fifop(&mut self) -> FIFOP_W {
        FIFOP_W { w: self }
    }
    #[doc = "Bit 1 - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SFD_W {
        SFD_W { w: self }
    }
    #[doc = "Bit 0 - Reserved 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn act_unused(&mut self) -> ACT_UNUSED_W {
        ACT_UNUSED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfirqf0](index.html) module"]
pub struct RFIRQF0_SPEC;
impl crate::RegisterSpec for RFIRQF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfirqf0::R](R) reader structure"]
impl crate::Readable for RFIRQF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfirqf0::W](W) writer structure"]
impl crate::Writable for RFIRQF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFIRQF0 to value 0"]
impl crate::Resettable for RFIRQF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

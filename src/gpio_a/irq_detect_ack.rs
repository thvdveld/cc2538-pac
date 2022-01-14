#[doc = "Register `IRQ_DETECT_ACK` reader"]
pub struct R(crate::R<IRQ_DETECT_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_DETECT_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_DETECT_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_DETECT_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_DETECT_ACK` writer"]
pub struct W(crate::W<IRQ_DETECT_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_DETECT_ACK_SPEC>;
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
impl From<crate::W<IRQ_DETECT_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_DETECT_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIACK7` reader - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK7_R(crate::FieldReader<bool, bool>);
impl PDIACK7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIACK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIACK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIACK7` writer - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `PDIACK6` reader - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK6_R(crate::FieldReader<bool, bool>);
impl PDIACK6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIACK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIACK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIACK6` writer - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PDIACK5` reader - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK5_R(crate::FieldReader<bool, bool>);
impl PDIACK5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIACK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIACK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIACK5` writer - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PDIACK4` reader - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK4_R(crate::FieldReader<bool, bool>);
impl PDIACK4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIACK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIACK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIACK4` writer - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PDIACK3` reader - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK3_R(crate::FieldReader<bool, bool>);
impl PDIACK3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIACK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIACK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIACK3` writer - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PDIACK2` reader - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK2_R(crate::FieldReader<bool, bool>);
impl PDIACK2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIACK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIACK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIACK2` writer - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PDIACK1` reader - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
pub struct PDIACK1_R(crate::FieldReader<bool, bool>);
impl PDIACK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIACK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIACK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIACK1` writer - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
pub struct PDIACK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PDIACK0` reader - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK0_R(crate::FieldReader<bool, bool>);
impl PDIACK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIACK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIACK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIACK0` writer - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PDIACK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PCIACK7` reader - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK7_R(crate::FieldReader<bool, bool>);
impl PCIACK7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIACK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIACK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIACK7` writer - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PCIACK6` reader - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK6_R(crate::FieldReader<bool, bool>);
impl PCIACK6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIACK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIACK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIACK6` writer - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PCIACK5` reader - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK5_R(crate::FieldReader<bool, bool>);
impl PCIACK5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIACK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIACK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIACK5` writer - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PCIACK4` reader - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK4_R(crate::FieldReader<bool, bool>);
impl PCIACK4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIACK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIACK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIACK4` writer - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PCIACK3` reader - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK3_R(crate::FieldReader<bool, bool>);
impl PCIACK3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIACK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIACK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIACK3` writer - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PCIACK2` reader - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK2_R(crate::FieldReader<bool, bool>);
impl PCIACK2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIACK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIACK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIACK2` writer - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PCIACK1` reader - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK1_R(crate::FieldReader<bool, bool>);
impl PCIACK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIACK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIACK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIACK1` writer - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PCIACK0` reader - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK0_R(crate::FieldReader<bool, bool>);
impl PCIACK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIACK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIACK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIACK0` writer - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PCIACK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PBIACK7` reader - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK7_R(crate::FieldReader<bool, bool>);
impl PBIACK7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIACK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIACK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIACK7` writer - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `PBIACK6` reader - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK6_R(crate::FieldReader<bool, bool>);
impl PBIACK6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIACK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIACK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIACK6` writer - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PBIACK5` reader - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK5_R(crate::FieldReader<bool, bool>);
impl PBIACK5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIACK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIACK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIACK5` writer - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PBIACK4` reader - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK4_R(crate::FieldReader<bool, bool>);
impl PBIACK4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIACK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIACK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIACK4` writer - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PBIACK3` reader - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK3_R(crate::FieldReader<bool, bool>);
impl PBIACK3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIACK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIACK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIACK3` writer - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PBIACK2` reader - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK2_R(crate::FieldReader<bool, bool>);
impl PBIACK2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIACK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIACK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIACK2` writer - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PBIACK1` reader - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK1_R(crate::FieldReader<bool, bool>);
impl PBIACK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIACK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIACK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIACK1` writer - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PBIACK0` reader - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK0_R(crate::FieldReader<bool, bool>);
impl PBIACK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIACK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIACK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIACK0` writer - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PBIACK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PAIACK7` reader - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK7_R(crate::FieldReader<bool, bool>);
impl PAIACK7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIACK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIACK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIACK7` writer - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK7_W<'a> {
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
#[doc = "Field `PAIACK6` reader - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK6_R(crate::FieldReader<bool, bool>);
impl PAIACK6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIACK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIACK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIACK6` writer - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK6_W<'a> {
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
#[doc = "Field `PAIACK5` reader - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK5_R(crate::FieldReader<bool, bool>);
impl PAIACK5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIACK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIACK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIACK5` writer - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK5_W<'a> {
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
#[doc = "Field `PAIACK4` reader - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK4_R(crate::FieldReader<bool, bool>);
impl PAIACK4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIACK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIACK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIACK4` writer - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK4_W<'a> {
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
#[doc = "Field `PAIACK3` reader - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK3_R(crate::FieldReader<bool, bool>);
impl PAIACK3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIACK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIACK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIACK3` writer - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK3_W<'a> {
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
#[doc = "Field `PAIACK2` reader - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK2_R(crate::FieldReader<bool, bool>);
impl PAIACK2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIACK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIACK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIACK2` writer - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK2_W<'a> {
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
#[doc = "Field `PAIACK1` reader - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK1_R(crate::FieldReader<bool, bool>);
impl PAIACK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIACK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIACK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIACK1` writer - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK1_W<'a> {
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
#[doc = "Field `PAIACK0` reader - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK0_R(crate::FieldReader<bool, bool>);
impl PAIACK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIACK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIACK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIACK0` writer - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
pub struct PAIACK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK0_W<'a> {
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
    #[doc = "Bit 31 - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack7(&self) -> PDIACK7_R {
        PDIACK7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack6(&self) -> PDIACK6_R {
        PDIACK6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack5(&self) -> PDIACK5_R {
        PDIACK5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack4(&self) -> PDIACK4_R {
        PDIACK4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack3(&self) -> PDIACK3_R {
        PDIACK3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack2(&self) -> PDIACK2_R {
        PDIACK2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
    #[inline(always)]
    pub fn pdiack1(&self) -> PDIACK1_R {
        PDIACK1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack0(&self) -> PDIACK0_R {
        PDIACK0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack7(&self) -> PCIACK7_R {
        PCIACK7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack6(&self) -> PCIACK6_R {
        PCIACK6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack5(&self) -> PCIACK5_R {
        PCIACK5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack4(&self) -> PCIACK4_R {
        PCIACK4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack3(&self) -> PCIACK3_R {
        PCIACK3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack2(&self) -> PCIACK2_R {
        PCIACK2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack1(&self) -> PCIACK1_R {
        PCIACK1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack0(&self) -> PCIACK0_R {
        PCIACK0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack7(&self) -> PBIACK7_R {
        PBIACK7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack6(&self) -> PBIACK6_R {
        PBIACK6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack5(&self) -> PBIACK5_R {
        PBIACK5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack4(&self) -> PBIACK4_R {
        PBIACK4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack3(&self) -> PBIACK3_R {
        PBIACK3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack2(&self) -> PBIACK2_R {
        PBIACK2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack1(&self) -> PBIACK1_R {
        PBIACK1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack0(&self) -> PBIACK0_R {
        PBIACK0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack7(&self) -> PAIACK7_R {
        PAIACK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack6(&self) -> PAIACK6_R {
        PAIACK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack5(&self) -> PAIACK5_R {
        PAIACK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack4(&self) -> PAIACK4_R {
        PAIACK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack3(&self) -> PAIACK3_R {
        PAIACK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack2(&self) -> PAIACK2_R {
        PAIACK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack1(&self) -> PAIACK1_R {
        PAIACK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack0(&self) -> PAIACK0_R {
        PAIACK0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack7(&mut self) -> PDIACK7_W {
        PDIACK7_W { w: self }
    }
    #[doc = "Bit 30 - Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack6(&mut self) -> PDIACK6_W {
        PDIACK6_W { w: self }
    }
    #[doc = "Bit 29 - Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack5(&mut self) -> PDIACK5_W {
        PDIACK5_W { w: self }
    }
    #[doc = "Bit 28 - Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack4(&mut self) -> PDIACK4_W {
        PDIACK4_W { w: self }
    }
    #[doc = "Bit 27 - Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack3(&mut self) -> PDIACK3_W {
        PDIACK3_W { w: self }
    }
    #[doc = "Bit 26 - Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack2(&mut self) -> PDIACK2_W {
        PDIACK2_W { w: self }
    }
    #[doc = "Bit 25 - Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
    #[inline(always)]
    pub fn pdiack1(&mut self) -> PDIACK1_W {
        PDIACK1_W { w: self }
    }
    #[doc = "Bit 24 - Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack0(&mut self) -> PDIACK0_W {
        PDIACK0_W { w: self }
    }
    #[doc = "Bit 23 - Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack7(&mut self) -> PCIACK7_W {
        PCIACK7_W { w: self }
    }
    #[doc = "Bit 22 - Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack6(&mut self) -> PCIACK6_W {
        PCIACK6_W { w: self }
    }
    #[doc = "Bit 21 - Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack5(&mut self) -> PCIACK5_W {
        PCIACK5_W { w: self }
    }
    #[doc = "Bit 20 - Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack4(&mut self) -> PCIACK4_W {
        PCIACK4_W { w: self }
    }
    #[doc = "Bit 19 - Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack3(&mut self) -> PCIACK3_W {
        PCIACK3_W { w: self }
    }
    #[doc = "Bit 18 - Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack2(&mut self) -> PCIACK2_W {
        PCIACK2_W { w: self }
    }
    #[doc = "Bit 17 - Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack1(&mut self) -> PCIACK1_W {
        PCIACK1_W { w: self }
    }
    #[doc = "Bit 16 - Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack0(&mut self) -> PCIACK0_W {
        PCIACK0_W { w: self }
    }
    #[doc = "Bit 15 - Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack7(&mut self) -> PBIACK7_W {
        PBIACK7_W { w: self }
    }
    #[doc = "Bit 14 - Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack6(&mut self) -> PBIACK6_W {
        PBIACK6_W { w: self }
    }
    #[doc = "Bit 13 - Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack5(&mut self) -> PBIACK5_W {
        PBIACK5_W { w: self }
    }
    #[doc = "Bit 12 - Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack4(&mut self) -> PBIACK4_W {
        PBIACK4_W { w: self }
    }
    #[doc = "Bit 11 - Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack3(&mut self) -> PBIACK3_W {
        PBIACK3_W { w: self }
    }
    #[doc = "Bit 10 - Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack2(&mut self) -> PBIACK2_W {
        PBIACK2_W { w: self }
    }
    #[doc = "Bit 9 - Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack1(&mut self) -> PBIACK1_W {
        PBIACK1_W { w: self }
    }
    #[doc = "Bit 8 - Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack0(&mut self) -> PBIACK0_W {
        PBIACK0_W { w: self }
    }
    #[doc = "Bit 7 - Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack7(&mut self) -> PAIACK7_W {
        PAIACK7_W { w: self }
    }
    #[doc = "Bit 6 - Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack6(&mut self) -> PAIACK6_W {
        PAIACK6_W { w: self }
    }
    #[doc = "Bit 5 - Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack5(&mut self) -> PAIACK5_W {
        PAIACK5_W { w: self }
    }
    #[doc = "Bit 4 - Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack4(&mut self) -> PAIACK4_W {
        PAIACK4_W { w: self }
    }
    #[doc = "Bit 3 - Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack3(&mut self) -> PAIACK3_W {
        PAIACK3_W { w: self }
    }
    #[doc = "Bit 2 - Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack2(&mut self) -> PAIACK2_W {
        PAIACK2_W { w: self }
    }
    #[doc = "Bit 1 - Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack1(&mut self) -> PAIACK1_W {
        PAIACK1_W { w: self }
    }
    #[doc = "Bit 0 - Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack0(&mut self) -> PAIACK0_W {
        PAIACK0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_detect_ack](index.html) module"]
pub struct IRQ_DETECT_ACK_SPEC;
impl crate::RegisterSpec for IRQ_DETECT_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_detect_ack::R](R) reader structure"]
impl crate::Readable for IRQ_DETECT_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_detect_ack::W](W) writer structure"]
impl crate::Writable for IRQ_DETECT_ACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_DETECT_ACK to value 0"]
impl crate::Resettable for IRQ_DETECT_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `PI_IEN` reader"]
pub struct R(crate::R<PI_IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PI_IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PI_IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PI_IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PI_IEN` writer"]
pub struct W(crate::W<PI_IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PI_IEN_SPEC>;
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
impl From<crate::W<PI_IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PI_IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIEN7` reader - Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN7_R(crate::FieldReader<bool, bool>);
impl PDIEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIEN7` writer - Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN7_W<'a> {
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
#[doc = "Field `PDIEN6` reader - Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN6_R(crate::FieldReader<bool, bool>);
impl PDIEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIEN6` writer - Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN6_W<'a> {
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
#[doc = "Field `PDIEN5` reader - Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN5_R(crate::FieldReader<bool, bool>);
impl PDIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIEN5` writer - Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN5_W<'a> {
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
#[doc = "Field `PDIEN4` reader - Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN4_R(crate::FieldReader<bool, bool>);
impl PDIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIEN4` writer - Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN4_W<'a> {
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
#[doc = "Field `PDIEN3` reader - Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN3_R(crate::FieldReader<bool, bool>);
impl PDIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIEN3` writer - Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN3_W<'a> {
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
#[doc = "Field `PDIEN2` reader - Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN2_R(crate::FieldReader<bool, bool>);
impl PDIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIEN2` writer - Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN2_W<'a> {
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
#[doc = "Field `PDIEN1` reader - Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN1_R(crate::FieldReader<bool, bool>);
impl PDIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIEN1` writer - Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN1_W<'a> {
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
#[doc = "Field `PDIEN0` reader - Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN0_R(crate::FieldReader<bool, bool>);
impl PDIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDIEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIEN0` writer - Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PDIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN0_W<'a> {
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
#[doc = "Field `PCIEN7` reader - Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN7_R(crate::FieldReader<bool, bool>);
impl PCIEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCIEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIEN7` writer - Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN7_W<'a> {
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
#[doc = "Field `PCIEN6` reader - Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN6_R(crate::FieldReader<bool, bool>);
impl PCIEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCIEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIEN6` writer - Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN6_W<'a> {
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
#[doc = "Field `PCIEN5` reader - Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN5_R(crate::FieldReader<bool, bool>);
impl PCIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCIEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIEN5` writer - Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN5_W<'a> {
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
#[doc = "Field `PCIEN4` reader - Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN4_R(crate::FieldReader<bool, bool>);
impl PCIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCIEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIEN4` writer - Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN4_W<'a> {
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
#[doc = "Field `PCIEN3` reader - Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN3_R(crate::FieldReader<bool, bool>);
impl PCIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCIEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIEN3` writer - Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN3_W<'a> {
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
#[doc = "Field `PCIEN2` reader - Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN2_R(crate::FieldReader<bool, bool>);
impl PCIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCIEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIEN2` writer - Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN2_W<'a> {
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
#[doc = "Field `PCIEN1` reader - Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN1_R(crate::FieldReader<bool, bool>);
impl PCIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCIEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIEN1` writer - Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN1_W<'a> {
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
#[doc = "Field `PCIEN0` reader - Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN0_R(crate::FieldReader<bool, bool>);
impl PCIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCIEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIEN0` writer - Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PCIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN0_W<'a> {
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
#[doc = "Field `PBIEN7` reader - Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN7_R(crate::FieldReader<bool, bool>);
impl PBIEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBIEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIEN7` writer - Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN7_W<'a> {
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
#[doc = "Field `PBIEN6` reader - Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN6_R(crate::FieldReader<bool, bool>);
impl PBIEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBIEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIEN6` writer - Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN6_W<'a> {
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
#[doc = "Field `PBIEN5` reader - Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN5_R(crate::FieldReader<bool, bool>);
impl PBIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBIEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIEN5` writer - Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN5_W<'a> {
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
#[doc = "Field `PBIEN4` reader - Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN4_R(crate::FieldReader<bool, bool>);
impl PBIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBIEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIEN4` writer - Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN4_W<'a> {
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
#[doc = "Field `PBIEN3` reader - Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN3_R(crate::FieldReader<bool, bool>);
impl PBIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBIEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIEN3` writer - Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN3_W<'a> {
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
#[doc = "Field `PBIEN2` reader - Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN2_R(crate::FieldReader<bool, bool>);
impl PBIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBIEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIEN2` writer - Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN2_W<'a> {
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
#[doc = "Field `PBIEN1` reader - Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN1_R(crate::FieldReader<bool, bool>);
impl PBIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBIEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIEN1` writer - Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN1_W<'a> {
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
#[doc = "Field `PBIEN0` reader - Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN0_R(crate::FieldReader<bool, bool>);
impl PBIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBIEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIEN0` writer - Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PBIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN0_W<'a> {
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
#[doc = "Field `PAIEN7` reader - Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN7_R(crate::FieldReader<bool, bool>);
impl PAIEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAIEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIEN7` writer - Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN7_W<'a> {
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
#[doc = "Field `PAIEN6` reader - Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN6_R(crate::FieldReader<bool, bool>);
impl PAIEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAIEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIEN6` writer - Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN6_W<'a> {
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
#[doc = "Field `PAIEN5` reader - Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN5_R(crate::FieldReader<bool, bool>);
impl PAIEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAIEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIEN5` writer - Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN5_W<'a> {
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
#[doc = "Field `PAIEN4` reader - Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN4_R(crate::FieldReader<bool, bool>);
impl PAIEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAIEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIEN4` writer - Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN4_W<'a> {
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
#[doc = "Field `PAIEN3` reader - Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN3_R(crate::FieldReader<bool, bool>);
impl PAIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAIEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIEN3` writer - Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN3_W<'a> {
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
#[doc = "Field `PAIEN2` reader - Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN2_R(crate::FieldReader<bool, bool>);
impl PAIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAIEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIEN2` writer - Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN2_W<'a> {
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
#[doc = "Field `PAIEN1` reader - Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN1_R(crate::FieldReader<bool, bool>);
impl PAIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAIEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIEN1` writer - Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN1_W<'a> {
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
#[doc = "Field `PAIEN0` reader - Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN0_R(crate::FieldReader<bool, bool>);
impl PAIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAIEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIEN0` writer - Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub struct PAIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN0_W<'a> {
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
    #[doc = "Bit 31 - Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien7(&self) -> PDIEN7_R {
        PDIEN7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien6(&self) -> PDIEN6_R {
        PDIEN6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien5(&self) -> PDIEN5_R {
        PDIEN5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien4(&self) -> PDIEN4_R {
        PDIEN4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien3(&self) -> PDIEN3_R {
        PDIEN3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien2(&self) -> PDIEN2_R {
        PDIEN2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien1(&self) -> PDIEN1_R {
        PDIEN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien0(&self) -> PDIEN0_R {
        PDIEN0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien7(&self) -> PCIEN7_R {
        PCIEN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien6(&self) -> PCIEN6_R {
        PCIEN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien5(&self) -> PCIEN5_R {
        PCIEN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien4(&self) -> PCIEN4_R {
        PCIEN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien3(&self) -> PCIEN3_R {
        PCIEN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien2(&self) -> PCIEN2_R {
        PCIEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien1(&self) -> PCIEN1_R {
        PCIEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien0(&self) -> PCIEN0_R {
        PCIEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien7(&self) -> PBIEN7_R {
        PBIEN7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien6(&self) -> PBIEN6_R {
        PBIEN6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien5(&self) -> PBIEN5_R {
        PBIEN5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien4(&self) -> PBIEN4_R {
        PBIEN4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien3(&self) -> PBIEN3_R {
        PBIEN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien2(&self) -> PBIEN2_R {
        PBIEN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien1(&self) -> PBIEN1_R {
        PBIEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien0(&self) -> PBIEN0_R {
        PBIEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien7(&self) -> PAIEN7_R {
        PAIEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien6(&self) -> PAIEN6_R {
        PAIEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien5(&self) -> PAIEN5_R {
        PAIEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien4(&self) -> PAIEN4_R {
        PAIEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien3(&self) -> PAIEN3_R {
        PAIEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien2(&self) -> PAIEN2_R {
        PAIEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien1(&self) -> PAIEN1_R {
        PAIEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien0(&self) -> PAIEN0_R {
        PAIEN0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien7(&mut self) -> PDIEN7_W {
        PDIEN7_W { w: self }
    }
    #[doc = "Bit 30 - Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien6(&mut self) -> PDIEN6_W {
        PDIEN6_W { w: self }
    }
    #[doc = "Bit 29 - Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien5(&mut self) -> PDIEN5_W {
        PDIEN5_W { w: self }
    }
    #[doc = "Bit 28 - Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien4(&mut self) -> PDIEN4_W {
        PDIEN4_W { w: self }
    }
    #[doc = "Bit 27 - Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien3(&mut self) -> PDIEN3_W {
        PDIEN3_W { w: self }
    }
    #[doc = "Bit 26 - Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien2(&mut self) -> PDIEN2_W {
        PDIEN2_W { w: self }
    }
    #[doc = "Bit 25 - Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien1(&mut self) -> PDIEN1_W {
        PDIEN1_W { w: self }
    }
    #[doc = "Bit 24 - Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien0(&mut self) -> PDIEN0_W {
        PDIEN0_W { w: self }
    }
    #[doc = "Bit 23 - Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien7(&mut self) -> PCIEN7_W {
        PCIEN7_W { w: self }
    }
    #[doc = "Bit 22 - Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien6(&mut self) -> PCIEN6_W {
        PCIEN6_W { w: self }
    }
    #[doc = "Bit 21 - Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien5(&mut self) -> PCIEN5_W {
        PCIEN5_W { w: self }
    }
    #[doc = "Bit 20 - Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien4(&mut self) -> PCIEN4_W {
        PCIEN4_W { w: self }
    }
    #[doc = "Bit 19 - Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien3(&mut self) -> PCIEN3_W {
        PCIEN3_W { w: self }
    }
    #[doc = "Bit 18 - Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien2(&mut self) -> PCIEN2_W {
        PCIEN2_W { w: self }
    }
    #[doc = "Bit 17 - Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien1(&mut self) -> PCIEN1_W {
        PCIEN1_W { w: self }
    }
    #[doc = "Bit 16 - Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien0(&mut self) -> PCIEN0_W {
        PCIEN0_W { w: self }
    }
    #[doc = "Bit 15 - Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien7(&mut self) -> PBIEN7_W {
        PBIEN7_W { w: self }
    }
    #[doc = "Bit 14 - Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien6(&mut self) -> PBIEN6_W {
        PBIEN6_W { w: self }
    }
    #[doc = "Bit 13 - Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien5(&mut self) -> PBIEN5_W {
        PBIEN5_W { w: self }
    }
    #[doc = "Bit 12 - Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien4(&mut self) -> PBIEN4_W {
        PBIEN4_W { w: self }
    }
    #[doc = "Bit 11 - Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien3(&mut self) -> PBIEN3_W {
        PBIEN3_W { w: self }
    }
    #[doc = "Bit 10 - Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien2(&mut self) -> PBIEN2_W {
        PBIEN2_W { w: self }
    }
    #[doc = "Bit 9 - Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien1(&mut self) -> PBIEN1_W {
        PBIEN1_W { w: self }
    }
    #[doc = "Bit 8 - Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien0(&mut self) -> PBIEN0_W {
        PBIEN0_W { w: self }
    }
    #[doc = "Bit 7 - Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien7(&mut self) -> PAIEN7_W {
        PAIEN7_W { w: self }
    }
    #[doc = "Bit 6 - Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien6(&mut self) -> PAIEN6_W {
        PAIEN6_W { w: self }
    }
    #[doc = "Bit 5 - Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien5(&mut self) -> PAIEN5_W {
        PAIEN5_W { w: self }
    }
    #[doc = "Bit 4 - Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien4(&mut self) -> PAIEN4_W {
        PAIEN4_W { w: self }
    }
    #[doc = "Bit 3 - Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien3(&mut self) -> PAIEN3_W {
        PAIEN3_W { w: self }
    }
    #[doc = "Bit 2 - Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien2(&mut self) -> PAIEN2_W {
        PAIEN2_W { w: self }
    }
    #[doc = "Bit 1 - Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien1(&mut self) -> PAIEN1_W {
        PAIEN1_W { w: self }
    }
    #[doc = "Bit 0 - Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien0(&mut self) -> PAIEN0_W {
        PAIEN0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_ien](index.html) module"]
pub struct PI_IEN_SPEC;
impl crate::RegisterSpec for PI_IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pi_ien::R](R) reader structure"]
impl crate::Readable for PI_IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pi_ien::W](W) writer structure"]
impl crate::Writable for PI_IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PI_IEN to value 0"]
impl crate::Resettable for PI_IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

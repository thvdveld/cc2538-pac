#[doc = "Register `P_EDGE_CTRL` reader"]
pub struct R(crate::R<P_EDGE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P_EDGE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P_EDGE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P_EDGE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P_EDGE_CTRL` writer"]
pub struct W(crate::W<P_EDGE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P_EDGE_CTRL_SPEC>;
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
impl From<crate::W<P_EDGE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P_EDGE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIRC7` reader - Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC7_R(crate::FieldReader<bool, bool>);
impl PDIRC7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIRC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIRC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIRC7` writer - Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC7_W<'a> {
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
#[doc = "Field `PDIRC6` reader - Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC6_R(crate::FieldReader<bool, bool>);
impl PDIRC6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIRC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIRC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIRC6` writer - Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC6_W<'a> {
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
#[doc = "Field `PDIRC5` reader - Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC5_R(crate::FieldReader<bool, bool>);
impl PDIRC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIRC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIRC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIRC5` writer - Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC5_W<'a> {
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
#[doc = "Field `PDIRC4` reader - Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC4_R(crate::FieldReader<bool, bool>);
impl PDIRC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIRC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIRC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIRC4` writer - Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC4_W<'a> {
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
#[doc = "Field `PDIRC3` reader - Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC3_R(crate::FieldReader<bool, bool>);
impl PDIRC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIRC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIRC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIRC3` writer - Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC3_W<'a> {
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
#[doc = "Field `PDIRC2` reader - Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC2_R(crate::FieldReader<bool, bool>);
impl PDIRC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIRC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIRC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIRC2` writer - Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC2_W<'a> {
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
#[doc = "Field `PDIRC1` reader - Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC1_R(crate::FieldReader<bool, bool>);
impl PDIRC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIRC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIRC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIRC1` writer - Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC1_W<'a> {
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
#[doc = "Field `PDIRC0` reader - Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC0_R(crate::FieldReader<bool, bool>);
impl PDIRC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDIRC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIRC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIRC0` writer - Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PDIRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC0_W<'a> {
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
#[doc = "Field `PCIRC7` reader - Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC7_R(crate::FieldReader<bool, bool>);
impl PCIRC7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIRC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIRC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIRC7` writer - Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC7_W<'a> {
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
#[doc = "Field `PCIRC6` reader - Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC6_R(crate::FieldReader<bool, bool>);
impl PCIRC6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIRC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIRC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIRC6` writer - Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC6_W<'a> {
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
#[doc = "Field `PCIRC5` reader - Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC5_R(crate::FieldReader<bool, bool>);
impl PCIRC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIRC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIRC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIRC5` writer - Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC5_W<'a> {
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
#[doc = "Field `PCIRC4` reader - Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC4_R(crate::FieldReader<bool, bool>);
impl PCIRC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIRC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIRC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIRC4` writer - Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC4_W<'a> {
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
#[doc = "Field `PCIRC3` reader - Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC3_R(crate::FieldReader<bool, bool>);
impl PCIRC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIRC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIRC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIRC3` writer - Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC3_W<'a> {
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
#[doc = "Field `PCIRC2` reader - Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC2_R(crate::FieldReader<bool, bool>);
impl PCIRC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIRC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIRC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIRC2` writer - Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC2_W<'a> {
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
#[doc = "Field `PCIRC1` reader - Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC1_R(crate::FieldReader<bool, bool>);
impl PCIRC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIRC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIRC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIRC1` writer - Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC1_W<'a> {
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
#[doc = "Field `PCIRC0` reader - Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC0_R(crate::FieldReader<bool, bool>);
impl PCIRC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCIRC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCIRC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCIRC0` writer - Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PCIRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC0_W<'a> {
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
#[doc = "Field `PBIRC7` reader - Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC7_R(crate::FieldReader<bool, bool>);
impl PBIRC7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIRC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIRC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIRC7` writer - Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC7_W<'a> {
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
#[doc = "Field `PBIRC6` reader - Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC6_R(crate::FieldReader<bool, bool>);
impl PBIRC6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIRC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIRC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIRC6` writer - Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC6_W<'a> {
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
#[doc = "Field `PBIRC5` reader - Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC5_R(crate::FieldReader<bool, bool>);
impl PBIRC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIRC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIRC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIRC5` writer - Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC5_W<'a> {
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
#[doc = "Field `PBIRC4` reader - Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC4_R(crate::FieldReader<bool, bool>);
impl PBIRC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIRC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIRC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIRC4` writer - Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC4_W<'a> {
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
#[doc = "Field `PBIRC3` reader - Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC3_R(crate::FieldReader<bool, bool>);
impl PBIRC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIRC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIRC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIRC3` writer - Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC3_W<'a> {
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
#[doc = "Field `PBIRC2` reader - Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC2_R(crate::FieldReader<bool, bool>);
impl PBIRC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIRC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIRC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIRC2` writer - Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC2_W<'a> {
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
#[doc = "Field `PBIRC1` reader - Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC1_R(crate::FieldReader<bool, bool>);
impl PBIRC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIRC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIRC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIRC1` writer - Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC1_W<'a> {
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
#[doc = "Field `PBIRC0` reader - Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC0_R(crate::FieldReader<bool, bool>);
impl PBIRC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PBIRC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIRC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIRC0` writer - Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PBIRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC0_W<'a> {
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
#[doc = "Field `PAIRC7` reader - Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC7_R(crate::FieldReader<bool, bool>);
impl PAIRC7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIRC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIRC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRC7` writer - Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC7_W<'a> {
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
#[doc = "Field `PAIRC6` reader - Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC6_R(crate::FieldReader<bool, bool>);
impl PAIRC6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIRC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIRC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRC6` writer - Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC6_W<'a> {
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
#[doc = "Field `PAIRC5` reader - Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC5_R(crate::FieldReader<bool, bool>);
impl PAIRC5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIRC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIRC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRC5` writer - Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC5_W<'a> {
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
#[doc = "Field `PAIRC4` reader - Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC4_R(crate::FieldReader<bool, bool>);
impl PAIRC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIRC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIRC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRC4` writer - Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC4_W<'a> {
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
#[doc = "Field `PAIRC3` reader - Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC3_R(crate::FieldReader<bool, bool>);
impl PAIRC3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIRC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIRC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRC3` writer - Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC3_W<'a> {
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
#[doc = "Field `PAIRC2` reader - Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC2_R(crate::FieldReader<bool, bool>);
impl PAIRC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIRC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIRC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRC2` writer - Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC2_W<'a> {
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
#[doc = "Field `PAIRC1` reader - Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC1_R(crate::FieldReader<bool, bool>);
impl PAIRC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIRC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIRC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRC1` writer - Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC1_W<'a> {
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
#[doc = "Field `PAIRC0` reader - Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC0_R(crate::FieldReader<bool, bool>);
impl PAIRC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAIRC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAIRC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAIRC0` writer - Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub struct PAIRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC0_W<'a> {
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
    #[doc = "Bit 31 - Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc7(&self) -> PDIRC7_R {
        PDIRC7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc6(&self) -> PDIRC6_R {
        PDIRC6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc5(&self) -> PDIRC5_R {
        PDIRC5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc4(&self) -> PDIRC4_R {
        PDIRC4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc3(&self) -> PDIRC3_R {
        PDIRC3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc2(&self) -> PDIRC2_R {
        PDIRC2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc1(&self) -> PDIRC1_R {
        PDIRC1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc0(&self) -> PDIRC0_R {
        PDIRC0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc7(&self) -> PCIRC7_R {
        PCIRC7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc6(&self) -> PCIRC6_R {
        PCIRC6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc5(&self) -> PCIRC5_R {
        PCIRC5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc4(&self) -> PCIRC4_R {
        PCIRC4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc3(&self) -> PCIRC3_R {
        PCIRC3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc2(&self) -> PCIRC2_R {
        PCIRC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc1(&self) -> PCIRC1_R {
        PCIRC1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc0(&self) -> PCIRC0_R {
        PCIRC0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc7(&self) -> PBIRC7_R {
        PBIRC7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc6(&self) -> PBIRC6_R {
        PBIRC6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc5(&self) -> PBIRC5_R {
        PBIRC5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc4(&self) -> PBIRC4_R {
        PBIRC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc3(&self) -> PBIRC3_R {
        PBIRC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc2(&self) -> PBIRC2_R {
        PBIRC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc1(&self) -> PBIRC1_R {
        PBIRC1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc0(&self) -> PBIRC0_R {
        PBIRC0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc7(&self) -> PAIRC7_R {
        PAIRC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc6(&self) -> PAIRC6_R {
        PAIRC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc5(&self) -> PAIRC5_R {
        PAIRC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc4(&self) -> PAIRC4_R {
        PAIRC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc3(&self) -> PAIRC3_R {
        PAIRC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc2(&self) -> PAIRC2_R {
        PAIRC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc1(&self) -> PAIRC1_R {
        PAIRC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc0(&self) -> PAIRC0_R {
        PAIRC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc7(&mut self) -> PDIRC7_W {
        PDIRC7_W { w: self }
    }
    #[doc = "Bit 30 - Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc6(&mut self) -> PDIRC6_W {
        PDIRC6_W { w: self }
    }
    #[doc = "Bit 29 - Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc5(&mut self) -> PDIRC5_W {
        PDIRC5_W { w: self }
    }
    #[doc = "Bit 28 - Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc4(&mut self) -> PDIRC4_W {
        PDIRC4_W { w: self }
    }
    #[doc = "Bit 27 - Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc3(&mut self) -> PDIRC3_W {
        PDIRC3_W { w: self }
    }
    #[doc = "Bit 26 - Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc2(&mut self) -> PDIRC2_W {
        PDIRC2_W { w: self }
    }
    #[doc = "Bit 25 - Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc1(&mut self) -> PDIRC1_W {
        PDIRC1_W { w: self }
    }
    #[doc = "Bit 24 - Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc0(&mut self) -> PDIRC0_W {
        PDIRC0_W { w: self }
    }
    #[doc = "Bit 23 - Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc7(&mut self) -> PCIRC7_W {
        PCIRC7_W { w: self }
    }
    #[doc = "Bit 22 - Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc6(&mut self) -> PCIRC6_W {
        PCIRC6_W { w: self }
    }
    #[doc = "Bit 21 - Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc5(&mut self) -> PCIRC5_W {
        PCIRC5_W { w: self }
    }
    #[doc = "Bit 20 - Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc4(&mut self) -> PCIRC4_W {
        PCIRC4_W { w: self }
    }
    #[doc = "Bit 19 - Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc3(&mut self) -> PCIRC3_W {
        PCIRC3_W { w: self }
    }
    #[doc = "Bit 18 - Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc2(&mut self) -> PCIRC2_W {
        PCIRC2_W { w: self }
    }
    #[doc = "Bit 17 - Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc1(&mut self) -> PCIRC1_W {
        PCIRC1_W { w: self }
    }
    #[doc = "Bit 16 - Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc0(&mut self) -> PCIRC0_W {
        PCIRC0_W { w: self }
    }
    #[doc = "Bit 15 - Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc7(&mut self) -> PBIRC7_W {
        PBIRC7_W { w: self }
    }
    #[doc = "Bit 14 - Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc6(&mut self) -> PBIRC6_W {
        PBIRC6_W { w: self }
    }
    #[doc = "Bit 13 - Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc5(&mut self) -> PBIRC5_W {
        PBIRC5_W { w: self }
    }
    #[doc = "Bit 12 - Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc4(&mut self) -> PBIRC4_W {
        PBIRC4_W { w: self }
    }
    #[doc = "Bit 11 - Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc3(&mut self) -> PBIRC3_W {
        PBIRC3_W { w: self }
    }
    #[doc = "Bit 10 - Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc2(&mut self) -> PBIRC2_W {
        PBIRC2_W { w: self }
    }
    #[doc = "Bit 9 - Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc1(&mut self) -> PBIRC1_W {
        PBIRC1_W { w: self }
    }
    #[doc = "Bit 8 - Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc0(&mut self) -> PBIRC0_W {
        PBIRC0_W { w: self }
    }
    #[doc = "Bit 7 - Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc7(&mut self) -> PAIRC7_W {
        PAIRC7_W { w: self }
    }
    #[doc = "Bit 6 - Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc6(&mut self) -> PAIRC6_W {
        PAIRC6_W { w: self }
    }
    #[doc = "Bit 5 - Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc5(&mut self) -> PAIRC5_W {
        PAIRC5_W { w: self }
    }
    #[doc = "Bit 4 - Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc4(&mut self) -> PAIRC4_W {
        PAIRC4_W { w: self }
    }
    #[doc = "Bit 3 - Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc3(&mut self) -> PAIRC3_W {
        PAIRC3_W { w: self }
    }
    #[doc = "Bit 2 - Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc2(&mut self) -> PAIRC2_W {
        PAIRC2_W { w: self }
    }
    #[doc = "Bit 1 - Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc1(&mut self) -> PAIRC1_W {
        PAIRC1_W { w: self }
    }
    #[doc = "Bit 0 - Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc0(&mut self) -> PAIRC0_W {
        PAIRC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p_edge_ctrl](index.html) module"]
pub struct P_EDGE_CTRL_SPEC;
impl crate::RegisterSpec for P_EDGE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p_edge_ctrl::R](R) reader structure"]
impl crate::Readable for P_EDGE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p_edge_ctrl::W](W) writer structure"]
impl crate::Writable for P_EDGE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P_EDGE_CTRL to value 0"]
impl crate::Resettable for P_EDGE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

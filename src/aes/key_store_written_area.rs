#[doc = "Register `KEY_STORE_WRITTEN_AREA` reader"]
pub struct R(crate::R<KEY_STORE_WRITTEN_AREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_STORE_WRITTEN_AREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_STORE_WRITTEN_AREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_STORE_WRITTEN_AREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_STORE_WRITTEN_AREA` writer"]
pub struct W(crate::W<KEY_STORE_WRITTEN_AREA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_STORE_WRITTEN_AREA_SPEC>;
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
impl From<crate::W<KEY_STORE_WRITTEN_AREA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_STORE_WRITTEN_AREA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_AREA_WRITTEN7` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN7_R(crate::FieldReader<bool, bool>);
impl RAM_AREA_WRITTEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN7` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN7_W<'a> {
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
#[doc = "Field `RAM_AREA_WRITTEN6` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN6_R(crate::FieldReader<bool, bool>);
impl RAM_AREA_WRITTEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN6` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN6_W<'a> {
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
#[doc = "Field `RAM_AREA_WRITTEN5` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN5_R(crate::FieldReader<bool, bool>);
impl RAM_AREA_WRITTEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN5` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN5_W<'a> {
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
#[doc = "Field `RAM_AREA_WRITTEN4` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN4_R(crate::FieldReader<bool, bool>);
impl RAM_AREA_WRITTEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN4` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN4_W<'a> {
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
#[doc = "Field `RAM_AREA_WRITTEN3` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN3_R(crate::FieldReader<bool, bool>);
impl RAM_AREA_WRITTEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN3` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN3_W<'a> {
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
#[doc = "Field `RAM_AREA_WRITTEN2` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN2_R(crate::FieldReader<bool, bool>);
impl RAM_AREA_WRITTEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN2` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN2_W<'a> {
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
#[doc = "Field `RAM_AREA_WRITTEN1` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN1_R(crate::FieldReader<bool, bool>);
impl RAM_AREA_WRITTEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN1` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN1_W<'a> {
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
#[doc = "Field `RAM_AREA_WRITTEN0` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN0_R(crate::FieldReader<bool, bool>);
impl RAM_AREA_WRITTEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_AREA_WRITTEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_AREA_WRITTEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_AREA_WRITTEN0` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub struct RAM_AREA_WRITTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_WRITTEN0_W<'a> {
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
    #[doc = "Bit 7 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written7(&self) -> RAM_AREA_WRITTEN7_R {
        RAM_AREA_WRITTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written6(&self) -> RAM_AREA_WRITTEN6_R {
        RAM_AREA_WRITTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written5(&self) -> RAM_AREA_WRITTEN5_R {
        RAM_AREA_WRITTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written4(&self) -> RAM_AREA_WRITTEN4_R {
        RAM_AREA_WRITTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written3(&self) -> RAM_AREA_WRITTEN3_R {
        RAM_AREA_WRITTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written2(&self) -> RAM_AREA_WRITTEN2_R {
        RAM_AREA_WRITTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written1(&self) -> RAM_AREA_WRITTEN1_R {
        RAM_AREA_WRITTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written0(&self) -> RAM_AREA_WRITTEN0_R {
        RAM_AREA_WRITTEN0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written7(&mut self) -> RAM_AREA_WRITTEN7_W {
        RAM_AREA_WRITTEN7_W { w: self }
    }
    #[doc = "Bit 6 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written6(&mut self) -> RAM_AREA_WRITTEN6_W {
        RAM_AREA_WRITTEN6_W { w: self }
    }
    #[doc = "Bit 5 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written5(&mut self) -> RAM_AREA_WRITTEN5_W {
        RAM_AREA_WRITTEN5_W { w: self }
    }
    #[doc = "Bit 4 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written4(&mut self) -> RAM_AREA_WRITTEN4_W {
        RAM_AREA_WRITTEN4_W { w: self }
    }
    #[doc = "Bit 3 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written3(&mut self) -> RAM_AREA_WRITTEN3_W {
        RAM_AREA_WRITTEN3_W { w: self }
    }
    #[doc = "Bit 2 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written2(&mut self) -> RAM_AREA_WRITTEN2_W {
        RAM_AREA_WRITTEN2_W { w: self }
    }
    #[doc = "Bit 1 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written1(&mut self) -> RAM_AREA_WRITTEN1_W {
        RAM_AREA_WRITTEN1_W { w: self }
    }
    #[doc = "Bit 0 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written0(&mut self) -> RAM_AREA_WRITTEN0_W {
        RAM_AREA_WRITTEN0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_store_written_area](index.html) module"]
pub struct KEY_STORE_WRITTEN_AREA_SPEC;
impl crate::RegisterSpec for KEY_STORE_WRITTEN_AREA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_store_written_area::R](R) reader structure"]
impl crate::Readable for KEY_STORE_WRITTEN_AREA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_store_written_area::W](W) writer structure"]
impl crate::Writable for KEY_STORE_WRITTEN_AREA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY_STORE_WRITTEN_AREA to value 0"]
impl crate::Resettable for KEY_STORE_WRITTEN_AREA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

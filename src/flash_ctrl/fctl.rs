#[doc = "Register `FCTL` reader"]
pub struct R(crate::R<FCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTL` writer"]
pub struct W(crate::W<FCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTL_SPEC>;
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
impl From<crate::W<FCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPPER_PAGE_ACCESS` reader - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
pub struct UPPER_PAGE_ACCESS_R(crate::FieldReader<bool, bool>);
impl UPPER_PAGE_ACCESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPPER_PAGE_ACCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPPER_PAGE_ACCESS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPPER_PAGE_ACCESS` writer - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
pub struct UPPER_PAGE_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPER_PAGE_ACCESS_W<'a> {
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
#[doc = "Field `SEL_INFO_PAGE` reader - Flash erase or write operation on APB bus must assert this when accessing the information page"]
pub struct SEL_INFO_PAGE_R(crate::FieldReader<bool, bool>);
impl SEL_INFO_PAGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_INFO_PAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_INFO_PAGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_INFO_PAGE` writer - Flash erase or write operation on APB bus must assert this when accessing the information page"]
pub struct SEL_INFO_PAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_INFO_PAGE_W<'a> {
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
#[doc = "Field `BUSY` reader - Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULL` reader - Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
pub struct FULL_R(crate::FieldReader<bool, bool>);
impl FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABORT` reader - Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
pub struct ABORT_R(crate::FieldReader<bool, bool>);
impl ABORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM` reader - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
pub struct CM_R(crate::FieldReader<u8, u8>);
impl CM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM` writer - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `WRITE` reader - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub struct WRITE_R(crate::FieldReader<bool, bool>);
impl WRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE` writer - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
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
#[doc = "Field `ERASE` reader - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub struct ERASE_R(crate::FieldReader<bool, bool>);
impl ERASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERASE` writer - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
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
    #[doc = "Bit 9 - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    pub fn upper_page_access(&self) -> UPPER_PAGE_ACCESS_R {
        UPPER_PAGE_ACCESS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    pub fn sel_info_page(&self) -> SEL_INFO_PAGE_R {
        SEL_INFO_PAGE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    pub fn upper_page_access(&mut self) -> UPPER_PAGE_ACCESS_W {
        UPPER_PAGE_ACCESS_W { w: self }
    }
    #[doc = "Bit 8 - Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    pub fn sel_info_page(&mut self) -> SEL_INFO_PAGE_W {
        SEL_INFO_PAGE_W { w: self }
    }
    #[doc = "Bits 2:3 - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Bit 1 - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 0 - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash control This register provides control and monitoring functions for the flash module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl](index.html) module"]
pub struct FCTL_SPEC;
impl crate::RegisterSpec for FCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fctl::R](R) reader structure"]
impl crate::Readable for FCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctl::W](W) writer structure"]
impl crate::Writable for FCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCTL to value 0"]
impl crate::Resettable for FCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

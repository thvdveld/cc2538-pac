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
pub type UPPER_PAGE_ACCESS_R = crate::BitReader<bool>;
#[doc = "Field `UPPER_PAGE_ACCESS` writer - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
pub type UPPER_PAGE_ACCESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTL_SPEC, bool, O>;
#[doc = "Field `SEL_INFO_PAGE` reader - Flash erase or write operation on APB bus must assert this when accessing the information page"]
pub type SEL_INFO_PAGE_R = crate::BitReader<bool>;
#[doc = "Field `SEL_INFO_PAGE` writer - Flash erase or write operation on APB bus must assert this when accessing the information page"]
pub type SEL_INFO_PAGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTL_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `FULL` reader - Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
pub type FULL_R = crate::BitReader<bool>;
#[doc = "Field `ABORT` reader - Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
pub type ABORT_R = crate::BitReader<bool>;
#[doc = "Field `CM` reader - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
pub type CM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CM` writer - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `WRITE` reader - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type WRITE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE` writer - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTL_SPEC, bool, O>;
#[doc = "Field `ERASE` reader - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type ERASE_R = crate::BitReader<bool>;
#[doc = "Field `ERASE` writer - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    pub fn upper_page_access(&self) -> UPPER_PAGE_ACCESS_R {
        UPPER_PAGE_ACCESS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    pub fn sel_info_page(&self) -> SEL_INFO_PAGE_R {
        SEL_INFO_PAGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 1 - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    pub fn upper_page_access(&mut self) -> UPPER_PAGE_ACCESS_W<9> {
        UPPER_PAGE_ACCESS_W::new(self)
    }
    #[doc = "Bit 8 - Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    pub fn sel_info_page(&mut self) -> SEL_INFO_PAGE_W<8> {
        SEL_INFO_PAGE_W::new(self)
    }
    #[doc = "Bits 2:3 - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<2> {
        CM_W::new(self)
    }
    #[doc = "Bit 1 - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W<1> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 0 - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W<0> {
        ERASE_W::new(self)
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

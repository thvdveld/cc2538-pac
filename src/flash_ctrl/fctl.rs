#[doc = "Register `FCTL` reader"]
pub type R = crate::R<FCTL_SPEC>;
#[doc = "Register `FCTL` writer"]
pub type W = crate::W<FCTL_SPEC>;
#[doc = "Field `ERASE` reader - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type ERASE_R = crate::BitReader;
#[doc = "Field `ERASE` writer - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type ERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type WRITE_R = crate::BitReader;
#[doc = "Field `WRITE` writer - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM` reader - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
pub type CM_R = crate::FieldReader;
#[doc = "Field `CM` writer - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ABORT` reader - Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
pub type ABORT_R = crate::BitReader;
#[doc = "Field `FULL` reader - Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
pub type FULL_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `SEL_INFO_PAGE` reader - Flash erase or write operation on APB bus must assert this when accessing the information page"]
pub type SEL_INFO_PAGE_R = crate::BitReader;
#[doc = "Field `SEL_INFO_PAGE` writer - Flash erase or write operation on APB bus must assert this when accessing the information page"]
pub type SEL_INFO_PAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPPER_PAGE_ACCESS` reader - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
pub type UPPER_PAGE_ACCESS_R = crate::BitReader;
#[doc = "Field `UPPER_PAGE_ACCESS` writer - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
pub type UPPER_PAGE_ACCESS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    pub fn sel_info_page(&self) -> SEL_INFO_PAGE_R {
        SEL_INFO_PAGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    pub fn upper_page_access(&self) -> UPPER_PAGE_ACCESS_R {
        UPPER_PAGE_ACCESS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<FCTL_SPEC> {
        ERASE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<FCTL_SPEC> {
        WRITE_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<FCTL_SPEC> {
        CM_W::new(self, 2)
    }
    #[doc = "Bit 8 - Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    #[must_use]
    pub fn sel_info_page(&mut self) -> SEL_INFO_PAGE_W<FCTL_SPEC> {
        SEL_INFO_PAGE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    #[must_use]
    pub fn upper_page_access(&mut self) -> UPPER_PAGE_ACCESS_W<FCTL_SPEC> {
        UPPER_PAGE_ACCESS_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash control This register provides control and monitoring functions for the flash module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL_SPEC;
impl crate::RegisterSpec for FCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctl::R`](R) reader structure"]
impl crate::Readable for FCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl::W`](W) writer structure"]
impl crate::Writable for FCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTL to value 0"]
impl crate::Resettable for FCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}

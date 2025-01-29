#[doc = "Register `FCTL` reader"]
pub type R = crate::R<FctlSpec>;
#[doc = "Register `FCTL` writer"]
pub type W = crate::W<FctlSpec>;
#[doc = "Field `ERASE` reader - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM` reader - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
pub type CmR = crate::FieldReader;
#[doc = "Field `CM` writer - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
pub type CmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ABORT` reader - Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
pub type AbortR = crate::BitReader;
#[doc = "Field `FULL` reader - Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
pub type FullR = crate::BitReader;
#[doc = "Field `BUSY` reader - Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `SEL_INFO_PAGE` reader - Flash erase or write operation on APB bus must assert this when accessing the information page"]
pub type SelInfoPageR = crate::BitReader;
#[doc = "Field `SEL_INFO_PAGE` writer - Flash erase or write operation on APB bus must assert this when accessing the information page"]
pub type SelInfoPageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPPER_PAGE_ACCESS` reader - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
pub type UpperPageAccessR = crate::BitReader;
#[doc = "Field `UPPER_PAGE_ACCESS` writer - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
pub type UpperPageAccessW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    pub fn sel_info_page(&self) -> SelInfoPageR {
        SelInfoPageR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    pub fn upper_page_access(&self) -> UpperPageAccessR {
        UpperPageAccessR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<FctlSpec> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<FctlSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<FctlSpec> {
        CmW::new(self, 2)
    }
    #[doc = "Bit 8 - Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    pub fn sel_info_page(&mut self) -> SelInfoPageW<FctlSpec> {
        SelInfoPageW::new(self, 8)
    }
    #[doc = "Bit 9 - Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    pub fn upper_page_access(&mut self) -> UpperPageAccessW<FctlSpec> {
        UpperPageAccessW::new(self, 9)
    }
}
#[doc = "Flash control This register provides control and monitoring functions for the flash module.\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctlSpec;
impl crate::RegisterSpec for FctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctl::R`](R) reader structure"]
impl crate::Readable for FctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctl::W`](W) writer structure"]
impl crate::Writable for FctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTL to value 0"]
impl crate::Resettable for FctlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DMAC_MST_RUNPARAMS` reader"]
pub type R = crate::R<DmacMstRunparamsSpec>;
#[doc = "Register `DMAC_MST_RUNPARAMS` writer"]
pub type W = crate::W<DmacMstRunparamsSpec>;
#[doc = "Field `AHB_MST1_BIGEND` reader - Endianess for the AHB master 0: Little endian 1: Big endian"]
pub type AhbMst1BigendR = crate::BitReader;
#[doc = "Field `AHB_MST1_BIGEND` writer - Endianess for the AHB master 0: Little endian 1: Big endian"]
pub type AhbMst1BigendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_MST1_LOCK_EN` reader - Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
pub type AhbMst1LockEnR = crate::BitReader;
#[doc = "Field `AHB_MST1_LOCK_EN` writer - Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
pub type AhbMst1LockEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_MST1_INCR_EN` reader - Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
pub type AhbMst1IncrEnR = crate::BitReader;
#[doc = "Field `AHB_MST1_INCR_EN` writer - Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
pub type AhbMst1IncrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_MST1_IDLE_EN` reader - Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
pub type AhbMst1IdleEnR = crate::BitReader;
#[doc = "Field `AHB_MST1_IDLE_EN` writer - Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
pub type AhbMst1IdleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_MST1_BURST_SIZE` reader - Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
pub type AhbMst1BurstSizeR = crate::FieldReader;
#[doc = "Field `AHB_MST1_BURST_SIZE` writer - Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
pub type AhbMst1BurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 8 - Endianess for the AHB master 0: Little endian 1: Big endian"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&self) -> AhbMst1BigendR {
        AhbMst1BigendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&self) -> AhbMst1LockEnR {
        AhbMst1LockEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&self) -> AhbMst1IncrEnR {
        AhbMst1IncrEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&self) -> AhbMst1IdleEnR {
        AhbMst1IdleEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&self) -> AhbMst1BurstSizeR {
        AhbMst1BurstSizeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Endianess for the AHB master 0: Little endian 1: Big endian"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_bigend(&mut self) -> AhbMst1BigendW<DmacMstRunparamsSpec> {
        AhbMst1BigendW::new(self, 8)
    }
    #[doc = "Bit 9 - Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_lock_en(&mut self) -> AhbMst1LockEnW<DmacMstRunparamsSpec> {
        AhbMst1LockEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_incr_en(&mut self) -> AhbMst1IncrEnW<DmacMstRunparamsSpec> {
        AhbMst1IncrEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_idle_en(&mut self) -> AhbMst1IdleEnW<DmacMstRunparamsSpec> {
        AhbMst1IdleEnW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_mst1_burst_size(&mut self) -> AhbMst1BurstSizeW<DmacMstRunparamsSpec> {
        AhbMst1BurstSizeW::new(self, 12)
    }
}
#[doc = "DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_mst_runparams::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_mst_runparams::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacMstRunparamsSpec;
impl crate::RegisterSpec for DmacMstRunparamsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_mst_runparams::R`](R) reader structure"]
impl crate::Readable for DmacMstRunparamsSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac_mst_runparams::W`](W) writer structure"]
impl crate::Writable for DmacMstRunparamsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_MST_RUNPARAMS to value 0"]
impl crate::Resettable for DmacMstRunparamsSpec {
    const RESET_VALUE: u32 = 0;
}

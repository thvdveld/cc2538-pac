#[doc = "Register `DMAC_CH1_CTRL` reader"]
pub type R = crate::R<DmacCh1CtrlSpec>;
#[doc = "Register `DMAC_CH1_CTRL` writer"]
pub type W = crate::W<DmacCh1CtrlSpec>;
#[doc = "Field `EN` reader - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIO` reader - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
pub type PrioR = crate::BitReader;
#[doc = "Field `PRIO` writer - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
pub type PrioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<DmacCh1CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
    #[inline(always)]
    pub fn prio(&mut self) -> PrioW<DmacCh1CtrlSpec> {
        PrioW::new(self, 1)
    }
}
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac_ch1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac_ch1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCh1CtrlSpec;
impl crate::RegisterSpec for DmacCh1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_ch1_ctrl::R`](R) reader structure"]
impl crate::Readable for DmacCh1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac_ch1_ctrl::W`](W) writer structure"]
impl crate::Writable for DmacCh1CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_CH1_CTRL to value 0"]
impl crate::Resettable for DmacCh1CtrlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DMAC_CH0_CTRL` reader"]
pub type R = crate::R<DMAC_CH0_CTRL_SPEC>;
#[doc = "Register `DMAC_CH0_CTRL` writer"]
pub type W = crate::W<DMAC_CH0_CTRL_SPEC>;
#[doc = "Field `EN` reader - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRIO` reader - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
pub type PRIO_R = crate::BitReader;
#[doc = "Field `PRIO` writer - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
pub type PRIO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DMAC_CH0_CTRL_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<DMAC_CH0_CTRL_SPEC, 1> {
        PRIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_CH0_CTRL_SPEC;
impl crate::RegisterSpec for DMAC_CH0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_ch0_ctrl::R`](R) reader structure"]
impl crate::Readable for DMAC_CH0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac_ch0_ctrl::W`](W) writer structure"]
impl crate::Writable for DMAC_CH0_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC_CH0_CTRL to value 0"]
impl crate::Resettable for DMAC_CH0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

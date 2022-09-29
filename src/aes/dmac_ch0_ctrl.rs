#[doc = "Register `DMAC_CH0_CTRL` reader"]
pub struct R(crate::R<DMAC_CH0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_CH0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_CH0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_CH0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_CH0_CTRL` writer"]
pub struct W(crate::W<DMAC_CH0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_CH0_CTRL_SPEC>;
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
impl From<crate::W<DMAC_CH0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_CH0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC_CH0_CTRL_SPEC, bool, O>;
#[doc = "Field `PRIO` reader - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
pub type PRIO_R = crate::BitReader<bool>;
#[doc = "Field `PRIO` writer - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
pub type PRIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC_CH0_CTRL_SPEC, bool, O>;
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
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W<1> {
        PRIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_ch0_ctrl](index.html) module"]
pub struct DMAC_CH0_CTRL_SPEC;
impl crate::RegisterSpec for DMAC_CH0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_ch0_ctrl::R](R) reader structure"]
impl crate::Readable for DMAC_CH0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_ch0_ctrl::W](W) writer structure"]
impl crate::Writable for DMAC_CH0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_CH0_CTRL to value 0"]
impl crate::Resettable for DMAC_CH0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

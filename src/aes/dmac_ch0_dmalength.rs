#[doc = "Register `DMAC_CH0_DMALENGTH` reader"]
pub type R = crate::R<DMAC_CH0_DMALENGTH_SPEC>;
#[doc = "Register `DMAC_CH0_DMALENGTH` writer"]
pub type W = crate::W<DMAC_CH0_DMALENGTH_SPEC>;
#[doc = "Field `DMALEN` reader - Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
pub type DMALEN_R = crate::FieldReader<u16>;
#[doc = "Field `DMALEN` writer - Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
pub type DMALEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    pub fn dmalen(&self) -> DMALEN_R {
        DMALEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn dmalen(&mut self) -> DMALEN_W<DMAC_CH0_DMALENGTH_SPEC, 0> {
        DMALEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel DMA length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch0_dmalength::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch0_dmalength::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_CH0_DMALENGTH_SPEC;
impl crate::RegisterSpec for DMAC_CH0_DMALENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_ch0_dmalength::R`](R) reader structure"]
impl crate::Readable for DMAC_CH0_DMALENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac_ch0_dmalength::W`](W) writer structure"]
impl crate::Writable for DMAC_CH0_DMALENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC_CH0_DMALENGTH to value 0"]
impl crate::Resettable for DMAC_CH0_DMALENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

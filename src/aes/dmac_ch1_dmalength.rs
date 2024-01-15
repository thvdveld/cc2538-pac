#[doc = "Register `DMAC_CH1_DMALENGTH` reader"]
pub type R = crate::R<DMAC_CH1_DMALENGTH_SPEC>;
#[doc = "Register `DMAC_CH1_DMALENGTH` writer"]
pub type W = crate::W<DMAC_CH1_DMALENGTH_SPEC>;
#[doc = "Field `DMALEN` reader - Channel DMA length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
pub type DMALEN_R = crate::FieldReader<u16>;
#[doc = "Field `DMALEN` writer - Channel DMA length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
pub type DMALEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel DMA length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    pub fn dmalen(&self) -> DMALEN_R {
        DMALEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel DMA length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn dmalen(&mut self) -> DMALEN_W<DMAC_CH1_DMALENGTH_SPEC> {
        DMALEN_W::new(self, 0)
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
#[doc = "Channel DMA length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch1_dmalength::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch1_dmalength::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_CH1_DMALENGTH_SPEC;
impl crate::RegisterSpec for DMAC_CH1_DMALENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_ch1_dmalength::R`](R) reader structure"]
impl crate::Readable for DMAC_CH1_DMALENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac_ch1_dmalength::W`](W) writer structure"]
impl crate::Writable for DMAC_CH1_DMALENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_CH1_DMALENGTH to value 0"]
impl crate::Resettable for DMAC_CH1_DMALENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CTRL_INT_SET` writer"]
pub type W = crate::W<CTRL_INT_SET_SPEC>;
#[doc = "Field `RESULT_AV` writer - If 1 is written to this bit, the result available (irq_result_av) interrupt output is set to one. Writing 0 has no effect. If the interrupt configuration register is programmed to pulse, clearing the result available (irq_result_av) interrupt is not needed. If it is programmed to level, clearing the interrupt output should be done by writing the interrupt clear register (CTRL_INT_CLR)."]
pub type RESULT_AV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_IN_DONE` writer - If 1 is written to this bit, the DMA data in done (irq_dma_in_done) interrupt output is set to one. Writing 0 has no effect. If the interrupt configuration register is programmed to pulse, clearing the DMA data in done (irq_dma_in_done) interrupt is not needed. If it is programmed to level, clearing the interrupt output should be done by writing the interrupt clear register (CTRL_INT_CLR)."]
pub type DMA_IN_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - If 1 is written to this bit, the result available (irq_result_av) interrupt output is set to one. Writing 0 has no effect. If the interrupt configuration register is programmed to pulse, clearing the result available (irq_result_av) interrupt is not needed. If it is programmed to level, clearing the interrupt output should be done by writing the interrupt clear register (CTRL_INT_CLR)."]
    #[inline(always)]
    #[must_use]
    pub fn result_av(&mut self) -> RESULT_AV_W<CTRL_INT_SET_SPEC, 0> {
        RESULT_AV_W::new(self)
    }
    #[doc = "Bit 1 - If 1 is written to this bit, the DMA data in done (irq_dma_in_done) interrupt output is set to one. Writing 0 has no effect. If the interrupt configuration register is programmed to pulse, clearing the DMA data in done (irq_dma_in_done) interrupt is not needed. If it is programmed to level, clearing the interrupt output should be done by writing the interrupt clear register (CTRL_INT_CLR)."]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W<CTRL_INT_SET_SPEC, 1> {
        DMA_IN_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_int_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_INT_SET_SPEC;
impl crate::RegisterSpec for CTRL_INT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl_int_set::W`](W) writer structure"]
impl crate::Writable for CTRL_INT_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_INT_SET to value 0"]
impl crate::Resettable for CTRL_INT_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

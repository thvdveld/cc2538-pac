#[doc = "Register `CTRL_INT_EN` reader"]
pub type R = crate::R<CTRL_INT_EN_SPEC>;
#[doc = "Register `CTRL_INT_EN` writer"]
pub type W = crate::W<CTRL_INT_EN_SPEC>;
#[doc = "Field `RESULT_AV` reader - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
pub type RESULT_AV_R = crate::BitReader;
#[doc = "Field `RESULT_AV` writer - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
pub type RESULT_AV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_IN_DONE` reader - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
pub type DMA_IN_DONE_R = crate::BitReader;
#[doc = "Field `DMA_IN_DONE` writer - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
pub type DMA_IN_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    pub fn result_av(&self) -> RESULT_AV_R {
        RESULT_AV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn result_av(&mut self) -> RESULT_AV_W<CTRL_INT_EN_SPEC, 0> {
        RESULT_AV_W::new(self)
    }
    #[doc = "Bit 1 - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W<CTRL_INT_EN_SPEC, 1> {
        DMA_IN_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_INT_EN_SPEC;
impl crate::RegisterSpec for CTRL_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_int_en::R`](R) reader structure"]
impl crate::Readable for CTRL_INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_int_en::W`](W) writer structure"]
impl crate::Writable for CTRL_INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_INT_EN to value 0"]
impl crate::Resettable for CTRL_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

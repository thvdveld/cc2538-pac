#[doc = "Register `CTRL_INT_EN` reader"]
pub type R = crate::R<CtrlIntEnSpec>;
#[doc = "Register `CTRL_INT_EN` writer"]
pub type W = crate::W<CtrlIntEnSpec>;
#[doc = "Field `RESULT_AV` reader - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
pub type ResultAvR = crate::BitReader;
#[doc = "Field `RESULT_AV` writer - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
pub type ResultAvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_IN_DONE` reader - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
pub type DmaInDoneR = crate::BitReader;
#[doc = "Field `DMA_IN_DONE` writer - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
pub type DmaInDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    pub fn result_av(&self) -> ResultAvR {
        ResultAvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DmaInDoneR {
        DmaInDoneR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    pub fn result_av(&mut self) -> ResultAvW<CtrlIntEnSpec> {
        ResultAvW::new(self, 0)
    }
    #[doc = "Bit 1 - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    pub fn dma_in_done(&mut self) -> DmaInDoneW<CtrlIntEnSpec> {
        DmaInDoneW::new(self, 1)
    }
}
#[doc = "Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlIntEnSpec;
impl crate::RegisterSpec for CtrlIntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_int_en::R`](R) reader structure"]
impl crate::Readable for CtrlIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_int_en::W`](W) writer structure"]
impl crate::Writable for CtrlIntEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_INT_EN to value 0"]
impl crate::Resettable for CtrlIntEnSpec {
    const RESET_VALUE: u32 = 0;
}

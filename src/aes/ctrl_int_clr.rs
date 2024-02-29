#[doc = "Register `CTRL_INT_CLR` writer"]
pub type W = crate::W<CtrlIntClrSpec>;
#[doc = "Field `RESULT_AV` writer - If 1 is written to this bit, the result available (irq_result_av) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to CTRL_INT_CFG)."]
pub type ResultAvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_IN_DONE` writer - If 1 is written to this bit, the DMA in done (irq_dma_in_done) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to CTRL_INT_CFG)."]
pub type DmaInDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ST_RD_ERR` writer - If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
pub type KeyStRdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ST_WR_ERR` writer - If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
pub type KeyStWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_BUS_ERR` writer - If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
pub type DmaBusErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - If 1 is written to this bit, the result available (irq_result_av) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to CTRL_INT_CFG)."]
    #[inline(always)]
    #[must_use]
    pub fn result_av(&mut self) -> ResultAvW<CtrlIntClrSpec> {
        ResultAvW::new(self, 0)
    }
    #[doc = "Bit 1 - If 1 is written to this bit, the DMA in done (irq_dma_in_done) interrupt output is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to CTRL_INT_CFG)."]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_done(&mut self) -> DmaInDoneW<CtrlIntClrSpec> {
        DmaInDoneW::new(self, 1)
    }
    #[doc = "Bit 29 - If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_rd_err(&mut self) -> KeyStRdErrW<CtrlIntClrSpec> {
        KeyStRdErrW::new(self, 29)
    }
    #[doc = "Bit 30 - If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_wr_err(&mut self) -> KeyStWrErrW<CtrlIntClrSpec> {
        KeyStWrErrW::new(self, 30)
    }
    #[doc = "Bit 31 - If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn dma_bus_err(&mut self) -> DmaBusErrW<CtrlIntClrSpec> {
        DmaBusErrW::new(self, 31)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlIntClrSpec;
impl crate::RegisterSpec for CtrlIntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl_int_clr::W`](W) writer structure"]
impl crate::Writable for CtrlIntClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_INT_CLR to value 0"]
impl crate::Resettable for CtrlIntClrSpec {
    const RESET_VALUE: u32 = 0;
}

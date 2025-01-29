#[doc = "Register `CTRL_INT_STAT` reader"]
pub type R = crate::R<CtrlIntStatSpec>;
#[doc = "Field `RESULT_AV` reader - This read only bit returns the actual result available (irq_result_av) interrupt status of the result available interrupt output pin (irq_result_av)."]
pub type ResultAvR = crate::BitReader;
#[doc = "Field `DMA_IN_DONE` reader - This read only bit returns the actual DMA data in done (irq_data_in_done) interrupt status of the DMA data in done interrupt output pin (irq_data_in_done)."]
pub type DmaInDoneR = crate::BitReader;
#[doc = "Field `KEY_ST_RD_ERR` reader - This bit is set when a read error is detected during the read of a key from the key store, while copying it to the AES core. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a key location is selected in the key store that is not available."]
pub type KeyStRdErrR = crate::BitReader;
#[doc = "Field `KEY_ST_WR_ERR` reader - This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected."]
pub type KeyStWrErrR = crate::BitReader;
#[doc = "Field `DMA_BUS_ERR` reader - This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation."]
pub type DmaBusErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This read only bit returns the actual result available (irq_result_av) interrupt status of the result available interrupt output pin (irq_result_av)."]
    #[inline(always)]
    pub fn result_av(&self) -> ResultAvR {
        ResultAvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This read only bit returns the actual DMA data in done (irq_data_in_done) interrupt status of the DMA data in done interrupt output pin (irq_data_in_done)."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DmaInDoneR {
        DmaInDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit is set when a read error is detected during the read of a key from the key store, while copying it to the AES core. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a key location is selected in the key store that is not available."]
    #[inline(always)]
    pub fn key_st_rd_err(&self) -> KeyStRdErrR {
        KeyStRdErrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected."]
    #[inline(always)]
    pub fn key_st_wr_err(&self) -> KeyStWrErrR {
        KeyStWrErrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation."]
    #[inline(always)]
    pub fn dma_bus_err(&self) -> DmaBusErrR {
        DmaBusErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_int_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlIntStatSpec;
impl crate::RegisterSpec for CtrlIntStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_int_stat::R`](R) reader structure"]
impl crate::Readable for CtrlIntStatSpec {}
#[doc = "`reset()` method sets CTRL_INT_STAT to value 0"]
impl crate::Resettable for CtrlIntStatSpec {
    const RESET_VALUE: u32 = 0;
}

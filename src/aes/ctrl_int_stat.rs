#[doc = "Register `CTRL_INT_STAT` reader"]
pub struct R(crate::R<CTRL_INT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_INT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_INT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_INT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_BUS_ERR` reader - This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation."]
pub struct DMA_BUS_ERR_R(crate::FieldReader<bool, bool>);
impl DMA_BUS_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_BUS_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_BUS_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_ST_WR_ERR` reader - This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected."]
pub struct KEY_ST_WR_ERR_R(crate::FieldReader<bool, bool>);
impl KEY_ST_WR_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEY_ST_WR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_ST_WR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_ST_RD_ERR` reader - This bit is set when a read error is detected during the read of a key from the key store, while copying it to the AES core. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a key location is selected in the key store that is not available."]
pub struct KEY_ST_RD_ERR_R(crate::FieldReader<bool, bool>);
impl KEY_ST_RD_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEY_ST_RD_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_ST_RD_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_DONE` reader - This read only bit returns the actual DMA data in done (irq_data_in_done) interrupt status of the DMA data in done interrupt output pin (irq_data_in_done)."]
pub struct DMA_IN_DONE_R(crate::FieldReader<bool, bool>);
impl DMA_IN_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_IN_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESULT_AV` reader - This read only bit returns the actual result available (irq_result_av) interrupt status of the result available interrupt output pin (irq_result_av)."]
pub struct RESULT_AV_R(crate::FieldReader<bool, bool>);
impl RESULT_AV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESULT_AV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_AV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation."]
    #[inline(always)]
    pub fn dma_bus_err(&self) -> DMA_BUS_ERR_R {
        DMA_BUS_ERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected."]
    #[inline(always)]
    pub fn key_st_wr_err(&self) -> KEY_ST_WR_ERR_R {
        KEY_ST_WR_ERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This bit is set when a read error is detected during the read of a key from the key store, while copying it to the AES core. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a key location is selected in the key store that is not available."]
    #[inline(always)]
    pub fn key_st_rd_err(&self) -> KEY_ST_RD_ERR_R {
        KEY_ST_RD_ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This read only bit returns the actual DMA data in done (irq_data_in_done) interrupt status of the DMA data in done interrupt output pin (irq_data_in_done)."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This read only bit returns the actual result available (irq_result_av) interrupt status of the result available interrupt output pin (irq_result_av)."]
    #[inline(always)]
    pub fn result_av(&self) -> RESULT_AV_R {
        RESULT_AV_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_int_stat](index.html) module"]
pub struct CTRL_INT_STAT_SPEC;
impl crate::RegisterSpec for CTRL_INT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_int_stat::R](R) reader structure"]
impl crate::Readable for CTRL_INT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTRL_INT_STAT to value 0"]
impl crate::Resettable for CTRL_INT_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

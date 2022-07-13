#[doc = "Register `DMAC_PERSR` reader"]
pub struct R(crate::R<DMAC_PERSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_PERSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_PERSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_PERSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PORT1_AHB_ERROR` reader - A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
pub type PORT1_AHB_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `PORT1_CHANNEL` reader - Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
pub type PORT1_CHANNEL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 12 - A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
    #[inline(always)]
    pub fn port1_ahb_error(&self) -> PORT1_AHB_ERROR_R {
        PORT1_AHB_ERROR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
    #[inline(always)]
    pub fn port1_channel(&self) -> PORT1_CHANNEL_R {
        PORT1_CHANNEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_persr](index.html) module"]
pub struct DMAC_PERSR_SPEC;
impl crate::RegisterSpec for DMAC_PERSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_persr::R](R) reader structure"]
impl crate::Readable for DMAC_PERSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAC_PERSR to value 0"]
impl crate::Resettable for DMAC_PERSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `DMAC_STATUS` reader"]
pub type R = crate::R<DMAC_STATUS_SPEC>;
#[doc = "Field `CH0_ACT` reader - A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
pub type CH0_ACT_R = crate::BitReader;
#[doc = "Field `CH1_ACT` reader - A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
pub type CH1_ACT_R = crate::BitReader;
#[doc = "Field `PORT_ERR` reader - Reflects possible transfer errors on the AHB port."]
pub type PORT_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch0_act(&self) -> CH0_ACT_R {
        CH0_ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch1_act(&self) -> CH1_ACT_R {
        CH1_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    pub fn port_err(&self) -> PORT_ERR_R {
        PORT_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_STATUS_SPEC;
impl crate::RegisterSpec for DMAC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_status::R`](R) reader structure"]
impl crate::Readable for DMAC_STATUS_SPEC {}
#[doc = "`reset()` method sets DMAC_STATUS to value 0"]
impl crate::Resettable for DMAC_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}

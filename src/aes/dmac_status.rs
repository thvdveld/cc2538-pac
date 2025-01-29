#[doc = "Register `DMAC_STATUS` reader"]
pub type R = crate::R<DmacStatusSpec>;
#[doc = "Field `CH0_ACT` reader - A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
pub type Ch0ActR = crate::BitReader;
#[doc = "Field `CH1_ACT` reader - A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
pub type Ch1ActR = crate::BitReader;
#[doc = "Field `PORT_ERR` reader - Reflects possible transfer errors on the AHB port."]
pub type PortErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch0_act(&self) -> Ch0ActR {
        Ch0ActR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch1_act(&self) -> Ch1ActR {
        Ch1ActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    pub fn port_err(&self) -> PortErrR {
        PortErrR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacStatusSpec;
impl crate::RegisterSpec for DmacStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_status::R`](R) reader structure"]
impl crate::Readable for DmacStatusSpec {}
#[doc = "`reset()` method sets DMAC_STATUS to value 0"]
impl crate::Resettable for DmacStatusSpec {
    const RESET_VALUE: u32 = 0;
}

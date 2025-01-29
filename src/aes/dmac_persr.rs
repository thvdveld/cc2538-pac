#[doc = "Register `DMAC_PERSR` reader"]
pub type R = crate::R<DmacPersrSpec>;
#[doc = "Field `PORT1_CHANNEL` reader - Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
pub type Port1ChannelR = crate::BitReader;
#[doc = "Field `PORT1_AHB_ERROR` reader - A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
pub type Port1AhbErrorR = crate::BitReader;
impl R {
    #[doc = "Bit 9 - Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
    #[inline(always)]
    pub fn port1_channel(&self) -> Port1ChannelR {
        Port1ChannelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
    #[inline(always)]
    pub fn port1_ahb_error(&self) -> Port1AhbErrorR {
        Port1AhbErrorR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac_persr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacPersrSpec;
impl crate::RegisterSpec for DmacPersrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_persr::R`](R) reader structure"]
impl crate::Readable for DmacPersrSpec {}
#[doc = "`reset()` method sets DMAC_PERSR to value 0"]
impl crate::Resettable for DmacPersrSpec {
    const RESET_VALUE: u32 = 0;
}

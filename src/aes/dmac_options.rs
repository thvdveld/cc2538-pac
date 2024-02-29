#[doc = "Register `DMAC_OPTIONS` reader"]
pub type R = crate::R<DmacOptionsSpec>;
#[doc = "Field `NR_OF_PORTS` reader - Number of ports implemented, value in range 1-4."]
pub type NrOfPortsR = crate::FieldReader;
#[doc = "Field `NR_OF_CHANNELS` reader - Number of channels implemented, value in the range 1-8."]
pub type NrOfChannelsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Number of ports implemented, value in range 1-4."]
    #[inline(always)]
    pub fn nr_of_ports(&self) -> NrOfPortsR {
        NrOfPortsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Number of channels implemented, value in the range 1-8."]
    #[inline(always)]
    pub fn nr_of_channels(&self) -> NrOfChannelsR {
        NrOfChannelsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "DMAC options register These registers contain information regarding the different options configured in this DMAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_options::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacOptionsSpec;
impl crate::RegisterSpec for DmacOptionsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_options::R`](R) reader structure"]
impl crate::Readable for DmacOptionsSpec {}
#[doc = "`reset()` method sets DMAC_OPTIONS to value 0"]
impl crate::Resettable for DmacOptionsSpec {
    const RESET_VALUE: u32 = 0;
}

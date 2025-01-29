#[doc = "Register `PP` reader"]
pub type R = crate::R<PpSpec>;
#[doc = "Field `SC` reader - Smart card support 1: The UART module provides smart card support. 0: The UART module does not provide smart card support."]
pub type ScR = crate::BitReader;
#[doc = "Field `NB` reader - 9-bit support 1: The UART module provides support for the transmission of 9-bit data for RS-485 support. 0: The UART module does not provide support for the transmission of 9-bit data for RS-485 support."]
pub type NbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Smart card support 1: The UART module provides smart card support. 0: The UART module does not provide smart card support."]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 9-bit support 1: The UART module provides support for the transmission of 9-bit data for RS-485 support. 0: The UART module does not provide support for the transmission of 9-bit data for RS-485 support."]
    #[inline(always)]
    pub fn nb(&self) -> NbR {
        NbR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "UART peripheral properties The PP register provides information regarding the properties of the UART module.\n\nYou can [`read`](crate::Reg::read) this register and get [`pp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpSpec;
impl crate::RegisterSpec for PpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp::R`](R) reader structure"]
impl crate::Readable for PpSpec {}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PpSpec {
    const RESET_VALUE: u32 = 0;
}

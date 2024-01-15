#[doc = "Register `RXFIRST` reader"]
pub type R = crate::R<RXFIRST_SPEC>;
#[doc = "Field `DATA` reader - First byte of the RX FIFO Note: Reading this register does not modify the contents of the FIFO."]
pub type DATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - First byte of the RX FIFO Note: Reading this register does not modify the contents of the FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "First byte in RX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfirst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFIRST_SPEC;
impl crate::RegisterSpec for RXFIRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfirst::R`](R) reader structure"]
impl crate::Readable for RXFIRST_SPEC {}
#[doc = "`reset()` method sets RXFIRST to value 0"]
impl crate::Resettable for RXFIRST_SPEC {
    const RESET_VALUE: u32 = 0;
}

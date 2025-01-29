#[doc = "Register `RXFIRST_PTR` reader"]
pub type R = crate::R<RxfirstPtrSpec>;
#[doc = "Field `RXFIRST_PTR` reader - RAM address offset of the first byte in the RX FIFO"]
pub type RxfirstPtrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the first byte in the RX FIFO"]
    #[inline(always)]
    pub fn rxfirst_ptr(&self) -> RxfirstPtrR {
        RxfirstPtrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfirst_ptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfirstPtrSpec;
impl crate::RegisterSpec for RxfirstPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfirst_ptr::R`](R) reader structure"]
impl crate::Readable for RxfirstPtrSpec {}
#[doc = "`reset()` method sets RXFIRST_PTR to value 0"]
impl crate::Resettable for RxfirstPtrSpec {
    const RESET_VALUE: u32 = 0;
}

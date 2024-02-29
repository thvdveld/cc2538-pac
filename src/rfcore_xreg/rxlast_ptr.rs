#[doc = "Register `RXLAST_PTR` reader"]
pub type R = crate::R<RxlastPtrSpec>;
#[doc = "Field `RXLAST_PTR` reader - RAM address offset of the last byte + 1 byte in the RX FIFO"]
pub type RxlastPtrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the last byte + 1 byte in the RX FIFO"]
    #[inline(always)]
    pub fn rxlast_ptr(&self) -> RxlastPtrR {
        RxlastPtrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlast_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxlastPtrSpec;
impl crate::RegisterSpec for RxlastPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlast_ptr::R`](R) reader structure"]
impl crate::Readable for RxlastPtrSpec {}
#[doc = "`reset()` method sets RXLAST_PTR to value 0"]
impl crate::Resettable for RxlastPtrSpec {
    const RESET_VALUE: u32 = 0;
}

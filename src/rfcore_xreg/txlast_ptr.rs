#[doc = "Register `TXLAST_PTR` reader"]
pub type R = crate::R<TxlastPtrSpec>;
#[doc = "Field `TXLAST_PTR` reader - RAM address offset of the last byte + 1 byte of the TX FIFO"]
pub type TxlastPtrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the last byte + 1 byte of the TX FIFO"]
    #[inline(always)]
    pub fn txlast_ptr(&self) -> TxlastPtrR {
        TxlastPtrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlast_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxlastPtrSpec;
impl crate::RegisterSpec for TxlastPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlast_ptr::R`](R) reader structure"]
impl crate::Readable for TxlastPtrSpec {}
#[doc = "`reset()` method sets TXLAST_PTR to value 0"]
impl crate::Resettable for TxlastPtrSpec {
    const RESET_VALUE: u32 = 0;
}

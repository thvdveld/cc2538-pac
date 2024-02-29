#[doc = "Register `TXFIRST_PTR` reader"]
pub type R = crate::R<TxfirstPtrSpec>;
#[doc = "Field `TXFIRST_PTR` reader - RAM address offset of the next byte to be transmitted from the TX FIFO"]
pub type TxfirstPtrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the next byte to be transmitted from the TX FIFO"]
    #[inline(always)]
    pub fn txfirst_ptr(&self) -> TxfirstPtrR {
        TxfirstPtrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfirst_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfirstPtrSpec;
impl crate::RegisterSpec for TxfirstPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfirst_ptr::R`](R) reader structure"]
impl crate::Readable for TxfirstPtrSpec {}
#[doc = "`reset()` method sets TXFIRST_PTR to value 0"]
impl crate::Resettable for TxfirstPtrSpec {
    const RESET_VALUE: u32 = 0;
}

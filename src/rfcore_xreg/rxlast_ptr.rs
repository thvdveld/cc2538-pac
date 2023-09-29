#[doc = "Register `RXLAST_PTR` reader"]
pub type R = crate::R<RXLAST_PTR_SPEC>;
#[doc = "Field `RXLAST_PTR` reader - RAM address offset of the last byte + 1 byte in the RX FIFO"]
pub type RXLAST_PTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the last byte + 1 byte in the RX FIFO"]
    #[inline(always)]
    pub fn rxlast_ptr(&self) -> RXLAST_PTR_R {
        RXLAST_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlast_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXLAST_PTR_SPEC;
impl crate::RegisterSpec for RXLAST_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlast_ptr::R`](R) reader structure"]
impl crate::Readable for RXLAST_PTR_SPEC {}
#[doc = "`reset()` method sets RXLAST_PTR to value 0"]
impl crate::Resettable for RXLAST_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

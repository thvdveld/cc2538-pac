#[doc = "Register `RXFIRST_PTR` reader"]
pub type R = crate::R<RXFIRST_PTR_SPEC>;
#[doc = "Field `RXFIRST_PTR` reader - RAM address offset of the first byte in the RX FIFO"]
pub type RXFIRST_PTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the first byte in the RX FIFO"]
    #[inline(always)]
    pub fn rxfirst_ptr(&self) -> RXFIRST_PTR_R {
        RXFIRST_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfirst_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFIRST_PTR_SPEC;
impl crate::RegisterSpec for RXFIRST_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfirst_ptr::R`](R) reader structure"]
impl crate::Readable for RXFIRST_PTR_SPEC {}
#[doc = "`reset()` method sets RXFIRST_PTR to value 0"]
impl crate::Resettable for RXFIRST_PTR_SPEC {
    const RESET_VALUE: u32 = 0;
}

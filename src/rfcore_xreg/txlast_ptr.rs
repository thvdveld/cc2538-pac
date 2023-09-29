#[doc = "Register `TXLAST_PTR` reader"]
pub type R = crate::R<TXLAST_PTR_SPEC>;
#[doc = "Field `TXLAST_PTR` reader - RAM address offset of the last byte + 1 byte of the TX FIFO"]
pub type TXLAST_PTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the last byte + 1 byte of the TX FIFO"]
    #[inline(always)]
    pub fn txlast_ptr(&self) -> TXLAST_PTR_R {
        TXLAST_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlast_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXLAST_PTR_SPEC;
impl crate::RegisterSpec for TXLAST_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlast_ptr::R`](R) reader structure"]
impl crate::Readable for TXLAST_PTR_SPEC {}
#[doc = "`reset()` method sets TXLAST_PTR to value 0"]
impl crate::Resettable for TXLAST_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

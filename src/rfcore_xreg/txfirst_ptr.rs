#[doc = "Register `TXFIRST_PTR` reader"]
pub type R = crate::R<TXFIRST_PTR_SPEC>;
#[doc = "Field `TXFIRST_PTR` reader - RAM address offset of the next byte to be transmitted from the TX FIFO"]
pub type TXFIRST_PTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the next byte to be transmitted from the TX FIFO"]
    #[inline(always)]
    pub fn txfirst_ptr(&self) -> TXFIRST_PTR_R {
        TXFIRST_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfirst_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFIRST_PTR_SPEC;
impl crate::RegisterSpec for TXFIRST_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfirst_ptr::R`](R) reader structure"]
impl crate::Readable for TXFIRST_PTR_SPEC {}
#[doc = "`reset()` method sets TXFIRST_PTR to value 0"]
impl crate::Resettable for TXFIRST_PTR_SPEC {
    const RESET_VALUE: u32 = 0;
}

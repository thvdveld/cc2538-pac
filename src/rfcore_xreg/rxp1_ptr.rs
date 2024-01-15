#[doc = "Register `RXP1_PTR` reader"]
pub type R = crate::R<RXP1_PTR_SPEC>;
#[doc = "Field `RXP1_PTR` reader - RAM address offset of the first byte of the first frame in the RX FIFO"]
pub type RXP1_PTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the first byte of the first frame in the RX FIFO"]
    #[inline(always)]
    pub fn rxp1_ptr(&self) -> RXP1_PTR_R {
        RXP1_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxp1_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXP1_PTR_SPEC;
impl crate::RegisterSpec for RXP1_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxp1_ptr::R`](R) reader structure"]
impl crate::Readable for RXP1_PTR_SPEC {}
#[doc = "`reset()` method sets RXP1_PTR to value 0"]
impl crate::Resettable for RXP1_PTR_SPEC {
    const RESET_VALUE: u32 = 0;
}

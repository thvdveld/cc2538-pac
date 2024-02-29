#[doc = "Register `RXP1_PTR` reader"]
pub type R = crate::R<Rxp1PtrSpec>;
#[doc = "Field `RXP1_PTR` reader - RAM address offset of the first byte of the first frame in the RX FIFO"]
pub type Rxp1PtrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the first byte of the first frame in the RX FIFO"]
    #[inline(always)]
    pub fn rxp1_ptr(&self) -> Rxp1PtrR {
        Rxp1PtrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxp1_ptr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxp1PtrSpec;
impl crate::RegisterSpec for Rxp1PtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxp1_ptr::R`](R) reader structure"]
impl crate::Readable for Rxp1PtrSpec {}
#[doc = "`reset()` method sets RXP1_PTR to value 0"]
impl crate::Resettable for Rxp1PtrSpec {
    const RESET_VALUE: u32 = 0;
}

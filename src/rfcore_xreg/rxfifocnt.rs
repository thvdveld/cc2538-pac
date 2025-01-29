#[doc = "Register `RXFIFOCNT` reader"]
pub type R = crate::R<RxfifocntSpec>;
#[doc = "Field `RXFIFOCNT` reader - Number of bytes in the RX FIFO (unsigned integer)"]
pub type RxfifocntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of bytes in the RX FIFO (unsigned integer)"]
    #[inline(always)]
    pub fn rxfifocnt(&self) -> RxfifocntR {
        RxfifocntR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes in RX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifocnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifocntSpec;
impl crate::RegisterSpec for RxfifocntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifocnt::R`](R) reader structure"]
impl crate::Readable for RxfifocntSpec {}
#[doc = "`reset()` method sets RXFIFOCNT to value 0"]
impl crate::Resettable for RxfifocntSpec {
    const RESET_VALUE: u32 = 0;
}

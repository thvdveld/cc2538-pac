#[doc = "Register `RXFIFOCNT` reader"]
pub type R = crate::R<RXFIFOCNT_SPEC>;
#[doc = "Field `RXFIFOCNT` reader - Number of bytes in the RX FIFO (unsigned integer)"]
pub type RXFIFOCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of bytes in the RX FIFO (unsigned integer)"]
    #[inline(always)]
    pub fn rxfifocnt(&self) -> RXFIFOCNT_R {
        RXFIFOCNT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes in RX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifocnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFIFOCNT_SPEC;
impl crate::RegisterSpec for RXFIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifocnt::R`](R) reader structure"]
impl crate::Readable for RXFIFOCNT_SPEC {}
#[doc = "`reset()` method sets RXFIFOCNT to value 0"]
impl crate::Resettable for RXFIFOCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

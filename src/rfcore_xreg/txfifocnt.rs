#[doc = "Register `TXFIFOCNT` reader"]
pub type R = crate::R<TXFIFOCNT_SPEC>;
#[doc = "Field `TXFIFOCNT` reader - Number of bytes in the TX FIFO (unsigned integer)"]
pub type TXFIFOCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of bytes in the TX FIFO (unsigned integer)"]
    #[inline(always)]
    pub fn txfifocnt(&self) -> TXFIFOCNT_R {
        TXFIFOCNT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes in TX FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfifocnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFIFOCNT_SPEC;
impl crate::RegisterSpec for TXFIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfifocnt::R`](R) reader structure"]
impl crate::Readable for TXFIFOCNT_SPEC {}
#[doc = "`reset()` method sets TXFIFOCNT to value 0"]
impl crate::Resettable for TXFIFOCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
